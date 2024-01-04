pub mod components;
pub mod resources;

mod bounds;
pub(crate) mod events;
mod systems;

use std::collections::HashMap;

use crate::components::uncover::Uncover;
use crate::components::{Coordinates, PauseCover};
use crate::resources::{BoardPosition, TileSize};
use bevy::log;
use bevy::prelude::*;
use events::TileTriggerEvent;
use resources::tile_map::TileMap;
use resources::{tile::Tile, BoardOptions};

use bevy::math::Vec3Swizzles;
use bounds::Bounds2;
use resources::board::Board;
use resources::BoardAssets;

/// White box
const BACKGROUND_Z: f32 = 0.0;
/// Tiles - boxed above background
const TILE_Z: f32 = 1.0;
/// Count of neighors bombs, bomb, etc.
const TILE_INFO_Z: f32 = 2.0;
/// Box above tile which is still not uncover by player
const TILE_COVER_Z: f32 = 3.0;
/// Pause box
const PAUSE_COVER_Z: f32 = 100.0;

// adopted 0.9 to 0.10, https://bevyengine.org/learn/migration-guides/0.9-0.10/#states
pub struct BoardPlugin<T>
where
    T: States,
{
    pub game_state: T,
    pub pause_state: T,
}

// struct PauseCover {
//     pub(crate) entity: Entity,
// }

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.game_state.clone()), Self::create_board)
            .add_systems(OnExit(self.game_state.clone()), Self::on_exit_log)
            .add_systems(
                Update,
                (
                    systems::input::input_handling,
                    systems::uncover::trigger_event_handler,
                    systems::uncover::uncover_tiles,
                    Self::recreate_board,
                    Self::pause,
                )
                    .run_if(in_state(self.game_state.clone())),
            )
            .add_systems(
                Update,
                (Self::unpause).run_if(in_state(self.pause_state.clone())),
            )
            .add_event::<TileTriggerEvent>();

        log::info!("Loaded Board Plugin");

        #[cfg(feature = "debug")]
        {
            // can't adopt, missing code.
        }
    }
}

impl<T: States> BoardPlugin<T> {
    pub(crate) fn create_board(
        mut commands: Commands,
        board_options: Res<BoardOptions<T>>,
        board_option: Option<Res<Board>>,
        board_assets: Res<BoardAssets>
    ) {
        // if board already exists, do nothing
        if board_option.is_some() {
            return;
        }

        let options = board_options.clone();

        let tile_size = options.tile_size_px();

        let mut tile_map = TileMap::empty(options.map_size.columns, options.map_size.rows);

        // We deduce the size of the complete board
        let board_size = options.board_size();

        log::info!("board_size: {}", board_size);

        // We define the board anchor position (bottom left)
        let board_position = options.board_position_px(BACKGROUND_Z);

        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());

        let mut safe_start: Option<Entity> = None;

        //adopted 0.8 to 0.9
        let board_entity = commands
            .spawn((
                Name::new("Board"),
                // adopted, original source doesn't pass the hierarchy check and gives the warning
                // https://bevyengine.org/learn/errors/#b0004
                SpatialBundle {
                    transform: Transform::from_translation(board_position),
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        // one big white box
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            board_size.x / 2.,
                            board_size.y / 2.,
                            BACKGROUND_Z,
                        ),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,                    
                    &mut covered_tiles,
                    &mut safe_start,
                    board_assets.as_ref()
                );
            })
            .id();

        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        commands.insert_resource(Board {
            tile_map: tile_map.clone(),
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            entity: board_entity,
        });
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
        board_assets: &BoardAssets,
    ) {
        // remove duplicate of logic from original tutorial
        let tile_real_size = tile_size - tile_padding;
        let sprites_size = Some(Vec2::splat(tile_real_size));

        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };

                log::info!("Spawn tile {:?} at {:?}", tile, coordinates);

                let mut commands = parent.spawn(SpriteBundle {                    
                    sprite: Sprite {
                        color: board_assets.tile_material.color,
                        custom_size: sprites_size,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.),
                        (y as f32 * tile_size) + (tile_size / 2.),
                        TILE_Z,
                    ),
                    ..Default::default()
                });

                commands
                    .insert(Name::new(format!("Tile: ({}, {})", x, y)))
                    .insert(coordinates);

                commands.with_children(|parent| {
                    let entity = parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: sprites_size,
                                color: board_assets.covered_tile_material.color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, TILE_COVER_Z),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

                match tile {
                    Tile::Bomb => {
                        commands.insert(components::Bomb);
                        commands.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: sprites_size,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., TILE_INFO_Z),
                                texture: board_assets.bomb_material.texture.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbour(bombs_count) => {
                        commands.insert(components::BombNeighbor {
                            count: *bombs_count,
                        });
                        commands.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *bombs_count,                                
                                tile_real_size,
                                board_assets
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    fn bomb_count_text_bundle(count: u8, font_size: f32, board_assets: &BoardAssets) -> Text2dBundle {
        let color = board_assets.bomb_counter_color(count);

        let style = TextStyle {
            font: board_assets.bomb_counter_font.clone(),
            font_size: font_size,
            color,
        };
        // adopted 0.9 to 0.10 and simplified API
        let text =
            Text::from_section(count.to_string(), style).with_alignment(TextAlignment::Center);

        Text2dBundle {
            text,
            // z-order, print text on top of the tile
            transform: Transform::from_xyz(0.0, 0.0, TILE_INFO_Z),
            ..Default::default()
        }
    }

    fn recreate_board(
        mut commands: Commands,
        keys: Res<Input<KeyCode>>,
        board: Res<Board>,
        board_assets: Res<BoardAssets>,
        board_options: Res<BoardOptions<T>>,
    ) {
        if keys.just_released(KeyCode::G) {
            log::info!("G is released");
            commands.entity(board.entity).despawn_recursive();
            BoardPlugin::create_board(commands, board_options, None, board_assets)
        }
    }

    fn pause(
        mut commands: Commands,
        keys: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState<T>>,
        board_options: Res<BoardOptions<T>>,
        board_assets: Res<BoardAssets>,
    ) {
        if keys.just_released(KeyCode::P) {
            next_state.set(board_options.pause_state.clone());

            let font: Handle<Font> = board_assets.menu_font.clone();
            let text_style = TextStyle {
                font: font,
                font_size: board_options.tile_size_px(),
                color: Color::YELLOW,
            };
            let text = Text::from_section("Paused! Press P to continue", text_style)
                .with_alignment(TextAlignment::Center);


            let board_size = board_options.board_size();
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::SEA_GREEN,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, PAUSE_COVER_Z),
                    ..Default::default()
                })
                .insert(Name::new("Pause cover"))
                .insert(PauseCover)
                .with_children(|parent| {
                    parent.spawn(Text2dBundle{
                        text,
                        transform: Transform::from_xyz(0.0, 0.0, PAUSE_COVER_Z+1.0),
                        ..Default::default()
                    });
        
                });
        }
    }

    fn unpause(
        mut commands: Commands,
        keys: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState<T>>,
        board_options: Res<BoardOptions<T>>,
        pause_cover_query: Query<Entity, With<PauseCover>>,
    ) {
        if keys.just_released(KeyCode::P) {            
            let x: Entity = pause_cover_query.single();
            commands.entity(x).despawn_recursive();
            next_state.set(board_options.game_state.clone())
        }
    }

    fn on_exit_log() {
        log::info!("exit from state")
    }
}

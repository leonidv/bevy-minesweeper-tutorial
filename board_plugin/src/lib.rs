pub mod components;
pub mod resources;

mod bounds;
pub(crate) mod events;
mod systems;

use std::collections::HashMap;

use crate::components::Coordinates;
use crate::components::bomb::Bomb;
use crate::components::bomb_neighbor::BombNeighbor;
use crate::resources::{BoardPosition, TileSize};
use bevy::log;
use bevy::prelude::*;
use events::TileTriggerEvent;
use resources::tile_map::TileMap;
use resources::{tile::Tile, BoardOptions};

use bevy::math::Vec3Swizzles;
use bounds::Bounds2;
use resources::board::Board;

pub struct BoardPlugin;

impl BoardPlugin {
    fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        asset_server: Res<AssetServer>,
    ) {
        // adopted, added Handle
        let font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
        let bomb_image: Handle<Image> = asset_server.load("sprites/bomb.png");
        
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let tile_size = match options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { .. } => panic!(
                "Not supported in this commit due to WindowDescriptor is not available as resource"
            ),
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);

        // We deduce the size of the complete board
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board_size: {}", board_size);

        // We define the board anchor position (bottom left)
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3 {
                    x: -(board_size.x / 2.0),
                    y: -(board_size.y / 2.0),
                    z: 0.0,
                } + offset
            }
            BoardPosition::Custom(p) => p,
        };

        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());

        //adopted 0.8 to 0.9
        commands
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
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    bomb_image,
                    font,
                    Color::DARK_GRAY,
                    &mut covered_tiles,
                );
            });

        commands.insert_resource(Board {
            tile_map: tile_map.clone(),
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
        });
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        background_color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
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
                        color: background_color,
                        custom_size: sprites_size,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.),
                        (y as f32 * tile_size) + (tile_size / 2.),
                        1.,
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
                                color: covered_tile_color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 2.0),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
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
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbour(bombs_count) => {
                        commands.insert(components::BombNeighbor{count: *bombs_count});
                        commands.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *bombs_count,
                                font.clone(),
                                tile_real_size,
                            ))
                            ;
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, font_size: f32) -> Text2dBundle {
        let color = match count {
            1 => Color::WHITE,
            2 => Color::GREEN,
            3 => Color::YELLOW_GREEN,
            4 => Color::YELLOW,
            5 => Color::ORANGE,
            _ => Color::PURPLE,
        };

        let style = TextStyle {
            font,
            font_size,
            color,
        };
        // adopted 0.9 to 0.10 and simplified API
        let text =
            Text::from_section(count.to_string(), style).with_alignment(TextAlignment::Center);

        Text2dBundle {
            text,
            // z-order, print text on top of the tile
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        }
    }
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board)
            .add_systems(Update, systems::input::input_handling)
            .add_systems(Update, systems::uncover::trigger_event_handler)
            .add_systems(Update, systems::uncover::uncover_tiles)
            .add_event::<TileTriggerEvent>();

        log::info!("Loaded Board Plugin");

        #[cfg(feature = "debug")]
        {
            // can't adopt, missing code.
        }
    }
}

use bevy::{prelude::*, log, transform::commands, ecs::query};
use crate::{Board, BoardAssets, events::TileMarkEvent, resources::board::ToggleMarkResult};


pub fn mark_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    board_assests: Res<BoardAssets>,
    mut tile_mark_event_rdr: EventReader<TileMarkEvent>,
    query: Query<&Children>
) {
    for event in tile_mark_event_rdr.read() {
        match board.try_toggle_mark(&event.coordinates) {
            ToggleMarkResult::FlagIsSet(entity) =>{
                commands.entity(entity).with_children(|parent| {
                    parent.spawn(SpriteBundle {
                        texture: board_assests.flag_material.texture.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(board.tile_size)),
                            color: board_assests.flag_material.color,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, crate::TILE_FLAG_Z),
                        ..Default::default()
                    })
                    .insert(Name::new("Flag"));
                });
            },
            ToggleMarkResult::FlagIsUnset(entity) => {
                let children = match query.get(entity) {
                    Ok(value) => value,
                    Err(e) => {
                        log::error!("Failed to retrieve flag entity components: {}", e);
                        continue;
                    },
                };

                for child in children {
                    commands.entity(*child).despawn_recursive();
                }
            },
            ToggleMarkResult::DidNothing => (),
        }
    }
}
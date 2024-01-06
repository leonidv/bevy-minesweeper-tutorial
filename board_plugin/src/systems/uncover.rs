use bevy::prelude::*;
use bevy::log;

use crate::events::BoardCompletedEvent;
use crate::events::BombExplosionEvent;
use crate::{
    components::{bomb::Bomb, bomb_neighbor::BombNeighbor, uncover::Uncover, Coordinates},
    events::TileTriggerEvent,
    resources::board::Board,
};

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {

    // adopted
    for trigger_event in tile_trigger_evr.read() {
        log::info!("Tile trigger event handler {:?}", trigger_event);
        if let Some(entity) = board.tile_to_uncover(&trigger_event.coordinates) {
            log::info!("insert Uncover to {:?}",*entity);
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,    
    mut board_compeleted_event_wr: EventWriter<BoardCompletedEvent>,
    mut board_bomb_explosion_event_wr: EventWriter<BombExplosionEvent>
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn_recursive();
        // adopted parent.0 -> parent.get 
        // https://bevyengine.org/learn/migration-guides/0.7-0.8/#hierarchy-commandization

        let (coordinates, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                log::error!{"{}" ,e};
                continue;
            }
        }; 

        if board.is_completed() {
            log::info!("Board is compeleted 🍾");
            board_compeleted_event_wr.send(BoardCompletedEvent);
        }

        match board.try_uncover_tile(coordinates) {
            None => log::info!("Tried to uncover an already uncovered tile"),
            Some(e) => { 
                //log::info!("Uncovered tile {} (entity: {:?})",coordinates, e) 
            },
        }

        if bomb.is_some() {
            log::info!("Boom 💥!");
            board_bomb_explosion_event_wr.send(BombExplosionEvent);
        }
        // If the tile is empty (no bomb near tile)...
        else if bomb_counter.is_none() {
            // ..We propagate the unconverng by adding the 'Uncover' 
            // which will then be removed next frame
            for entity in board.adjancent_covered_tiles(*coordinates) {
                commands.entity(entity).insert(Uncover);
            };
        }
    }

    
}

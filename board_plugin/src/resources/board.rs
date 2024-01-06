use std::collections::{HashMap, HashSet};

use crate::bounds::Bounds2;
use crate::{Coordinates, TileMap};
use bevy::ecs::system::Resource;
use bevy::math::Vec2;
use bevy::{log, prelude::*};

use bevy::window::Window;

#[cfg_attr(
    feature = "debug",
    derive(bevy_inspector_egui::prelude::InspectorOptions)
)]
#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,

    pub bounds: Bounds2,
    pub tile_size: f32,

    #[reflect(ignore)]
    #[reflect(default = "HashMap::new")]
    pub covered_tiles: HashMap<Coordinates, Entity>,

    #[reflect(ignore)]
    #[reflect(default = "HashSet::new")]
    pub marked_tiles: HashSet<Coordinates>,

    pub entity: Entity,
}

pub(crate) enum ToggleMarkResult {
    FlagIsSet(Entity),
    FlagIsUnset(Entity),
    DidNothing
}

impl Board {
    /// Translates a mouse position to board coordinates (column and row of tile)
    pub(crate) fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        let windows_size = Vec2 {
            x: window.width(),
            y: window.height(),
        };

        let position_at_board = position - (windows_size / 2.0);

        if !self.bounds.in_bounds(position_at_board) {
            return None;
        }

        let coordinates = position_at_board - self.bounds.position;
        return Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            // adopted 0.10 to 0.11, the y of click is inverted realtive to the board
            // https://bevyengine.org/learn/migration-guides/0.10-0.11/#consistent-screen-space-coordinates
            // max row index is height - 1
            y: self.tile_map.height() - 1 - (coordinates.y / self.tile_size) as u16,
        });
    }

    /// Retrivies a covered tile entity
    pub fn tile_to_uncover(&self, coordinates: &Coordinates) -> Option<&Entity> {
        return self.covered_tiles.get(coordinates);
    }

    /// We try to uncover a tile, returning the entity
    pub fn try_uncover_tile(&mut self, coordinates: &Coordinates) -> Option<Entity> {
        return if self.marked_tiles.contains(coordinates) {
            None
        } else {
            self.covered_tiles.remove(coordinates)
        }
    }


    /// Different from tutorial. use custom enum ToggleMarkResult instead of Option<(Entity,Bool>)
    pub(crate) fn try_toggle_mark(
        &mut self,
        coordinates: &Coordinates,
    ) -> ToggleMarkResult {
        // can set flag only on covered tiles
        return match self.covered_tiles.get(coordinates) {
            Some(entity) => {
                if self.marked_tiles.contains(coordinates) {
                    // Different from tutorial. Don't create fn unmark_title
                    self.marked_tiles.remove(coordinates);
                    ToggleMarkResult::FlagIsUnset(entity.clone())
                } else {
                    self.marked_tiles.insert(*coordinates);
                    ToggleMarkResult::FlagIsSet(entity.clone())
                }       
            }
            None => ToggleMarkResult::DidNothing,
        }
    }

    /// We retrieve the adjancent covered tile entities of `coordinates`
    pub fn adjancent_covered_tiles(&self, coordinate: Coordinates) -> Vec<Entity> {
        return self
            .tile_map
            .safe_square_at(coordinate)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect();
    }

    pub fn is_completed(&self) -> bool {
        return self.tile_map.bomb_count() as usize == self.covered_tiles.len();
    }
}

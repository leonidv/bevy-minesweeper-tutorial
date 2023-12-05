use bevy::ecs::event::Event;

use crate::components::Coordinates;

// adopted 0.10 to 0.11 
// https://bevyengine.org/learn/migration-guides/0.10-0.11/#require-derive-event-on-all-events
#[derive(Debug, Clone, Copy, Event)]
pub struct TileTriggerEvent{ 
    pub coordinates: Coordinates
}
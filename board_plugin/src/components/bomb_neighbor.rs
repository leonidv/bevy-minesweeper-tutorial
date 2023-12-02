use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::prelude::InspectorOptions))]
#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct BombNeighbor {
    /// Number of neighbor tiles with bomb
    pub count : u8
}

use bevy::prelude::{Component, Reflect};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::prelude::InspectorOptions))]
#[cfg_attr(feature = "debug", derive(Reflect))]

#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct Bomb;
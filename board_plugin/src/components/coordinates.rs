use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};
use bevy::prelude::Component;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{prelude::*, reflect_inspector};

// adopted https://github.com/jakobhellermann/bevy-inspector-egui/blob/main/docs/MIGRATION_GUIDE_0.15_0.16.md
#[cfg_attr(feature = "debug", derive(InspectorOptions))]
#[derive(Clone, Copy, Component)] // lv - add derives on demand
// todo
pub struct Coordinates {
    pub x: u16,
    pub y: u16
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<(i8,i8)> for Coordinates {
    type Output = Self;

    fn add(self, (rhs_x, rhs_y): (i8,i8)) -> Self::Output {
        let x = ((self.x as i16) + rhs_x as i16) as u16;
        let y = ((self.y as i16) + rhs_y as i16) as u16;
        Self {x, y}
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y)
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
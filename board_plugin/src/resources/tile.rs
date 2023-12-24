use std::fmt::format;

use bevy::ecs::component::Component;
#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Bomb,
    BombNeighbour(u8),
    Empty,
}

impl Tile {
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }

    pub fn console_output(&self) -> String {        
        #[cfg(feature = "debug")]
        return format!(
            "{}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbour(bombs_count) => match bombs_count {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => bombs_count.to_string().red()
                },
                Tile::Empty => " ".black(),
            }
        );

        #[cfg(not(feature = "debug"))]
        return "".to_string();
    }
}

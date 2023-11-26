use crate::components::Coordinates;
use crate::resources::tile::Tile;

use std::ops::{Deref, DerefMut};

use rand::{thread_rng, Rng};

// Delta coordinates for all 8 square neighbors
// [column, row]
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1), // Bottom left
    (0, -1),  // Bottom
    (1, -1),  // Bottom right
    (-1, 0),  // Left
    (1, 0),   // Right
    (-1, 1),  // Top Left
    (0, 1),   // Top
    (1, 1),   // Top Right
];

#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        Self {
            bomb_count: 9,
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map: ({},{}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );

        let table_separator: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, table_separator);

        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output())
            }
            buffer = format!("{}|\n", buffer);
        }

        format!("{}{}", buffer, table_separator)
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        }

        self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }

        let res = self
            .safe_square_at(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count();

        res as u8
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = thread_rng();

        while remaining_bombs > 0 {
            let row = rng.gen_range(0..self.height) as usize;
            let column = rng.gen_range(0..self.width) as usize;
            if let Tile::Empty = self[row][column] {
                self[row][column] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }

        for row in 0..self.height {
            for col in 0..self.width {
                let coords = Coordinates { y: row, x: col };

                if self.is_bomb_at(coords) {
                    continue;
                };

                let bomb_count = self.bomb_count_at(coords);
                if bomb_count == 0 {
                    continue;
                }

                let tile = &mut self[row as usize][col as usize];
                *tile = Tile::BombNeighbour(bomb_count);
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl SpriteMaterial {
    pub fn color(color: Color) -> SpriteMaterial {
        SpriteMaterial {
            color,
            ..Default::default()
        }
    }

    pub fn texture(texture: &Handle<Image>) -> SpriteMaterial {
        SpriteMaterial {
            texture: texture.clone(),
            ..Default::default()
        }
    }
}

#[derive(Resource)]
pub struct BoardAssets {
    pub label: String,

    pub board_material: SpriteMaterial,

    pub tile_material: SpriteMaterial,

    pub covered_tile_material: SpriteMaterial,

    pub bomb_counter_font: Handle<Font>,

    pub bomb_counter_colors: Vec<Color>,

    pub flag_material: SpriteMaterial,

    pub bomb_material: SpriteMaterial,

    pub menu_font: Handle<Font>,
}

impl BoardAssets {
    pub fn default_colors() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::GREEN,
            Color::YELLOW,
            Color::ORANGE,
            Color::PURPLE,
        ]
    }

    pub fn bomb_counter_color(&self, counter: u8) -> Color {
        let color_idx = counter.saturating_sub(1) as usize;
        match self.bomb_counter_colors.get(color_idx) {
            Some(color) => *color,
            None => match self.bomb_counter_colors.last() {
                Some(color) => *color, // after some counts of bomb will used the last color
                None => Color::WHITE,  // empty list of colors
            },
        }
    }
}

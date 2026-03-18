use macroquad::prelude::*;

pub struct GameSettings {
    pub window_size: Vec2,
}

impl GameSettings {}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_size: Vec2 {
                x: 1000.0,
                y: 1000.0,
            },
        }
    }
}

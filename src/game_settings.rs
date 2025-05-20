use macroquad::prelude::*;

pub struct GameSettings {
    pub window_size: Vec2,
}

impl GameSettings {
    pub fn new(window_size: Vec2) -> Self {
        Self { window_size }
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_size: Vec2 { x: 1000.0, y: 1000.0 },
        }
    }
}
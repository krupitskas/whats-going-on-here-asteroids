use std::collections::HashMap;

use macroquad::{color::WHITE, math::Vec2, shapes::draw_circle};

use crate::texture_manager::{Sprite, SpriteId};

pub struct Enemy {
    pub position: Vec2,
    pub direction: Vec2,
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {
            position: Vec2::default(),
            direction: Vec2::default(),
        }
    }

    pub fn render(&self, delta_time: f32, sprites: &mut HashMap<SpriteId, Sprite>) {
        sprites
            .get_mut(&SpriteId::Enemy)
            .unwrap()
            .draw_animated(delta_time, self.position, 0.0, 40.0);
        // draw_circle(
        //     self.position.x,
        //     self.position.y,
        //     40.0,
        //     WHITE,
        // );
    }
}

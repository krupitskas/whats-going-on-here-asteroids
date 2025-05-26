use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::texture_manager::{Sprite, SpriteId};

pub struct Enemy {
    pub position: Vec2,
}

impl Enemy {
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

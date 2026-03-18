use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::texture_manager::{Sprite, SpriteId};

pub struct Enemy {
    pub position: Vec2,
    pub frame_index: u32,
    pub frame_time: f32,
}

impl Enemy {
    pub fn new(position: Vec2) -> Enemy {
        Enemy {
            position,
            frame_index: 0,
            frame_time: 0.0,
        }
    }

    pub fn render(&mut self, delta_time: f32, sprites: &mut HashMap<SpriteId, Sprite>) {
        let sprite = sprites.get_mut(&SpriteId::Enemy).unwrap();

        if sprite.animation_count > 0 && sprite.fps > 0.0 {
            let frame_duration = 1.0 / sprite.fps;
            self.frame_time += delta_time;

            while self.frame_time >= frame_duration {
                self.frame_time -= frame_duration;
                self.frame_index = (self.frame_index + 1) % sprite.animation_count;
            }
        }

        sprite.texture_index = self.frame_index;
        sprite.draw(self.position, 0.0, 40.0);

        // draw_circle(
        //     self.position.x,
        //     self.position.y,
        //     40.0,
        //     WHITE,
        // );
    }
}

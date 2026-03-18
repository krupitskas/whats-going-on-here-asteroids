use macroquad::prelude::Vec2;

use crate::{
    animation_state::AnimationState,
    texture_manager::{Sprite, SpriteId, TextureManager},
};

pub struct Sparkle {
    pub position: Vec2,
    pub size: f32,
    pub animation: AnimationState,
}

impl Sparkle {
    pub fn new(position: Vec2, size: f32) -> Sparkle {
        Sparkle {
            position,
            size,
            animation: AnimationState::default(),
        }
    }

    pub fn update(&mut self, delta_time: f32, sprite: &Sprite) -> bool {
        self.animation
            .advance(delta_time, sprite.fps, sprite.animation_count, false);
        self.animation.finished
    }

    pub fn render(&self, texture_manager: &mut TextureManager) {
        texture_manager
            .textures
            .get(&SpriteId::SparkleVFX)
            .unwrap()
            .draw_frame(self.animation.frame_index, self.position, 0.0, self.size);
    }
}

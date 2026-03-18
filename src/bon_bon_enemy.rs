use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    animation_state::AnimationState,
    constants::{BON_BON_ENEMY_POINTS, BON_BON_ENEMY_RADIUS, BON_BON_ENEMY_SPEED},
    texture_manager::{Sprite, SpriteId},
};

pub struct BonBonEnemy {
    pub position: Vec2,
    pub animation: AnimationState,
    pub alive: bool,
}

impl BonBonEnemy {
    pub fn spawn_at(position: Vec2) -> BonBonEnemy {
        BonBonEnemy {
            position,
            animation: AnimationState::default(),
            alive: true,
        }
    }

    pub fn update(&mut self, delta_time: f32, player_position: Vec2) {
        if !self.alive {
            return;
        }

        let direction = (player_position - self.position).normalize_or_zero();
        self.position += direction * BON_BON_ENEMY_SPEED * delta_time;
    }

    pub fn radius(&self) -> f32 {
        BON_BON_ENEMY_RADIUS
    }

    pub fn points(&self) -> u32 {
        BON_BON_ENEMY_POINTS
    }

    pub fn render(&mut self, delta_time: f32, sprites: &mut HashMap<SpriteId, Sprite>) {
        if !self.alive {
            return;
        }

        let sprite = sprites.get_mut(&SpriteId::BonBonEnemy).unwrap();

        self.animation
            .advance(delta_time, sprite.fps, sprite.animation_count, true);
        sprite.draw_frame(
            self.animation.frame_index,
            self.position,
            0.0,
            self.radius() * 2.0,
        );
    }
}

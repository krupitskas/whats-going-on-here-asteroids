use std::collections::HashMap;

use macroquad::{
    prelude::*,
    rand::gen_range,
    window::{screen_height, screen_width},
};

use crate::{
    animation_state::AnimationState,
    bullet::Bullet,
    constants::{ENEMY_POINTS, ENEMY_RADIUS, ENEMY_SPEED, ENEMY_TARGET_REACHED_DISTANCE},
    enemy_vision::EnemyVision,
    ship::Ship,
    texture_manager::{Sprite, SpriteId},
};

pub struct Enemy {
    pub position: Vec2,
    pub target_position: Vec2,
    pub animation: AnimationState,
    pub vision: EnemyVision,
    pub alive: bool,
}

impl Enemy {
    pub fn spawn_at(position: Vec2) -> Enemy {
        Enemy {
            target_position: random_screen_point(),
            position,
            animation: AnimationState::default(),
            vision: EnemyVision::new(24, 240.0),
            alive: true,
        }
    }

    pub fn update(&mut self, delta_time: f32, player: &Ship) {
        if !self.alive {
            return;
        }

        let to_target = self.target_position - self.position;

        if to_target.length() <= ENEMY_TARGET_REACHED_DISTANCE {
            self.target_position = random_screen_point();
        } else {
            let step = to_target.normalize_or_zero() * ENEMY_SPEED * delta_time;

            if step.length_squared() >= to_target.length_squared() {
                self.position = self.target_position;
                self.target_position = random_screen_point();
            } else {
                self.position += step;
            }
        }

        self.vision.scan(
            self.position,
            player.position,
            crate::constants::SHIP_SIZE / 2.0,
        );
    }

    pub fn colliding_ship(&self, ship: &Ship) -> bool {
        self.alive
            && self.position.distance(ship.position)
                < ENEMY_RADIUS + crate::constants::SHIP_SIZE / 2.0
    }

    pub fn colliding_bullet(&self, bullet: &Bullet) -> bool {
        self.alive
            && self.position.distance(bullet.position)
                < ENEMY_RADIUS + crate::constants::BULLET_SIZE / 2.0
    }

    pub fn points(&self) -> u32 {
        ENEMY_POINTS
    }

    pub fn render(&mut self, delta_time: f32, sprites: &mut HashMap<SpriteId, Sprite>) {
        if !self.alive {
            return;
        }

        self.vision.render(self.position);

        let sprite = sprites.get_mut(&SpriteId::Enemy).unwrap();

        self.animation
            .advance(delta_time, sprite.fps, sprite.animation_count, true);
        sprite.draw_frame(
            self.animation.frame_index,
            self.position,
            0.0,
            ENEMY_RADIUS * 2.0,
        );
    }
}

fn random_screen_point() -> Vec2 {
    Vec2::new(
        gen_range(40.0, screen_width() - 40.0),
        gen_range(40.0, screen_height() - 40.0),
    )
}

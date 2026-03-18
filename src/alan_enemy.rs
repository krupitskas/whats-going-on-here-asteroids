use std::collections::HashMap;

use macroquad::{
    prelude::*,
    rand::gen_range,
    window::{screen_height, screen_width},
};

use crate::{
    animation_state::AnimationState,
    constants::{
        ALAN_ENEMY_HIT_POINTS, ALAN_ENEMY_POINTS, ALAN_ENEMY_RADIUS, ALAN_ENEMY_SPEED,
        ALAN_PROJECTILE_RELOAD_TIME, ALAN_TARGET_REACHED_DISTANCE,
    },
    enemy_projectile::EnemyProjectile,
    enemy_vision::{EnemyVision, VisionTarget},
    texture_manager::{Sprite, SpriteId},
};

pub struct AlanEnemy {
    pub position: Vec2,
    pub target_position: Vec2,
    pub animation: AnimationState,
    pub vision: EnemyVision,
    pub hit_points: u8,
    pub last_shot_time: f64,
    pub alive: bool,
}

impl AlanEnemy {
    pub fn spawn_at(position: Vec2) -> AlanEnemy {
        AlanEnemy {
            target_position: random_screen_point(),
            position,
            animation: AnimationState::default(),
            vision: EnemyVision::new(24, 240.0),
            hit_points: ALAN_ENEMY_HIT_POINTS,
            last_shot_time: 0.0,
            alive: true,
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        vision_targets: &[VisionTarget],
    ) -> Option<EnemyProjectile> {
        if !self.alive {
            return None;
        }

        let to_target = self.target_position - self.position;

        if to_target.length() <= ALAN_TARGET_REACHED_DISTANCE {
            self.target_position = random_screen_point();
        } else {
            let step = to_target.normalize_or_zero() * ALAN_ENEMY_SPEED * delta_time;

            if step.length_squared() >= to_target.length_squared() {
                self.position = self.target_position;
                self.target_position = random_screen_point();
            } else {
                self.position += step;
            }
        }

        let Some(target_position) = self.vision.scan(self.position, vision_targets) else {
            return None;
        };

        if (get_time() - self.last_shot_time) <= ALAN_PROJECTILE_RELOAD_TIME {
            return None;
        }

        self.last_shot_time = get_time();
        Some(EnemyProjectile::new(
            self.position,
            target_position - self.position,
        ))
    }

    pub fn damage(&mut self) -> bool {
        if !self.alive {
            return false;
        }

        if self.hit_points > 1 {
            self.hit_points -= 1;
            false
        } else {
            self.hit_points = 0;
            self.alive = false;
            true
        }
    }

    pub fn radius(&self) -> f32 {
        ALAN_ENEMY_RADIUS
    }

    pub fn points(&self) -> u32 {
        ALAN_ENEMY_POINTS
    }

    pub fn render(&mut self, delta_time: f32, sprites: &mut HashMap<SpriteId, Sprite>) {
        if !self.alive {
            return;
        }

        self.vision.render(self.position);

        let sprite = sprites.get_mut(&SpriteId::AlanEnemy).unwrap();

        self.animation
            .advance(delta_time, sprite.fps, sprite.animation_count, true);

        let damage_level = (ALAN_ENEMY_HIT_POINTS - self.hit_points) as f32;
        let wobble = (get_time() as f32 * 12.0).sin() * 0.08 * damage_level;
        let tint = Color::new(
            1.0,
            1.0 - damage_level * 0.18,
            1.0 - damage_level * 0.22,
            1.0,
        );

        sprite.draw_frame_scaled(
            self.animation.frame_index,
            self.position,
            wobble * 0.6,
            self.radius() * 2.0,
            Vec2::new(1.0 + wobble + damage_level * 0.08, 1.0 - wobble * 0.7),
            tint,
        );
    }
}

fn random_screen_point() -> Vec2 {
    Vec2::new(
        gen_range(40.0, screen_width() - 40.0),
        gen_range(40.0, screen_height() - 40.0),
    )
}

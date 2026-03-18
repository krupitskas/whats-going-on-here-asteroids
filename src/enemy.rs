use std::collections::HashMap;

use macroquad::{prelude::Vec2, rand::gen_range};

use crate::{
    alan_enemy::AlanEnemy,
    bon_bon_enemy::BonBonEnemy,
    bullet::Bullet,
    constants,
    enemy_projectile::EnemyProjectile,
    enemy_vision::VisionTarget,
    math::circles_overlap,
    texture_manager::{Sprite, SpriteId},
};

pub enum Enemy {
    Alan(AlanEnemy),
    BonBon(BonBonEnemy),
}

impl Enemy {
    pub fn spawn_random(position: Vec2) -> Enemy {
        if gen_range(0, 2) == 0 {
            Enemy::Alan(AlanEnemy::spawn_at(position))
        } else {
            Enemy::BonBon(BonBonEnemy::spawn_at(position))
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        player_position: Vec2,
        vision_targets: &[VisionTarget],
    ) -> Option<EnemyProjectile> {
        match self {
            Enemy::Alan(enemy) => enemy.update(delta_time, vision_targets),
            Enemy::BonBon(enemy) => {
                enemy.update(delta_time, player_position);
                None
            }
        }
    }

    pub fn render(&mut self, delta_time: f32, sprites: &mut HashMap<SpriteId, Sprite>) {
        match self {
            Enemy::Alan(enemy) => enemy.render(delta_time, sprites),
            Enemy::BonBon(enemy) => enemy.render(delta_time, sprites),
        }
    }

    pub fn position(&self) -> Vec2 {
        match self {
            Enemy::Alan(enemy) => enemy.position,
            Enemy::BonBon(enemy) => enemy.position,
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            Enemy::Alan(enemy) => enemy.radius(),
            Enemy::BonBon(enemy) => enemy.radius(),
        }
    }

    pub fn alive(&self) -> bool {
        match self {
            Enemy::Alan(enemy) => enemy.alive,
            Enemy::BonBon(enemy) => enemy.alive,
        }
    }

    pub fn damage(&mut self) -> bool {
        match self {
            Enemy::Alan(enemy) => enemy.damage(),
            Enemy::BonBon(enemy) => {
                if !enemy.alive {
                    return false;
                }

                enemy.alive = false;
                true
            }
        }
    }

    pub fn is_bon_bon(&self) -> bool {
        matches!(self, Enemy::BonBon(_))
    }

    pub fn points(&self) -> u32 {
        match self {
            Enemy::Alan(enemy) => enemy.points(),
            Enemy::BonBon(enemy) => enemy.points(),
        }
    }

    pub fn explosion_size(&self) -> f32 {
        match self {
            Enemy::Alan(_) => constants::ALAN_ENEMY_EXPLOSION_SIZE,
            Enemy::BonBon(_) => constants::BON_BON_ENEMY_EXPLOSION_SIZE,
        }
    }

    pub fn colliding_ship_position(&self, ship_position: Vec2) -> bool {
        self.alive()
            && circles_overlap(
                self.position(),
                self.radius(),
                ship_position,
                constants::SHIP_SIZE / 2.0,
            )
    }

    pub fn colliding_bullet(&self, bullet: &Bullet) -> bool {
        self.alive()
            && circles_overlap(
                self.position(),
                self.radius(),
                bullet.position,
                constants::BULLET_SIZE / 2.0,
            )
    }

    pub fn colliding_enemy(&self, other: &Enemy) -> bool {
        self.alive()
            && other.alive()
            && circles_overlap(
                self.position(),
                self.radius(),
                other.position(),
                other.radius(),
            )
    }
}

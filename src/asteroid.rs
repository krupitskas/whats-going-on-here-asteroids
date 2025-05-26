use macroquad::prelude::*;

use crate::{bullet::Bullet, constants::BULLET_SIZE, constants::SHIP_SIZE, ship::Ship};

#[derive(PartialEq, Clone, Copy)]
pub enum AsteroidType {
    Small,
    Medium,
    Big,
}

impl AsteroidType {
    pub fn size(self) -> f32 {
        match self {
            AsteroidType::Small => 10.0,
            AsteroidType::Medium => 20.0,
            AsteroidType::Big => 30.0,
        }
    }

    pub fn speed(self) -> f32 {
        match self {
            AsteroidType::Small => 200.0,
            AsteroidType::Medium => 150.0,
            AsteroidType::Big => 100.0,
        }
    }

    pub fn rotation_speed(self) -> f32 {
        match self {
            AsteroidType::Small => 2.0,
            AsteroidType::Medium => 1.0,
            AsteroidType::Big => 0.5,
        }
    }

    pub fn points(self) -> u32 {
        match self {
            AsteroidType::Small => 300,
            AsteroidType::Medium => 200,
            AsteroidType::Big => 100,
        }
    }
}

pub struct Asteroid {
    pub active_type: AsteroidType,
    pub position: Vec2,
    pub rotation: f32,
    pub direction: Vec2,
    pub alive: bool,
}

impl Asteroid {
    pub fn new(position: Vec2, direction: Vec2) -> Asteroid {
        Asteroid {
            active_type: AsteroidType::Big,
            position,
            rotation: 0.0,
            direction,
            alive: true,
        }
    }

    pub fn colliding_ship(&self, ship: &Ship) -> bool {
        ship.position.distance(self.position) < self.active_type.size() + SHIP_SIZE / 2.0
    }

    pub fn colliding_bullet(&self, bullet: &Bullet) -> bool {
        bullet.position.distance(self.position) < self.active_type.size() + BULLET_SIZE / 2.0
    }
}

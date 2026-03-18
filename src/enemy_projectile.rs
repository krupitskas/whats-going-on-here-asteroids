use macroquad::prelude::*;

pub struct EnemyProjectile {
    pub position: Vec2,
    pub direction: Vec2,
    pub alive: bool,
    pub time_passed: f32,
}

impl EnemyProjectile {
    pub fn new(position: Vec2, direction: Vec2) -> EnemyProjectile {
        EnemyProjectile {
            position,
            direction: direction.normalize_or_zero(),
            alive: true,
            time_passed: 0.0,
        }
    }
}

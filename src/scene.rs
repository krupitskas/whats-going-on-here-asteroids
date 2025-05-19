use std::cmp::max;

use macroquad::prelude::*;

use crate::{asteroid::Asteroid, bullet::Bullet, ship::Ship};

pub struct Scene {
    pub player: Ship,
    pub asteroids: Vec<Asteroid>,
    pub new_asteroids: Vec<Asteroid>,
    pub bullets: Vec<Bullet>,
    pub last_time_asteroid_spawned: f64,
    pub player_score: u32,
    pub best_player_score: u32,
    pub player_died_times: u32,
}

impl Scene {
    pub fn new(screen_size: Vec2) -> Scene {
        Scene {
            player: Ship::new(screen_size),
            asteroids: Vec::new(),
            new_asteroids: Vec::new(),
            bullets: Vec::new(),
            last_time_asteroid_spawned: 0.0,
            player_score: 0,
            best_player_score: 0,
            player_died_times: 0,
        }
    }

    pub fn new_level(&mut self, screen_size: Vec2) {
        self.best_player_score = max(self.best_player_score, self.player_score);
        self.player_score = 0;
        self.player = Ship::new(screen_size);
        self.asteroids.clear();
        self.bullets.clear();
        self.last_time_asteroid_spawned = 0.0;
        self.player_died_times += 1;
    }
}

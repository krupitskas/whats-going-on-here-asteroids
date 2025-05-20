use std::cmp::max;

use macroquad::prelude::*;

use crate::{
    asteroid::{Asteroid, AsteroidType},
    bullet::Bullet,
    constants,
    ship::Ship,
    texture_manager::{self, TextureId, TextureManager},
};

#[derive(PartialEq)]
pub enum SceneState {
    MainMenu,
    InGame,
    Lost,
}

pub struct Scene {
    pub player: Ship,
    pub asteroids: Vec<Asteroid>,
    pub new_asteroids: Vec<Asteroid>,
    pub bullets: Vec<Bullet>,
    pub last_time_asteroid_spawned: f64,
    pub player_score: u32,
    pub best_player_score: u32,
    pub player_died_times: u32,
    pub current_state: SceneState,
    pub texture_manager: TextureManager,
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
            current_state: SceneState::InGame,
            texture_manager: TextureManager::new(),
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

    pub fn try_spawn_asteroid(&mut self) {
        if (get_time() - self.last_time_asteroid_spawned) > constants::ASTEROID_SPAWN_SEC {
            self.last_time_asteroid_spawned = get_time();

            let x = rand::gen_range(0.0, 800.0);
            let y = rand::gen_range(0.0, 800.0);

            let direction = (Vec2 { x: 400.0, y: 400.0 } - Vec2 { x, y }).normalize();

            let new_asteroid = Asteroid::new(Vec2 { x, y }, direction);

            self.asteroids.push(new_asteroid);
        }
    }

    pub fn check_asteroids_vs_player(&mut self, delta_time: f32) {
        for asteroid in self.asteroids.iter_mut() {
            if asteroid.alive {
                asteroid.position += asteroid.direction * delta_time * asteroid.active_type.speed();

                if asteroid.colliding_ship(&self.player) {
                    self.current_state = SceneState::Lost;
                    continue;
                }
            }
        }
    }

    pub fn check_bullets_vs_asteroids(&mut self, delta_time: f32) {
        for bullet in self.bullets.iter_mut() {
            if bullet.alive {
                bullet.position += bullet.direction * delta_time * constants::BULLET_SPEED;

                for asteroid in self.asteroids.iter_mut() {
                    if asteroid.alive && asteroid.colliding_bullet(&bullet) {
                        bullet.alive = false;
                        asteroid.alive = false;
                        self.player_score += asteroid.active_type.points();

                        if asteroid.active_type != AsteroidType::Small {
                            let new_type = if asteroid.active_type == AsteroidType::Big {
                                AsteroidType::Medium
                            } else if asteroid.active_type == AsteroidType::Medium {
                                AsteroidType::Small
                            } else {
                                unimplemented!();
                            };

                            {
                                let x = rand::gen_range(0.0, 800.0);
                                let y = rand::gen_range(0.0, 800.0);

                                let direction = (asteroid.position - Vec2 { x, y }).normalize();

                                self.new_asteroids.push(Asteroid {
                                    active_type: new_type,
                                    position: asteroid.position,
                                    direction: direction,
                                    alive: true,
                                });
                            }
                            {
                                let x = rand::gen_range(0.0, 800.0);
                                let y = rand::gen_range(0.0, 800.0);

                                let direction = (asteroid.position - Vec2 { x, y }).normalize();

                                self.new_asteroids.push(Asteroid {
                                    active_type: new_type,
                                    position: asteroid.position,
                                    direction: direction,
                                    alive: true,
                                });
                            }
                            // asteroids.
                        }
                    }
                }
            }
        }

        // Append new possible asteroids
        self.asteroids.append(&mut self.new_asteroids);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.player.update_inputs(&mut self.bullets);
        self.try_spawn_asteroid();
        self.check_asteroids_vs_player(delta_time);
        self.check_bullets_vs_asteroids(delta_time);
    }

    pub fn render_ship(&self) {
        let ship_draw_params = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: constants::SHIP_SIZE,
                y: constants::SHIP_SIZE,
            }),
            source: Some(Rect {
                x: 16.0,
                y: 0.0,
                w: 16.0,
                h: 16.0,
            }),
            rotation: 0.,
            pivot: None,
            flip_x: false,
            flip_y: false,
        };
        let ship_texture = &self
            .texture_manager
            .textures
            .get(&TextureId::Player)
            .unwrap();
        draw_texture_ex(
            ship_texture,
            self.player.position.x - 8.0,
            self.player.position.y - 8.0,
            WHITE,
            ship_draw_params,
        );
        // draw_circle(self.player.position.x, self.player.position.y, constants::SHIP_SIZE, RED);
        draw_line(
            self.player.position.x,
            self.player.position.y,
            self.player.position.x + self.player.direction.x * 100.0,
            self.player.position.y + self.player.direction.y * 100.0,
            1.0,
            YELLOW,
        );

        // debug
        draw_circle(mouse_position().0, mouse_position().1, 5.0, PURPLE);
    }

    pub fn render(&self, delta_time: f32) {
        clear_background(DARKBLUE);

        self.render_ship();

        for bullet in self.bullets.iter() {
            if bullet.alive {
                draw_circle(
                    bullet.position.x,
                    bullet.position.y,
                    constants::BULLET_SIZE,
                    YELLOW,
                );
            }
        }

        for asteroid in self.asteroids.iter() {
            if asteroid.alive {
                draw_circle(
                    asteroid.position.x,
                    asteroid.position.y,
                    asteroid.active_type.size(),
                    BROWN,
                );
            }
        }

        draw_text(
            std::format!("ms: {}", delta_time).as_str(),
            20.0,
            20.0,
            20.0,
            DARKGRAY,
        );

        draw_text(
            std::format!("SCORE: {}", self.player_score).as_str(),
            20.0,
            40.0,
            20.0,
            YELLOW,
        );
        draw_text(
            std::format!("BEST SCORE: {}", self.best_player_score).as_str(),
            20.0,
            60.0,
            20.0,
            YELLOW,
        );
        draw_text(
            std::format!("DIED {} TIMES", self.player_died_times).as_str(),
            20.0,
            80.0,
            20.0,
            YELLOW,
        );
    }
}

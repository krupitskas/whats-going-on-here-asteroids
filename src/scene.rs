use std::cmp::max;

use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{
    asteroid::{Asteroid, AsteroidType}, bullet::Bullet, constants, math::contrain_play_area, ship::Ship, texture_manager::{SpriteId, TextureManager}
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

            let window_size = screen_size();
            let safe_zone = 40.0;

            let x = if rand::gen_range(0.0, 1.0) < 0.5 {
                rand::gen_range(-safe_zone, 0.0)
            } else {
                rand::gen_range(window_size.0, window_size.0 + safe_zone)
            };

            let y = if rand::gen_range(0.0, 1.0) < 0.5 {
                rand::gen_range(-safe_zone, 0.0)
            } else {
                rand::gen_range(window_size.1, window_size.1 + safe_zone)
            };

            let direction = (Vec2 { x: window_size.0 / 2.0, y: window_size.1 / 2.0 } - Vec2 { x, y }).normalize();

            let new_asteroid = Asteroid::new(Vec2 { x, y }, direction);

            println!("new asteroid: {} {}", x, y);

            self.asteroids.push(new_asteroid);
        }
    }

    pub fn check_asteroids_vs_player(&mut self, delta_time: f32) {
        for asteroid in self.asteroids.iter_mut() {
            if asteroid.alive {
                asteroid.position += asteroid.direction * delta_time * asteroid.active_type.speed();
                asteroid.rotation += asteroid.active_type.rotation_speed() * delta_time;

                asteroid.position = contrain_play_area(asteroid.position);

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
                bullet.position = contrain_play_area(bullet.position);
                
                bullet.time_passed += delta_time;

                if bullet.time_passed > 2.0 {
                    bullet.alive = false;
                    continue;
                }

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
                                    rotation: 0.0,
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
                                    rotation: 0.0,
                                    direction: direction,
                                    alive: true,
                                });
                            }
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
        // draw_circle(
        //     self.player.position.x,
        //     self.player.position.y,
        //     constants::SHIP_SIZE / 2.0,
        //     RED,
        // );

        self.texture_manager
            .textures
            .get(&SpriteId::Player)
            .unwrap()
            .draw(
                self.player.position,
                self.player.rotation,
                constants::SHIP_SIZE,
            );

        if self.player.booster_active {
            self.texture_manager
                .textures
                .get(&SpriteId::PlayerBooster)
                .unwrap()
                .draw(self.player.exhaust_pos, self.player.rotation, 10.0);
        }

        // draw_line(
        //     self.player.barrel_pos.x,
        //     self.player.barrel_pos.y,
        //     self.player.barrel_pos.x + self.player.direction.x * 100.0,
        //     self.player.barrel_pos.y + self.player.direction.y * 100.0,
        //     1.0,
        //     YELLOW,
        // );

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
                // draw_circle(
                //     asteroid.position.x,
                //     asteroid.position.y,
                //     asteroid.active_type.size(),
                //     RED,
                // );

                self.texture_manager
                    .textures
                    .get(&SpriteId::Asteroid)
                    .unwrap()
                    .draw(
                        asteroid.position,
                        asteroid.rotation,
                        asteroid.active_type.size(),
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

use std::cmp::max;

use macroquad::{miniquad::window::screen_size, prelude::*, rand::gen_range};

use crate::{
    asteroid::{Asteroid, AsteroidType},
    bullet::Bullet,
    constants,
    enemy::Enemy,
    enemy_projectile::EnemyProjectile,
    enemy_vision::VisionTarget,
    explosion::Explosion,
    math::{circles_overlap, contrain_play_area},
    ship::Ship,
    texture_manager::{SpriteId, TextureManager},
};

#[derive(PartialEq)]
pub enum SceneState {
    MainMenu,
    InGame,
    PlayerDying,
    Lost,
}

enum BonBonCollision {
    Player {
        enemy_index: usize,
    },
    Asteroid {
        enemy_index: usize,
        asteroid_index: usize,
    },
    Enemy {
        enemy_index: usize,
        other_enemy_index: usize,
    },
}

pub struct Scene {
    pub player: Ship,
    pub player_explosion: Option<Explosion>,
    pub impact_explosions: Vec<Explosion>,
    pub asteroids: Vec<Asteroid>,
    pub new_asteroids: Vec<Asteroid>,
    pub bullets: Vec<Bullet>,
    pub enemy_projectiles: Vec<EnemyProjectile>,
    pub enemies: Vec<Enemy>,
    pub last_time_asteroid_spawned: f64,
    pub last_time_enemy_spawned: f64,
    pub level_started_at: f64,
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
            player_explosion: None,
            impact_explosions: Vec::new(),
            asteroids: Vec::new(),
            new_asteroids: Vec::new(),
            bullets: Vec::new(),
            enemy_projectiles: Vec::new(),
            enemies: Vec::new(),
            last_time_asteroid_spawned: 0.0,
            last_time_enemy_spawned: 0.0,
            level_started_at: 0.0,
            player_score: 0,
            best_player_score: 0,
            player_died_times: 0,
            current_state: SceneState::MainMenu,
            texture_manager: TextureManager::new(),
        }
    }

    pub fn new_level(&mut self, screen_size: Vec2) {
        self.best_player_score = max(self.best_player_score, self.player_score);
        self.player_score = 0;
        self.player = Ship::new(screen_size);
        self.player_explosion = None;
        self.impact_explosions.clear();
        self.asteroids.clear();
        self.new_asteroids.clear();
        self.bullets.clear();
        self.enemy_projectiles.clear();
        self.enemies.clear();

        let now = get_time();
        self.last_time_asteroid_spawned = now;
        self.last_time_enemy_spawned = now;
        self.level_started_at = now;
    }

    pub fn try_spawn_asteroid(&mut self) {
        if (get_time() - self.last_time_asteroid_spawned) > constants::ASTEROID_SPAWN_SEC {
            self.last_time_asteroid_spawned = get_time();

            let window_size = screen_size();
            let safe_zone = 40.0;

            let x = if gen_range(0.0, 1.0) < 0.5 {
                gen_range(-safe_zone, 0.0)
            } else {
                gen_range(window_size.0, window_size.0 + safe_zone)
            };

            let y = if gen_range(0.0, 1.0) < 0.5 {
                gen_range(-safe_zone, 0.0)
            } else {
                gen_range(window_size.1, window_size.1 + safe_zone)
            };

            let direction =
                (Vec2::new(window_size.0 / 2.0, window_size.1 / 2.0) - Vec2 { x, y }).normalize();

            self.asteroids.push(Asteroid::new(Vec2 { x, y }, direction));
        }
    }

    pub fn enemy_spawn_interval(&self) -> f64 {
        let elapsed = get_time() - self.level_started_at;
        let step = (elapsed / constants::ENEMY_SPAWN_STEP_SEC).floor();

        (constants::ENEMY_SPAWN_INITIAL_SEC - step).max(constants::ENEMY_SPAWN_MIN_SEC)
    }

    pub fn try_spawn_enemy(&mut self) {
        if (get_time() - self.last_time_enemy_spawned) < self.enemy_spawn_interval() {
            return;
        }

        self.last_time_enemy_spawned = get_time();
        self.enemies
            .push(Enemy::spawn_random(self.random_enemy_spawn_position()));
    }

    pub fn next_enemy_spawn_in(&self) -> f64 {
        (self.enemy_spawn_interval() - (get_time() - self.last_time_enemy_spawned)).max(0.0)
    }

    pub fn random_enemy_spawn_position(&self) -> Vec2 {
        let max_radius = constants::ALAN_ENEMY_RADIUS.max(constants::BON_BON_ENEMY_RADIUS);
        let margin = max_radius * 2.0;
        let safe_distance = 140.0;

        for _ in 0..12 {
            let position = Vec2::new(
                gen_range(margin, screen_width() - margin),
                gen_range(margin, screen_height() - margin),
            );

            if position.distance(self.player.position) >= safe_distance {
                return position;
            }
        }

        Vec2::new(screen_width() - margin, margin)
    }

    pub fn update_asteroids(&mut self, delta_time: f32) {
        for asteroid in self.asteroids.iter_mut() {
            if asteroid.alive {
                asteroid.position += asteroid.direction * delta_time * asteroid.active_type.speed();
                asteroid.rotation += asteroid.active_type.rotation_speed() * delta_time;
                asteroid.position = contrain_play_area(asteroid.position);
            }
        }
    }

    pub fn destroy_asteroid(&mut self, asteroid_index: usize, grant_score: bool) {
        if asteroid_index >= self.asteroids.len() || !self.asteroids[asteroid_index].alive {
            return;
        }

        let asteroid_type = self.asteroids[asteroid_index].active_type;
        let asteroid_position = self.asteroids[asteroid_index].position;

        self.asteroids[asteroid_index].alive = false;

        if grant_score {
            self.player_score += asteroid_type.points();
        }

        if asteroid_type == AsteroidType::Small {
            return;
        }

        let new_type = if asteroid_type == AsteroidType::Big {
            AsteroidType::Medium
        } else if asteroid_type == AsteroidType::Medium {
            AsteroidType::Small
        } else {
            unimplemented!();
        };

        for fallback_direction in [Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)] {
            let x = gen_range(0.0, screen_width());
            let y = gen_range(0.0, screen_height());

            let mut direction = (asteroid_position - Vec2 { x, y }).normalize_or_zero();
            if direction.length_squared() == 0.0 {
                direction = fallback_direction;
            }

            self.new_asteroids.push(Asteroid {
                active_type: new_type,
                position: asteroid_position,
                direction,
                rotation: 0.0,
                alive: true,
            });
        }
    }

    pub fn damage_enemy(&mut self, enemy_index: usize, grant_score: bool) {
        if enemy_index >= self.enemies.len() || !self.enemies[enemy_index].alive() {
            return;
        }

        let enemy_position = self.enemies[enemy_index].position();
        let enemy_points = self.enemies[enemy_index].points();
        let explosion_size = self.enemies[enemy_index].explosion_size();

        if self.enemies[enemy_index].damage() {
            if grant_score {
                self.player_score += enemy_points;
            }

            self.impact_explosions
                .push(Explosion::new(enemy_position, 0.0, explosion_size));
        }
    }

    pub fn build_alan_vision_targets(&self) -> Vec<VisionTarget> {
        let mut targets = Vec::new();

        if self.current_state == SceneState::InGame {
            targets.push(VisionTarget {
                position: self.player.position,
                radius: constants::SHIP_SIZE / 2.0,
            });
        }

        for enemy in self.enemies.iter() {
            if enemy.alive() && enemy.is_bon_bon() {
                targets.push(VisionTarget {
                    position: enemy.position(),
                    radius: enemy.radius(),
                });
            }
        }

        targets
    }

    pub fn update_enemies(&mut self, delta_time: f32) {
        let vision_targets = self.build_alan_vision_targets();
        let player_position = self.player.position;

        for enemy in self.enemies.iter_mut() {
            if let Some(projectile) = enemy.update(delta_time, player_position, &vision_targets) {
                self.enemy_projectiles.push(projectile);
            }
        }
    }

    pub fn begin_player_destruction(&mut self) {
        if self.current_state != SceneState::InGame {
            return;
        }

        self.current_state = SceneState::PlayerDying;
        self.player_died_times += 1;
        self.player_explosion = Some(Explosion::new(
            self.player.position,
            self.player.rotation,
            constants::PLAYER_EXPLOSION_SIZE,
        ));
    }

    fn find_bon_bon_collisions(&self) -> Vec<BonBonCollision> {
        let mut collisions = Vec::new();

        for enemy_index in 0..self.enemies.len() {
            let enemy = &self.enemies[enemy_index];

            if !enemy.alive() || !enemy.is_bon_bon() {
                continue;
            }

            let position = enemy.position();
            let radius = enemy.radius();

            if self.current_state == SceneState::InGame
                && circles_overlap(
                    position,
                    radius,
                    self.player.position,
                    constants::SHIP_SIZE / 2.0,
                )
            {
                collisions.push(BonBonCollision::Player { enemy_index });
                continue;
            }

            if let Some(asteroid_index) = self.asteroids.iter().position(|asteroid| {
                asteroid.alive
                    && circles_overlap(
                        position,
                        radius,
                        asteroid.position,
                        asteroid.active_type.size(),
                    )
            }) {
                collisions.push(BonBonCollision::Asteroid {
                    enemy_index,
                    asteroid_index,
                });
                continue;
            }

            for other_enemy_index in 0..self.enemies.len() {
                if enemy_index == other_enemy_index {
                    continue;
                }

                let other_enemy = &self.enemies[other_enemy_index];

                if !other_enemy.alive() {
                    continue;
                }

                if other_enemy.is_bon_bon() && other_enemy_index < enemy_index {
                    continue;
                }

                if enemy.colliding_enemy(other_enemy) {
                    collisions.push(BonBonCollision::Enemy {
                        enemy_index,
                        other_enemy_index,
                    });
                    break;
                }
            }
        }

        collisions
    }

    pub fn resolve_bon_bon_collisions(&mut self) {
        let collisions = self.find_bon_bon_collisions();

        for collision in collisions {
            match collision {
                BonBonCollision::Player { enemy_index } => {
                    if enemy_index >= self.enemies.len() || !self.enemies[enemy_index].alive() {
                        continue;
                    }

                    self.damage_enemy(enemy_index, false);
                    self.begin_player_destruction();
                }
                BonBonCollision::Asteroid {
                    enemy_index,
                    asteroid_index,
                } => {
                    if enemy_index >= self.enemies.len()
                        || asteroid_index >= self.asteroids.len()
                        || !self.enemies[enemy_index].alive()
                        || !self.asteroids[asteroid_index].alive
                    {
                        continue;
                    }

                    self.damage_enemy(enemy_index, false);
                    self.destroy_asteroid(asteroid_index, false);
                }
                BonBonCollision::Enemy {
                    enemy_index,
                    other_enemy_index,
                } => {
                    if enemy_index >= self.enemies.len()
                        || other_enemy_index >= self.enemies.len()
                        || !self.enemies[enemy_index].alive()
                        || !self.enemies[other_enemy_index].alive()
                    {
                        continue;
                    }

                    self.damage_enemy(enemy_index, false);
                    self.damage_enemy(other_enemy_index, false);
                }
            }
        }
    }

    pub fn check_player_collisions(&mut self) {
        for asteroid in self.asteroids.iter() {
            if asteroid.alive && asteroid.colliding_ship(&self.player) {
                self.begin_player_destruction();
                return;
            }
        }

        for enemy in self.enemies.iter() {
            if enemy.alive() && enemy.colliding_ship_position(self.player.position) {
                self.begin_player_destruction();
                return;
            }
        }
    }

    pub fn update_bullets(&mut self, delta_time: f32) {
        for bullet_index in 0..self.bullets.len() {
            if !self.bullets[bullet_index].alive {
                continue;
            }

            {
                let bullet = &mut self.bullets[bullet_index];
                bullet.position += bullet.direction * delta_time * constants::BULLET_SPEED;
                bullet.position = contrain_play_area(bullet.position);
                bullet.time_passed += delta_time;
            }

            if self.bullets[bullet_index].time_passed > 2.0 {
                self.bullets[bullet_index].alive = false;
                continue;
            }

            let asteroid_hit = {
                let bullet = &self.bullets[bullet_index];

                (0..self.asteroids.len()).find(|&asteroid_index| {
                    self.asteroids[asteroid_index].alive
                        && self.asteroids[asteroid_index].colliding_bullet(bullet)
                })
            };

            if let Some(asteroid_index) = asteroid_hit {
                self.bullets[bullet_index].alive = false;
                self.destroy_asteroid(asteroid_index, true);
                continue;
            }

            let enemy_hit = {
                let bullet = &self.bullets[bullet_index];

                (0..self.enemies.len()).find(|&enemy_index| {
                    self.enemies[enemy_index].alive()
                        && self.enemies[enemy_index].colliding_bullet(bullet)
                })
            };

            if let Some(enemy_index) = enemy_hit {
                self.bullets[bullet_index].alive = false;
                self.damage_enemy(enemy_index, true);
            }
        }

        self.asteroids.append(&mut self.new_asteroids);
    }

    pub fn update_enemy_projectiles(&mut self, delta_time: f32) {
        for projectile_index in 0..self.enemy_projectiles.len() {
            if !self.enemy_projectiles[projectile_index].alive {
                continue;
            }

            {
                let projectile = &mut self.enemy_projectiles[projectile_index];
                projectile.position +=
                    projectile.direction * delta_time * constants::ALAN_PROJECTILE_SPEED;
                projectile.position = contrain_play_area(projectile.position);
                projectile.time_passed += delta_time;
            }

            if self.enemy_projectiles[projectile_index].time_passed
                > constants::ENEMY_PROJECTILE_LIFETIME
            {
                self.enemy_projectiles[projectile_index].alive = false;
                continue;
            }

            let projectile_position = self.enemy_projectiles[projectile_index].position;

            if self.current_state == SceneState::InGame
                && circles_overlap(
                    projectile_position,
                    constants::BULLET_SIZE / 2.0,
                    self.player.position,
                    constants::SHIP_SIZE / 2.0,
                )
            {
                self.enemy_projectiles[projectile_index].alive = false;
                self.begin_player_destruction();
                continue;
            }

            let bon_bon_hit = (0..self.enemies.len()).find(|&enemy_index| {
                self.enemies[enemy_index].alive()
                    && self.enemies[enemy_index].is_bon_bon()
                    && circles_overlap(
                        projectile_position,
                        constants::BULLET_SIZE / 2.0,
                        self.enemies[enemy_index].position(),
                        self.enemies[enemy_index].radius(),
                    )
            });

            if let Some(enemy_index) = bon_bon_hit {
                self.enemy_projectiles[projectile_index].alive = false;
                self.damage_enemy(enemy_index, false);
            }
        }
    }

    pub fn update_player_explosion(&mut self, delta_time: f32) {
        let Some(explosion) = self.player_explosion.as_mut() else {
            self.current_state = SceneState::Lost;
            return;
        };

        let sprite = self
            .texture_manager
            .textures
            .get(&SpriteId::ExplosionVFX)
            .unwrap();

        if explosion.update(delta_time, sprite) {
            self.current_state = SceneState::Lost;
            self.player_explosion = None;
        }
    }

    pub fn update_impact_explosions(&mut self, delta_time: f32) {
        let sprite = self
            .texture_manager
            .textures
            .get(&SpriteId::ExplosionVFX)
            .unwrap();

        for explosion in self.impact_explosions.iter_mut() {
            explosion.update(delta_time, sprite);
        }

        self.impact_explosions
            .retain(|explosion| !explosion.animation.finished);
    }

    pub fn cleanup_entities(&mut self) {
        self.bullets.retain(|bullet| bullet.alive);
        self.enemy_projectiles.retain(|projectile| projectile.alive);
        self.asteroids.retain(|asteroid| asteroid.alive);
        self.enemies.retain(|enemy| enemy.alive());
    }

    pub fn update(&mut self, delta_time: f32) {
        match self.current_state {
            SceneState::InGame => {
                self.player.update_inputs(&mut self.bullets);
                self.try_spawn_asteroid();
                self.try_spawn_enemy();
                self.update_asteroids(delta_time);
                self.update_enemies(delta_time);
                self.resolve_bon_bon_collisions();
                self.check_player_collisions();

                if self.current_state == SceneState::InGame {
                    self.update_bullets(delta_time);
                    self.update_enemy_projectiles(delta_time);
                }

                self.update_impact_explosions(delta_time);
                self.cleanup_entities();
            }
            SceneState::PlayerDying => {
                self.update_asteroids(delta_time);
                self.update_enemies(delta_time);
                self.resolve_bon_bon_collisions();
                self.update_bullets(delta_time);
                self.update_enemy_projectiles(delta_time);
                self.update_impact_explosions(delta_time);
                self.cleanup_entities();
                self.update_player_explosion(delta_time);
            }
            SceneState::MainMenu | SceneState::Lost => {}
        }
    }

    pub fn render_background(&mut self, delta_time: f32) {
        let pos = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let target = Vec2::new(screen_width(), screen_height());

        self.texture_manager
            .textures
            .get_mut(&SpriteId::Background0)
            .unwrap()
            .draw_animated_cover(delta_time, pos, 0.0, target);

        self.texture_manager
            .textures
            .get_mut(&SpriteId::Background1)
            .unwrap()
            .draw_animated_cover(delta_time, pos, 0.0, target);
    }

    pub fn render(&mut self, delta_time: f32) {
        clear_background(MAGENTA);
        self.render_background(delta_time);

        if self.current_state == SceneState::MainMenu {
            let pos = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
            self.texture_manager
                .textures
                .get(&SpriteId::StartUI)
                .unwrap()
                .draw(pos, 0.0, 360.0);
            return;
        }

        if self.current_state == SceneState::Lost {
            let pos = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
            self.texture_manager
                .textures
                .get(&SpriteId::GameOverUI)
                .unwrap()
                .draw(pos, 0.0, 420.0);
            return;
        }

        if self.current_state == SceneState::InGame {
            self.player
                .render(delta_time, &mut self.texture_manager.textures);
        }

        for enemy in self.enemies.iter_mut() {
            enemy.render(delta_time, &mut self.texture_manager.textures);
        }

        for asteroid in self.asteroids.iter() {
            if asteroid.alive {
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

        for bullet in self.bullets.iter() {
            if bullet.alive {
                self.texture_manager
                    .textures
                    .get_mut(&SpriteId::PlayerBullet)
                    .unwrap()
                    .draw_animated(delta_time, bullet.position, 0.0, 40.0);
            }
        }

        for projectile in self.enemy_projectiles.iter() {
            if projectile.alive {
                self.texture_manager
                    .textures
                    .get_mut(&SpriteId::AlanProjectile)
                    .unwrap()
                    .draw_animated(
                        delta_time,
                        projectile.position,
                        0.0,
                        constants::ENEMY_PROJECTILE_SIZE,
                    );
            }
        }

        for explosion in self.impact_explosions.iter() {
            explosion.render(&mut self.texture_manager);
        }

        if let Some(explosion) = self.player_explosion.as_ref() {
            explosion.render(&mut self.texture_manager);
        }

        draw_text(
            std::format!("dt: {:.3}s", delta_time).as_str(),
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
        draw_text(
            std::format!("ENEMIES: {}", self.enemies.len()).as_str(),
            20.0,
            100.0,
            20.0,
            YELLOW,
        );
        draw_text(
            std::format!("NEXT ENEMY: {:.1}s", self.next_enemy_spawn_in()).as_str(),
            20.0,
            120.0,
            20.0,
            SKYBLUE,
        );
    }
}

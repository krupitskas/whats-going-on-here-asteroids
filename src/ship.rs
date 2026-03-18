use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    bullet::Bullet,
    constants::{self, BULLET_RELOAD_TIME, SHIP_SIZE},
    math::{contrain_play_area, rotate_vector},
    texture_manager::{Sprite, SpriteId},
};

pub enum ActiveTilt {
    None,
    Left,
    Right,
}

pub struct Ship {
    pub position: Vec2,
    pub direction: Vec2,
    pub barrel_pos: Vec2,
    pub exhaust_pos: Vec2,
    pub rotation: f32,
    pub speed: f32,
    pub rotation_speed: f32,
    pub inertia: Vec2,
    pub booster_active: bool,
    pub engine_active: bool,
    pub active_tilt: ActiveTilt,
    pub last_shot_time: f64, // Something better for reloading?
}

impl Ship {
    pub fn new(screen_size: Vec2) -> Ship {
        Ship {
            position: Vec2 {
                x: screen_size.x / 2.0,
                y: screen_size.y / 2.0,
            },
            direction: Vec2 { x: 0.0, y: -1.0 },
            barrel_pos: Vec2::default(),
            exhaust_pos: Vec2::default(),
            rotation: 0.0,
            speed: 5.0,
            rotation_speed: 3.0, // rad / sec?
            inertia: Vec2 { x: 0.0, y: 0.0 },
            active_tilt: ActiveTilt::None,
            booster_active: false,
            engine_active: false,
            last_shot_time: 0.0,
        }
    }

    pub fn update_inputs(&mut self, bullets: &mut Vec<Bullet>) {
        self.booster_active = false;
        self.engine_active = false;
        self.active_tilt = ActiveTilt::None;

        let delta_time = get_frame_time();

        let boost_multiplier: f32 = if is_key_down(KeyCode::LeftShift) {
            self.booster_active = true;
            2.0
        } else {
            1.0
        };

        if is_key_down(KeyCode::A) {
            self.rotation -= self.rotation_speed * delta_time;
        }

        if is_key_down(KeyCode::D) {
            self.rotation += self.rotation_speed * delta_time;
        }

        let vec_up = Vec2 { x: 0.0, y: -1.0 };

        self.direction = rotate_vector(vec_up, self.rotation);

        self.barrel_pos = self.position + self.direction * SHIP_SIZE / 2.0;
        self.exhaust_pos = self.position - self.direction * (SHIP_SIZE / 2.0 + 10.0);

        if is_key_down(KeyCode::W) {
            self.inertia += self.direction * self.speed * delta_time * boost_multiplier;
            self.engine_active = true;
        }
        if is_key_down(KeyCode::S) {
            self.inertia -= self.inertia * 1.5 * delta_time;
        }
        if is_key_down(KeyCode::Q) {
            self.active_tilt = ActiveTilt::Left;
            self.inertia -= self.direction.perp() * self.speed * delta_time / 2.0;
        }
        if is_key_down(KeyCode::E) {
            self.active_tilt = ActiveTilt::Right;
            self.inertia += self.direction.perp() * self.speed * delta_time / 2.0;
        }

        self.position += self.inertia;
        self.position = contrain_play_area(self.position);

        self.inertia -= self.inertia * 1.5 * delta_time;

        if is_key_down(KeyCode::Space) && (get_time() - self.last_shot_time) > BULLET_RELOAD_TIME {
            self.last_shot_time = get_time();

            bullets.push(Bullet {
                position: self.barrel_pos,
                direction: self.direction,
                alive: true,
                time_passed: 0.0,
            });
        }
    }

    pub fn render(&self, _delta_time: f32, sprites: &mut HashMap<SpriteId, Sprite>) {
        // draw_circle(
        //     self.player.position.x,
        //     self.player.position.y,
        //     constants::SHIP_SIZE / 2.0,
        //     RED,
        // );

        let ship_sprite = sprites.get_mut(&SpriteId::Player).unwrap();

        match self.active_tilt {
            ActiveTilt::None => ship_sprite.texture_index = 1,
            ActiveTilt::Left => ship_sprite.texture_index = 0,
            ActiveTilt::Right => ship_sprite.texture_index = 2,
        }

        ship_sprite.draw(self.position, self.rotation, constants::SHIP_SIZE);

        let booster_sprite = sprites.get_mut(&SpriteId::PlayerBooster).unwrap();

        if self.booster_active {
            booster_sprite.texture_index = 0;
            booster_sprite.draw(self.exhaust_pos, self.rotation, 10.0);
        } else if self.engine_active {
            booster_sprite.texture_index = 1;
            booster_sprite.draw(self.exhaust_pos, self.rotation, 10.0);
        }

        // draw_line(
        //     self.player.barrel_pos.x,
        //     self.player.barrel_pos.y,
        //     self.player.barrel_pos.x + self.player.direction.x * 100.0,
        //     self.player.barrel_pos.y + self.player.direction.y * 100.0,
        //     1.0,
        //     YELLOW,
        // );
    }
}

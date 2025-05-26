use macroquad::prelude::*;

use crate::{
    bullet::Bullet,
    constants::{BULLET_RELOAD_TIME, SHIP_SIZE}, math::contrain_play_area,
};

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
    pub last_shot_time: f64, // Something better for reloading?
}

fn get_angle(a: Vec2, b: Vec2) -> f32 {
    let angle = (b.x * a.y - b.y * a.x).atan2(b.x * a.x + b.y * a.y);
    let angle = if angle < 0.0 {
        angle + std::f32::consts::TAU
    } else {
        angle
    };
    angle
}

fn rotate_vector(v: Vec2, angle_rad: f32) -> Vec2 {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    Vec2::new(
        v.x * cos_theta - v.y * sin_theta,
        v.x * sin_theta + v.y * cos_theta,
    )
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
            booster_active: false,
            last_shot_time: 0.0,
        }
    }

    pub fn update_inputs(&mut self, bullets: &mut Vec<Bullet>) {
        self.booster_active = false;

        let delta_time = get_frame_time();

        let boost_multiplier: f32 = if is_key_down(KeyCode::LeftShift) {
            2.0
        } else {
            1.0
        };

        let mouse_pos = mouse_position();

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
            self.booster_active = true;
        }
        if is_key_down(KeyCode::S) {
            self.inertia -= self.inertia * 0.01;
        }
        // if is_key_down(KeyCode::S) {
        //     self.inertia.y += self.speed * delta_time * boost_multiplier;
        // }
        // if is_key_down(KeyCode::D) {
        //     self.inertia.x += self.speed * delta_time * boost_multiplier;
        // }

        self.position += self.inertia;
        self.position = contrain_play_area(self.position);

        self.inertia -= self.inertia * 0.01;

        if is_key_down(KeyCode::Space)
            && (get_time() as f64 - self.last_shot_time) > BULLET_RELOAD_TIME
        {
            self.last_shot_time = get_time();

            bullets.push(Bullet {
                position: self.barrel_pos,
                direction: self.direction,
                alive: true,
                time_passed: 0.0
            });
        }
    }
}

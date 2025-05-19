use macroquad::prelude::*;

use crate::{ bullet::Bullet, constants::BULLET_RELOAD_TIME, constants::SHIP_SIZE };

pub struct Ship {
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub inertia: Vec2,
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
            speed: 200.0,
            inertia: Vec2 { x: 0.0, y: 0.0 },
            last_shot_time: 0.0,
        }
    }

    pub fn update_inputs(&mut self, bullets: &mut Vec<Bullet>) {
        let delta_time = get_frame_time();

        let boost_multiplier: f32 = if is_key_down(KeyCode::LeftShift) {
            2.0
        } else {
            1.0
        };

        if is_key_down(KeyCode::W) {
            self.position.y -= self.speed * delta_time * boost_multiplier;
        }
        if is_key_down(KeyCode::A) {
            self.position.x -= self.speed * delta_time * boost_multiplier;
        }
        if is_key_down(KeyCode::S) {
            self.position.y += self.speed * delta_time * boost_multiplier;
        }
        if is_key_down(KeyCode::D) {
            self.position.x += self.speed * delta_time * boost_multiplier;
        }

        let mouse_pos = mouse_position();

        self.direction = (Vec2 {
            x: mouse_pos.0,
            y: mouse_pos.1,
        } - self.position)
            .normalize();

        if is_mouse_button_down(MouseButton::Left)
            && (get_time() as f64 - self.last_shot_time) > BULLET_RELOAD_TIME
        {
            self.last_shot_time = get_time();

            bullets.push(Bullet {
                position: self.position,
                direction: self.direction,
                alive: true,
            });
        }
    }

    pub fn render(&self) {
        draw_circle(self.position.x, self.position.y, SHIP_SIZE, RED);
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.direction.x * 100.0,
            self.position.y + self.direction.y * 100.0,
            1.0,
            YELLOW,
        );

        // debug
        draw_circle(mouse_position().0, mouse_position().1, 5.0, PURPLE);
    }
}
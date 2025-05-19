use macroquad::prelude::*;

pub struct Bullet {
    pub position: Vec2,
    pub direction: Vec2,
    pub alive: bool,
}

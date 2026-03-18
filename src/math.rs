use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};

pub fn contrain_play_area(pos: Vec2) -> Vec2 {
    let mut new_pos = pos;

    if pos.x < -80.0 {
        new_pos.x = screen_width() + 80.0;
    } else if pos.x > screen_width() + 80.0 {
        new_pos.x = -80.0;
    }

    if pos.y < -80.0 {
        new_pos.y = screen_height() + 80.0;
    } else if pos.y > screen_height() + 80.0 {
        new_pos.y = -80.0;
    }

    new_pos
}

pub fn rotate_vector(v: Vec2, angle_rad: f32) -> Vec2 {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    Vec2::new(
        v.x * cos_theta - v.y * sin_theta,
        v.x * sin_theta + v.y * cos_theta,
    )
}

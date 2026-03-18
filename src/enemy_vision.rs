use std::f32::consts::TAU;

use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct VisionTarget {
    pub position: Vec2,
    pub radius: f32,
}

#[derive(Clone, Copy)]
pub struct EnemyRay {
    pub end: Vec2,
    pub hit_target: bool,
}

pub struct EnemyVision {
    pub rays: Vec<EnemyRay>,
    pub target_visible: bool,
    pub ray_count: u16,
    pub max_distance: f32,
}

impl EnemyVision {
    pub fn new(ray_count: u16, max_distance: f32) -> EnemyVision {
        EnemyVision {
            rays: Vec::with_capacity(ray_count as usize),
            target_visible: false,
            ray_count,
            max_distance,
        }
    }

    pub fn scan(&mut self, origin: Vec2, targets: &[VisionTarget]) -> Option<Vec2> {
        self.rays.clear();
        self.target_visible = false;

        let mut closest_target: Option<(f32, Vec2)> = None;

        for ray_index in 0..self.ray_count {
            let angle = TAU * ray_index as f32 / self.ray_count as f32;
            let direction = Vec2::new(angle.cos(), angle.sin());
            let mut ray_end = origin + direction * self.max_distance;
            let mut hit_any_target = false;

            for target in targets {
                if let Some(hit_distance) = ray_circle_hit(
                    origin,
                    direction,
                    target.position,
                    target.radius,
                    self.max_distance,
                ) {
                    let hit_point = origin + direction * hit_distance;

                    if !hit_any_target
                        || hit_point.distance_squared(origin) < ray_end.distance_squared(origin)
                    {
                        ray_end = hit_point;
                    }

                    hit_any_target = true;
                    self.target_visible = true;

                    let target_distance = target.position.distance_squared(origin);
                    match closest_target {
                        Some((closest_distance, _)) if closest_distance <= target_distance => {}
                        _ => {
                            closest_target = Some((target_distance, target.position));
                        }
                    }
                }
            }

            self.rays.push(EnemyRay {
                end: ray_end,
                hit_target: hit_any_target,
            });
        }

        closest_target.map(|(_, position)| position)
    }

    pub fn render(&self, origin: Vec2) {
        for ray in &self.rays {
            let color = if ray.hit_target {
                Color::new(1.0, 0.25, 0.2, 0.85)
            } else {
                Color::new(0.75, 0.9, 1.0, 0.14)
            };

            draw_line(origin.x, origin.y, ray.end.x, ray.end.y, 1.0, color);
        }

        if self.target_visible {
            draw_circle_lines(
                origin.x,
                origin.y,
                22.0,
                1.5,
                Color::new(1.0, 0.3, 0.2, 0.8),
            );
        }
    }
}

fn ray_circle_hit(
    ray_origin: Vec2,
    ray_direction: Vec2,
    circle_center: Vec2,
    circle_radius: f32,
    max_distance: f32,
) -> Option<f32> {
    let to_circle = circle_center - ray_origin;
    let projection = to_circle.dot(ray_direction);

    if projection < 0.0 {
        return None;
    }

    let closest_point = ray_origin + ray_direction * projection;
    let perpendicular_distance_sq = circle_center.distance_squared(closest_point);
    let radius_sq = circle_radius * circle_radius;

    if perpendicular_distance_sq > radius_sq {
        return None;
    }

    let offset = (radius_sq - perpendicular_distance_sq).sqrt();
    let mut hit_distance = projection - offset;

    if hit_distance < 0.0 {
        hit_distance = projection + offset;
    }

    (hit_distance <= max_distance).then_some(hit_distance)
}

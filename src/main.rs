mod constants;
mod asteroid;
mod bullet;
mod game_settings;
mod scene;
mod ship;

use macroquad::{miniquad::window::screen_size, prelude::*};
use std::cmp::max;

use asteroid::{Asteroid, AsteroidType};
use bullet::Bullet;
use game_settings::GameSettings;
use scene::Scene;
use ship::Ship;


/*
inertia
clipping screen edges
more vfx when asteroids break
enemies

*/

#[macroquad::main("asteroids")]
async fn main() {
    let game_settings = GameSettings::default();

    request_new_screen_size(game_settings.window_size.x, game_settings.window_size.y);

    let mut scene = Scene::new(game_settings.window_size);

    let mut player_lost = false;

    loop {
        if player_lost {
            scene.new_level(screen_size().into());
            player_lost = false;
        }

        // Update
        let delta_time = get_frame_time(); // milliseconds

        if is_key_down(KeyCode::Escape) {
            break;
        }

        scene.player.update_inputs(&mut scene.bullets);

        if (get_time() - scene.last_time_asteroid_spawned) > constants::ASTEROID_SPAWN_SEC {
            scene.last_time_asteroid_spawned = get_time();

            let x = rand::gen_range(0.0, 800.0);
            let y = rand::gen_range(0.0, 800.0);

            let direction = (Vec2 { x: 400.0, y: 400.0 } - Vec2 { x, y }).normalize();

            let new_asteroid = Asteroid::new(Vec2 { x, y }, direction);

            scene.asteroids.push(new_asteroid);
        }

        for asteroid in scene.asteroids.iter_mut() {
            if asteroid.alive {
                asteroid.position += asteroid.direction * delta_time * asteroid.active_type.speed();

                if asteroid.colliding_ship(&scene.player) {
                    player_lost = true;
                    continue;
                }
            }
        }

        for bullet in scene.bullets.iter_mut() {
            if bullet.alive {
                bullet.position += bullet.direction * delta_time * constants::BULLET_SPEED;

                for asteroid in scene.asteroids.iter_mut() {
                    if asteroid.alive && asteroid.colliding_bullet(&bullet) {
                        bullet.alive = false;
                        asteroid.alive = false;
                        scene.player_score += asteroid.active_type.points();

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

                                scene.new_asteroids.push(Asteroid {
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

                                scene.new_asteroids.push(Asteroid {
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

        scene.asteroids.append(&mut scene.new_asteroids);

        // Render
        clear_background(DARKBLUE);

        scene.player.render();

        for bullet in scene.bullets.iter_mut() {
            if bullet.alive {
                draw_circle(bullet.position.x, bullet.position.y, constants::BULLET_SIZE, YELLOW);
            }
        }

        for asteroid in scene.asteroids.iter_mut() {
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
            std::format!("SCORE: {}", scene.player_score).as_str(),
            20.0,
            40.0,
            20.0,
            YELLOW,
        );
        draw_text(
            std::format!("BEST SCORE: {}", scene.best_player_score).as_str(),
            20.0,
            60.0,
            20.0,
            YELLOW,
        );
        draw_text(
            std::format!("DIED {} TIMES", scene.player_died_times).as_str(),
            20.0,
            80.0,
            20.0,
            YELLOW,
        );

        next_frame().await
    }
}


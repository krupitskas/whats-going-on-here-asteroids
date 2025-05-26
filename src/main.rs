mod constants;
mod asteroid;
mod bullet;
mod game_settings;
mod scene;
mod ship;
mod texture_manager;
mod math;
mod enemy;

use macroquad::{miniquad::window::screen_size, prelude::*};

use game_settings::GameSettings;
use scene::{Scene, SceneState};

#[macroquad::main("asteroids")]
async fn main() {
    let game_settings = GameSettings::default();

    request_new_screen_size(game_settings.window_size.x, game_settings.window_size.y);

    let mut scene = Scene::new(game_settings.window_size);
    scene.texture_manager.load_assets().await;

    loop {
        let delta_time = get_frame_time(); // milliseconds

        if scene.current_state == SceneState::Lost {
            scene.new_level(screen_size().into());
            scene.current_state = SceneState::InGame;
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        scene.update(delta_time);
        scene.render(delta_time);

        next_frame().await
    }
}


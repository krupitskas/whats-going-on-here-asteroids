use std::collections::HashMap;

use macroquad::{
    color::WHITE,
    math::{Rect, Vec2},
    texture::{DrawTextureParams, FilterMode, Texture2D, draw_texture_ex, load_texture},
};

#[derive(PartialEq, Eq, Hash)]
pub enum SpriteId {
    Player,
    Asteroid,
    PlayerBooster,
    Enemy,
    PlayerBullet,
    Background0,
    Background1,
    Background2,
}

pub struct Sprite {
    pub texture: Texture2D,
    pub size: Vec2,
    pub texture_index: u32,
    pub animation_count: u32,
    pub size_mult: f32,
    pub time_scince_frame: f32,
    pub fps: f32
}

impl Sprite {
    pub fn draw_animated(&mut self, delta_time: f32, pos: Vec2, rot: f32, size: f32) {
        let target_time_slice = 1.0 / self.fps as f32;

        self.time_scince_frame += delta_time; // BUG: Not sprite but instance of sprite

        if self.time_scince_frame > target_time_slice {
            self.time_scince_frame = 0.0;
            self.texture_index += 1;
            self.texture_index = self.texture_index % self.animation_count;
        }

        let size = size * self.size_mult;

        let draw_params = DrawTextureParams {
            dest_size: Some(Vec2 { x: size, y: size }),
            source: Some(Rect {
                x: self.texture_index as f32 * self.size.x,
                y: 0.0,
                w: self.size.x,
                h: self.size.y,
            }),
            rotation: rot,
            pivot: None,
            flip_x: false,
            flip_y: false,
        };

        draw_texture_ex(
            &self.texture,
            pos.x - size / 2.0,
            pos.y - size / 2.0,
            WHITE,
            draw_params,
        );
    }

    pub fn draw(&self, pos: Vec2, rot: f32, size: f32) {
        let size = size * self.size_mult;

        let draw_params = DrawTextureParams {
            dest_size: Some(Vec2 { x: size, y: size }),
            source: Some(Rect {
                x: self.texture_index as f32 * self.size.x,
                y: 0.0,
                w: self.size.x,
                h: self.size.y,
            }),
            rotation: rot,
            pivot: None,
            flip_x: false,
            flip_y: false,
        };

        draw_texture_ex(
            &self.texture,
            pos.x - size / 2.0,
            pos.y - size / 2.0,
            WHITE,
            draw_params,
        );
    }
}

pub struct TextureManager {
    pub textures: HashMap<SpriteId, Sprite>,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub async fn load_assets(&mut self) {
        // Player
        {
            let texture =
                load_texture("assets/Mini Pixel Pack 3/Player ship/Player_ship (16 x 16).png")
                    .await
                    .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::Player,
                Sprite {
                    texture: texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 1,
                    animation_count: 1,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: 0.0
                },
            );
        }

        // Enemy
        {
            let texture = load_texture("assets/Mini Pixel Pack 3/Enemies/Alan (16 x 16).png")
                .await
                .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::Enemy,
                Sprite {
                    texture: texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 0,
                    animation_count: 6,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: 3.0
                },
            );
        }
        let background_fps = 10.0;
        // Background0
        {
            let texture = load_texture("assets/Foozle_2DS0015_Void_EnvironmentPack/Backgrounds/PNGs/Condesed/Starry background  - Layer 01 - Void.png")
                .await
                .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::Background0,
                Sprite {
                    texture: texture,
                    size: Vec2 { x: 638.0, y: 360.0 },
                    texture_index: 0,
                    animation_count: 9,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: background_fps
                },
            );
        }

        // Background1
        {
            let texture = load_texture("assets/Foozle_2DS0015_Void_EnvironmentPack/Backgrounds/PNGs/Condesed/Starry background  - Layer 02 - Stars.png")
                .await
                .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::Background1,
                Sprite {
                    texture: texture,
                    size: Vec2 { x: 640.0, y: 360.0 },
                    texture_index: 0,
                    animation_count: 9,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: background_fps
                },
            );
        }

        // Background2
        {
            let texture = load_texture("assets/Foozle_2DS0015_Void_EnvironmentPack/Backgrounds/PNGs/Condesed/Starry background  - Layer 03 - Stars.png")
                .await
                .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::Background2,
                Sprite {
                    texture: texture,
                    size: Vec2 { x: 640.0, y: 360.0 },
                    texture_index: 0,
                    animation_count: 9,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: background_fps
                },
            );
        }

        // Player Bullet
        {
            let texture = load_texture(
                "assets/Mini Pixel Pack 3/Projectiles/Player_donut_shot (16 x 16).png",
            )
            .await
            .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::PlayerBullet,
                Sprite {
                    texture: texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 0,
                    animation_count: 2,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: 10.0
                },
            );
        }

        // Player Booster
        {
            let texture =
                load_texture("assets/Mini Pixel Pack 3/Player ship/Boosters (16 x 16).png")
                    .await
                    .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::PlayerBooster,
                Sprite {
                    texture: texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 0,
                    animation_count: 1,
                    size_mult: 3.0,
                    time_scince_frame: 0.0,
                    fps: 0.0
                },
            );
        }

        // Asteroid
        {
            let texture = load_texture(
                "assets/Foozle_2DS0015_Void_EnvironmentPack/Asteroids/PNGs/Asteroid 01 - Base.png",
            )
            .await
            .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::Asteroid,
                Sprite {
                    texture: texture,
                    size: Vec2 { x: 38.0, y: 33.0 },
                    texture_index: 0,
                    animation_count: 1,
                    size_mult: 2.0,
                    time_scince_frame: 0.0,
                    fps: 0.0
                },
            );
        }
    }
}

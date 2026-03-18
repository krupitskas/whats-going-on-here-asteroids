use std::collections::HashMap;

use macroquad::{
    color::{Color, WHITE},
    math::{Rect, Vec2},
    texture::{DrawTextureParams, FilterMode, Texture2D, draw_texture_ex, load_texture},
};

#[derive(PartialEq, Eq, Hash)]
pub enum SpriteId {
    Player,
    Asteroid,
    PlayerBooster,
    AlanEnemy,
    BonBonEnemy,
    AlanProjectile,
    PlayerBullet,
    Background0,
    Background1,
    Background2,
    ExplosionVFX,
    StartUI,
    GameOverUI,
}

pub struct Sprite {
    pub texture: Texture2D,
    pub size: Vec2,
    pub texture_index: u32,
    pub animation_count: u32,
    pub size_mult: f32,
    pub time_scince_frame: f32,
    pub fps: f32,
}

impl Sprite {
    fn scaled_dest_size(&self, target_size: f32) -> Vec2 {
        let base = self.size.x.max(self.size.y);
        let scale = (target_size * self.size_mult) / base;

        Vec2 {
            x: self.size.x * scale,
            y: self.size.y * scale,
        }
    }

    pub fn draw_animated_cover(&mut self, delta_time: f32, pos: Vec2, rot: f32, target: Vec2) {
        let target_time_slice = 1.0 / self.fps;

        self.time_scince_frame += delta_time;

        if self.time_scince_frame > target_time_slice {
            self.time_scince_frame = 0.0;
            self.texture_index += 1;
            self.texture_index %= self.animation_count;
        }

        let scale_x = target.x / self.size.x;
        let scale_y = target.y / self.size.y;
        let scale = scale_x.max(scale_y) * self.size_mult;

        let dest_size = Vec2 {
            x: self.size.x * scale,
            y: self.size.y * scale,
        };

        let draw_params = DrawTextureParams {
            dest_size: Some(dest_size),
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
            pos.x - dest_size.x / 2.0,
            pos.y - dest_size.y / 2.0,
            WHITE,
            draw_params,
        );
    }

    pub fn draw_animated(&mut self, delta_time: f32, pos: Vec2, rot: f32, size: f32) {
        let target_time_slice = 1.0 / self.fps;

        self.time_scince_frame += delta_time; // BUG: Not sprite but instance of sprite

        if self.time_scince_frame > target_time_slice {
            self.time_scince_frame = 0.0;
            self.texture_index += 1;
            self.texture_index %= self.animation_count;
        }

        let dest_size = self.scaled_dest_size(size);

        let draw_params = DrawTextureParams {
            dest_size: Some(dest_size),
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
            pos.x - dest_size.x / 2.0,
            pos.y - dest_size.y / 2.0,
            WHITE,
            draw_params,
        );
    }

    pub fn draw(&self, pos: Vec2, rot: f32, size: f32) {
        self.draw_frame(self.texture_index, pos, rot, size);
    }

    pub fn draw_frame(&self, frame_index: u32, pos: Vec2, rot: f32, size: f32) {
        self.draw_frame_scaled(frame_index, pos, rot, size, Vec2::ONE, WHITE);
    }

    pub fn draw_frame_scaled(
        &self,
        frame_index: u32,
        pos: Vec2,
        rot: f32,
        size: f32,
        scale: Vec2,
        tint: Color,
    ) {
        let dest_size = self.scaled_dest_size(size);
        let dest_size = Vec2::new(dest_size.x * scale.x, dest_size.y * scale.y);

        let draw_params = DrawTextureParams {
            dest_size: Some(dest_size),
            source: Some(Rect {
                x: frame_index as f32 * self.size.x,
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
            pos.x - dest_size.x / 2.0,
            pos.y - dest_size.y / 2.0,
            tint,
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
                    texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 1,
                    animation_count: 1,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: 0.0,
                },
            );
        }

        // Alan enemy
        {
            let texture = load_texture("assets/Mini Pixel Pack 3/Enemies/Alan (16 x 16).png")
                .await
                .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::AlanEnemy,
                Sprite {
                    texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 0,
                    animation_count: 6,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: 3.0,
                },
            );
        }

        // BonBon enemy
        {
            let texture = load_texture("assets/Mini Pixel Pack 3/Enemies/Bon_Bon (16 x 16).png")
                .await
                .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::BonBonEnemy,
                Sprite {
                    texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 0,
                    animation_count: 4,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: 4.0,
                },
            );
        }

        // Alan projectile
        {
            let texture =
                load_texture("assets/Mini Pixel Pack 3/Projectiles/Enemy_projectile (16 x 16).png")
                    .await
                    .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::AlanProjectile,
                Sprite {
                    texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 0,
                    animation_count: 4,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: 8.0,
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
                    texture,
                    size: Vec2 { x: 638.0, y: 360.0 },
                    texture_index: 0,
                    animation_count: 9,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: background_fps,
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
                    texture,
                    size: Vec2 { x: 640.0, y: 360.0 },
                    texture_index: 0,
                    animation_count: 9,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: background_fps,
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
                    texture,
                    size: Vec2 { x: 640.0, y: 360.0 },
                    texture_index: 0,
                    animation_count: 9,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: background_fps,
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
                    texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 0,
                    animation_count: 2,
                    size_mult: 1.0,
                    time_scince_frame: 0.0,
                    fps: 10.0,
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
                    texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 0,
                    animation_count: 1,
                    size_mult: 3.0,
                    time_scince_frame: 0.0,
                    fps: 0.0,
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
                    texture,
                    size: Vec2 { x: 38.0, y: 33.0 },
                    texture_index: 0,
                    animation_count: 1,
                    size_mult: 2.0,
                    time_scince_frame: 0.0,
                    fps: 0.0,
                },
            );
        }

        // Start
        {
            let texture = load_texture("assets/Mini Pixel Pack 3/UI objects/START (48 x 8).png")
                .await
                .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::StartUI,
                Sprite {
                    texture,
                    size: Vec2 { x: 48.0, y: 8.0 },
                    texture_index: 0,
                    animation_count: 1,
                    size_mult: 2.0,
                    time_scince_frame: 0.0,
                    fps: 0.0,
                },
            );
        }

        // Game Over
        {
            let texture =
                load_texture("assets/Mini Pixel Pack 3/UI objects/GAME_OVER (72 x 8).png")
                    .await
                    .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::GameOverUI,
                Sprite {
                    texture,
                    size: Vec2 { x: 72.0, y: 8.0 },
                    texture_index: 0,
                    animation_count: 1,
                    size_mult: 2.0,
                    time_scince_frame: 0.0,
                    fps: 0.0,
                },
            );
        }

        // Explosion
        {
            let texture = load_texture("assets/Mini Pixel Pack 3/Effects/Explosion (16 x 16).png")
                .await
                .unwrap();
            texture.set_filter(FilterMode::Nearest);

            self.textures.insert(
                SpriteId::ExplosionVFX,
                Sprite {
                    texture,
                    size: Vec2 { x: 16.0, y: 16.0 },
                    texture_index: 0,
                    animation_count: 4,
                    size_mult: 2.0,
                    time_scince_frame: 0.0,
                    fps: 10.0,
                },
            );
        }
    }
}

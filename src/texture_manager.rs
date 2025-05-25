use std::{collections::HashMap, iter::Filter};

use macroquad::{
    color::WHITE,
    math::{Rect, Vec2},
    text,
    texture::{DrawTextureParams, FilterMode, Texture2D, draw_texture_ex, load_texture},
};

use crate::constants;

#[derive(PartialEq, Eq, Hash)]
pub enum SpriteId {
    Player,
    Asteroid,
    PlayerBooster
}

pub struct Sprite {
    pub texture: Texture2D,
    pub size: Vec2,
    pub texture_index: u32,
    pub size_mult: f32,
}

impl Sprite {
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
                    size_mult: 1.0,
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
                    size_mult: 3.0,
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
                    size_mult: 2.0
                },
            );
        }
    }
}

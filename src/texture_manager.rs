use std::collections::HashMap;

use macroquad::texture::{load_texture, Texture2D};

#[derive(PartialEq, Eq, Hash)]
pub enum TextureId {
    Player,
    Asteroid,
}

pub struct TextureManager {
    pub textures: HashMap<TextureId, Texture2D>,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub async fn load_assets(&mut self) {
        self.textures.insert(TextureId::Player, load_texture("assets/Mini Pixel Pack 3/Player ship/Player_ship (16 x 16).png").await.unwrap());
    }
}

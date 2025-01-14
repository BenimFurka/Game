use std::collections::HashMap;

use macroquad::texture::Texture2D;


#[derive(Clone)]
pub struct TileTextures {
    pub grass: Texture2D,
    pub tilled: Texture2D,
    pub grass_border: Texture2D,
    pub sand: Texture2D,
    pub snow_grass: Texture2D,
    pub water: Texture2D,
    pub custom: HashMap<String, Texture2D>,
}
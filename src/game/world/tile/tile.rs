use std::collections::HashMap;

use macroquad::{math::{vec2, Vec2}, texture::load_texture};

use crate::game::{utils::draw::DrawBatch, world::generator::biome::Biome};

use super::{state::TileState, textures::TileTextures};

pub struct Tile {
    pub state: TileState,
    pub pos: Vec2,
    pub biome: Biome,
    pub size: f32,
    pub textures: Option<TileTextures>,
}

impl Tile {
    pub fn new(x: f32, y: f32, textures: Option<TileTextures>) -> Self {
        Tile {
            state: TileState::Grass,
            biome: Biome::Plains,
            pos: vec2(x, y),
            size: 32.0,
            textures,
        }
    }

    pub async fn load_textures() -> TileTextures {
                    
        let custom = HashMap::new();
        TileTextures {
            
            grass: load_texture("assets/textures/tiles/grass.png").await.unwrap(),
            tilled: load_texture("assets/textures/tiles/tilled.png").await.unwrap(),
            grass_border: load_texture("assets/textures/tiles/grass_border.png").await.unwrap(),
            sand: load_texture("assets/textures/tiles/sand.png").await.unwrap(),
            snow_grass: load_texture("assets/textures/tiles/snow_grass.png").await.unwrap(),
            water: load_texture("assets/textures/tiles/water.png").await.unwrap(),
            custom,
        }
    }

    pub fn draw(&self, tiles: &[Tile], width: usize, batch: &mut DrawBatch) {
        if let Some(textures) = &self.textures {
            let texture = match &self.state {
                TileState::Grass => &textures.grass,
                TileState::Tilled => &textures.tilled,
                TileState::Sand => &textures.sand,
                TileState::SnowGrass => &textures.snow_grass,
                TileState::Water => &textures.water,
                TileState::Custom(id) => {
                    let clean_id = id.trim_start_matches("Custom(").trim_end_matches(")");
                    if let Some(tex) = textures.custom.get(clean_id) {
                        tex
                    } else {
                        &textures.grass
                    }
                },
            };
            
            batch.add(texture.clone(), 
            vec2(self.pos.x, self.pos.y), 
            self.size,
            Some(vec2(self.size, self.size)));
            
            let x = (self.pos.x / self.size) as usize;
            let y = (self.pos.y / self.size) as usize;
            let tile_index = y * width + x;
            
            if y > 0 && tile_index < tiles.len() && tile_index >= width {
                if tiles[tile_index - width].state == TileState::Tilled && self.state == TileState::Grass {
                    batch.add(textures.grass_border.clone(),
                    vec2(self.pos.x, self.pos.y),
                    self.size,
                    Some(vec2(self.size, 4.0)));
                }
            }
        }
    }
}
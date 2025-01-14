use macroquad::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::game::world::{generator::biome::Biome, tile::{state::TileState, tile::Tile}};

use super::vec2::Vec2Save;

#[derive(Serialize, Deserialize, Clone)]
pub struct TileSave {
    pos: Vec2Save,
    biome: Biome,
    state: TileState,
}

impl From<TileSave> for Tile {
    fn from(save: TileSave) -> Self {
        Self {
            pos: Vec2::from(save.pos),
            biome: save.biome,
            state: save.state,
            size: 32.0,
            textures: None,
        }
    }
}

impl From<&Tile> for TileSave {
    fn from(tile: &Tile) -> Self {
        TileSave {
            pos: Vec2Save::from(tile.pos),
            biome: tile.biome.clone(),
            state: tile.state.clone(),
        }
    }
}
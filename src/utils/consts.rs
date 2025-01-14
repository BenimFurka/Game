use lazy_static::lazy_static;
use macroquad::math::Vec2;

use crate::game::world::{generator::biome::Biome, tile::{state::TileState, tile::Tile}};


pub const MAX_CHUNKS: usize = 32;
pub const CHUNK_SIZE: usize = 16;
pub const RENDER_DISTANCE: f32 = 2.0;
pub const WORLD_SIZE: usize = 1024;
pub const MENU_WORLD_SIZE: usize = 48;
pub const TILE_SIZE: f32 = 32.0;
pub const CHUNK_PIXELS: f32 = CHUNK_SIZE as f32 * TILE_SIZE;
pub const BLOCK_CACHE: usize = 32;
lazy_static! {
    pub static ref DEFAULT_TILE: Tile = Tile {
    state: TileState::Grass,
    pos: Vec2::new(0.0, 0.0),
    biome: Biome::Plains,
    size: 32.0,
    textures: None,
    };
}
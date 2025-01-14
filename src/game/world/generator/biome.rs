use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::game::world::tile::state::TileState;

use super::generator::WorldGenerator;

pub struct BiomeCache {
    pub biomes: HashMap<(usize, usize), Biome>,
    pub tile_states: HashMap<Biome, TileState>
}

impl BiomeCache {
    pub fn new() -> Self {
        Self {
            biomes: HashMap::new(),
            tile_states: HashMap::new()
        }
    }

    pub fn get_or_calculate_biome(&mut self, pos: (usize, usize), height: f64, moisture: f64, temperature: f64, generator: &WorldGenerator) -> Biome {
        if let Some(biome) = self.biomes.get(&pos) {
            biome.clone()
        } else {
            let biome = generator.get_biome(height, moisture, temperature);
            self.biomes.insert(pos, biome.clone());
            biome
        }
    }

    pub fn get_or_calculate_tile_state(&mut self, biome: Biome, generator: &WorldGenerator) -> TileState {
        if let Some(state) = self.tile_states.get(&biome) {
            state.clone()
        } else {
            let state = generator.get_tile_state(biome.clone());
            self.tile_states.insert(biome, state.clone());
            state
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Eq, Hash)]
pub enum Biome {
    River,
    Beach,
    Plains,
    Forest,
    SnowPlains,
    SnowForest,
    Desert,
    Custom(String),  
}

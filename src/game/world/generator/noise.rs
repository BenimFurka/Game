use std::collections::HashMap;

use noise::{NoiseFn, Perlin};

pub struct NoiseCache {
    height: HashMap<(usize, usize), f64>,
    moisture: HashMap<(usize, usize), f64>,
    temperature: HashMap<(usize, usize), f64>,
    block_chance: HashMap<(usize, usize), f64>
}

impl NoiseCache {
    pub fn new() -> Self {
        Self {
            height: HashMap::new(),
            moisture: HashMap::new(),
            temperature: HashMap::new(),
            block_chance: HashMap::new()
        }
    }

    pub fn get_or_generate(&mut self, pos: (usize, usize), noise_type: NoiseType, perlin: &Perlin, scale: f64, moisture_scale: f64) -> f64 {
        use NoiseType::*;
        let cache = match noise_type {
            Height => &mut self.height,
            Moisture => &mut self.moisture,
            Temperature => &mut self.temperature,
            BlockChance => &mut self.block_chance,
        };
        
        *cache.entry(pos).or_insert_with(|| {
            let (x, y) = (pos.0 as f64, pos.1 as f64);
            match noise_type {
                Height => perlin.get([x * scale, y * scale]),
                Moisture => perlin.get([x * moisture_scale + 1000.0, y * moisture_scale + 1000.0]),
                Temperature => perlin.get([x * 0.02 + 2000.0, y * 0.02 + 2000.0]),
                BlockChance => perlin.get([x * 0.5 + 4000.0, y * 0.5 + 4000.0]),
            }
        })
    }
}
pub enum NoiseType {
    Height,
    Moisture,
    Temperature,
    BlockChance
}
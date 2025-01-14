
use std::path::{Path, PathBuf};
use std::fs;
use serde_json;
use serde::Deserialize;
use game_core::*;

pub struct ModLoader {
    mods: Vec<ModConfig>,
    mods_path: PathBuf,
    biomes: Vec<BiomeMod>,
    tiles: Vec<TileMod>,
}

impl ModLoader {
    pub fn new() -> Self {
        Self {
            mods: Vec::new(),
            mods_path: PathBuf::from("mods"),
            biomes: Vec::new(),
            tiles: Vec::new()
        }
    }

    pub fn load_mods(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.mods.clear();
        self.biomes.clear();

        if !self.mods_path.exists() {
            fs::create_dir(&self.mods_path)?;
        }

        for entry in fs::read_dir(&self.mods_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Err(e) = self.load_mod(&path) {
                    println!("Failed to load mod at {:?}: {}", path, e);
                }
            }
        }

        println!("Loaded {} mods with {} biomes", self.mods.len(), self.biomes.len());
        Ok(())
    }

    fn load_mod(&mut self, mod_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json_path = mod_path.join("mod.json");
        let json_content = fs::read_to_string(json_path)?;
        
        #[derive(Deserialize)]
        struct ModData {
            #[serde(flatten)]
            config: ModConfig,
            biomes: Option<Vec<BiomeMod>>,
            tiles: Option<Vec<TileMod>>,
        }
        
        let mod_data: ModData = serde_json::from_str(&json_content)?;
        
        println!("Loading mod: {} ({})", mod_data.config.name, mod_data.config.id);
        
        if let Some(mod_tiles) = mod_data.tiles {
            for tile in mod_tiles {
                println!("Loading tile: {}", tile.name);
                self.tiles.push(tile);
            }
        }
        
        if let Some(mod_biomes) = mod_data.biomes {
            for biome in mod_biomes {
                println!("Loading biome: {}", biome.name);
                self.biomes.push(biome);
            }
        }
        
        self.mods.push(mod_data.config);
        Ok(())
    }
        
    pub fn get_biomes(&self) -> &[BiomeMod] {
        &self.biomes
    }
    
    pub fn get_tiles(&self) -> &[TileMod] {
        &self.tiles
    }
}
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BiomeMod {
    pub id: String,
    pub name: String,
    pub conditions: BiomeConditions,
    pub tile_state: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BiomeConditions {
    pub height_range: Option<(f64, f64)>,
    pub moisture_range: Option<(f64, f64)>,
    pub temperature_range: Option<(f64, f64)>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct TileMod {
    pub id: String,
    pub name: String,
    pub texture_path: String,
}   

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub enum TileState {
    Grass,
    Tilled,
    Sand,
    SnowGrass,
    Water,
    Custom(String),
}
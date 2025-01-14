use serde::Deserialize;

pub mod mod_content; 
pub use mod_content::*; 

#[derive(Debug, Deserialize)]
pub struct ModConfig {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: Option<String>,
}

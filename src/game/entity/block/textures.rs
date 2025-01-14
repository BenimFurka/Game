use macroquad::texture::{load_texture, Texture2D};

pub struct BlockTextures {
    pub tree: Texture2D,
    pub tree_top: Texture2D,
    pub tree_snow_top: Texture2D,
    pub cactus: Texture2D,
}
    
impl BlockTextures {
    pub async fn load() -> Self {
        Self {
            tree: load_texture("assets/textures/blocks/tree.png").await.unwrap(),
            tree_top: load_texture("assets/textures/blocks/tree_top.png").await.unwrap(),
            tree_snow_top: load_texture("assets/textures/blocks/tree_snow_top.png").await.unwrap(),
            cactus: load_texture("assets/textures/blocks/cactus.png").await.unwrap(),
        }
    }
}
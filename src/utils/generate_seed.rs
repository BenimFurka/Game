
use std::time::{SystemTime, UNIX_EPOCH};
pub fn generate_seed() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}
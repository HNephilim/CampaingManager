
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone)]
pub enum TraitType {
    Alignment(String),
    Rarity(String),
    Size(String),
    Other(String),
}

impl TraitType {
    pub const COLOR_ALIGNMENT: [f32; 4] = [0.245, 0.398, 0.665, 1.];
    pub const COLOR_RARITY: [f32; 4] = [0.5, 0., 0.46, 1.];
    pub const COLOR_SIZE: [f32; 4] = [0.202, 0.492, 0.204, 1.];
    pub const COLOR_OTHER: [f32; 4] = [0.237, 0.130, 0.130, 1.];
    
    pub fn get(&self) -> &String{
        match self{
            TraitType::Alignment(str) => str,
            TraitType::Rarity(str) => str,
            TraitType::Size(str) => str,
            TraitType::Other(str) => str,
        }
    }
}
use anyhow::*;
use serde_json::Value;


use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct AbilityScore {
    pub strenght: i8,
    pub dexterity: i8,
    pub constituition: i8,
    pub inteligency: i8,
    pub wisdom: i8,
    pub charisma: i8,
}

impl AbilityScore {
    pub fn new(value: &Value) -> Result<Self> {
        let strenght = value["data"]["abilities"]["str"]["mod"]
            .as_i64()
            .context("Failed to get => STR")? as i8;
        let dexterity = value["data"]["abilities"]["dex"]["mod"]
            .as_i64()
            .context("Failed to get => DEX")? as i8;
        let constituition = value["data"]["abilities"]["con"]["mod"]
            .as_i64()
            .context("Failed to get => CON")? as i8;
        let inteligency = value["data"]["abilities"]["int"]["mod"]
            .as_i64()
            .context("Failed to get => INT")? as i8;
        let wisdom = value["data"]["abilities"]["wis"]["mod"]
            .as_i64()
            .context("Failed to get => WIS")? as i8;
        let charisma = value["data"]["abilities"]["cha"]["mod"]
            .as_i64()
            .context("Failed to get => CHA")? as i8;

        Ok(Self {
            strenght,
            dexterity,
            constituition,
            inteligency,
            wisdom,
            charisma,
        })
    }
}


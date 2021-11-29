use parser::UiInstruction;
use serde_json::Value;
use anyhow::*;



use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Treasure{
    pub name: String,
    pub description_with_level: Vec<UiInstruction>,
    pub description_wout_level: Vec<UiInstruction>,
    pub quantity: i16,
    pub value: i32,
}

impl Treasure{
    pub fn new(value: &Value) -> Result<Self> {
        let name = value["name"].as_str().context("Failed to get => Treasure Name")?.to_string();

        let desc_vec = parser::parse(value["data"]["description"]["value"].as_str().context("Failed to get => Treasure Description")?.to_string(), 0);

        let quantity = value["data"]["quantity"]["value"].as_i64().context("Failed to get => Treasure Quantity")? as i16;
        let value = value["data"]["value"]["value"].as_i64().context("Failed to get => Treasure Value")? as i32;


        Ok(Self{
            name,
            description_with_level: desc_vec.0,
            description_wout_level: desc_vec.1,
            quantity,
            value,
        })
    }
}

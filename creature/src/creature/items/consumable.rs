use parser::UiInstruction;
use serde_json::Value;
use anyhow::*;



use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Consumable{
    pub name: String,
    pub description_with_level: Vec<UiInstruction>,
    pub description_wout_level: Vec<UiInstruction>,
    pub quantity: i16,
}

impl Consumable{
    pub fn new(value: &Value) -> Result<Self> {
        let name = value["name"].as_str().context("Failed to get => Consumable Name")?.to_string();
        let desc = parser::parse(value["data"]["description"]["value"].as_str().context("Failed to get => Consumable Description")?.to_string(), 0);
        let quantity = value["data"]["quantity"]["value"].as_i64().context("Failed to get => Consumable Quantity")? as i16;


        Ok(Self{
            name,
            description_with_level: desc.0,
            description_wout_level: desc.1,
            quantity,
        })
    }
}

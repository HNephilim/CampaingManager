use serde_json::Value;
use anyhow::*;

use serde::{Serialize, Deserialize};

use crate::helper_functions::str_to_option_string;

#[derive(Serialize, Deserialize)]
pub struct Weapon{
    pub name: String,
    pub potency: i8,
    pub striking: Option<String>,
    pub quantity: i8,
    pub property_1: Option<String>,
    pub property_2: Option<String>,
    pub property_3: Option<String>,
    pub property_4: Option<String>,
}

impl Weapon{
    pub fn new(value: &Value) -> Result<Self> {
        let name = value["name"].as_str().context("Failed to get => Weapon Name")?.to_string();
        let potency = value["data"]["potencyRune"]["value"].as_i64().or(Some(0)).context("Failed to get => Weapon Potency Rune")? as i8;
        let striking = str_to_option_string(value["data"]["strikingRune"]["value"].as_str().context("Failed to get => Weapon Strinking Rune")?);
        let quantity = value["data"]["quantity"]["value"].as_i64().context("Failed to get => Weapon Quantity")? as i8;
        let property_1 = str_to_option_string(value["data"]["propertyRune1"]["value"].as_str().or(Some("")).context("Failed to get => Weapon Property Rune 1")?);
        let property_2 = str_to_option_string(value["data"]["propertyRune2"]["value"].as_str().or(Some("")).context("Failed to get => Weapon Property Rune 2")?);
        let property_3 = str_to_option_string(value["data"]["propertyRune3"]["value"].as_str().or(Some("")).context("Failed to get => Weapon Property Rune 3")?);
        let property_4 = str_to_option_string(value["data"]["propertyRune4"]["value"].as_str().or(Some("")).context("Failed to get => Weapon Property Rune 4")?);
     
        
        
        
        
        
        
        Ok(Self{
            name,
            potency,
            striking,
            quantity,
            property_1,
            property_2,
            property_3,
            property_4,
        })
    }
}

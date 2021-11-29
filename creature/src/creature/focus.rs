use serde_json::Value;
use anyhow::*;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Focus{
    pub max: i8,
    pub actual: i8
}

impl Focus{
    pub fn new(value: &Value) -> Result<Self> {
        let max = value["data"]["resources"]["max"].as_i64().or(Some(0)).context("Failed to get => Resources Max")? as i8;
        let actual = value["data"]["resources"]["value"].as_i64().or(Some(0)).context("Failed to get => Resources Max")? as i8;

        Ok(Self{
            max,
            actual,
        })

    }
}
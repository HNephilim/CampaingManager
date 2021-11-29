use crate::{helper_functions::str_to_option_string};
use anyhow::*;
use serde_json::Value;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Hp {
    pub modifier: u8,
    pub desc: Option<String>,
}

impl Hp {
    pub fn new(value: &Value) -> Result<Self> {
        let modifier = value["data"]["attributes"]["hp"]["max"]
            .as_i64()
            .context("Failed to get => HP Mod")? as u8;
        let desc = str_to_option_string(
            value["data"]["attributes"]["hp"]["details"]
                .as_str()
                .context("Failed to get => HP Desc")?,
        );

        Ok(Self { modifier, desc })
    }
}


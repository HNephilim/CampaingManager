use serde_json::Value;
use anyhow::*;
use crate::{helper_functions::str_to_option_string};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Ac{
    pub modifier: u8,
    pub desc: Option<String>
}

impl Ac{
    pub fn new(value: &Value) -> Result<Self>{
        let modifier = value["data"]["attributes"]["ac"]["value"].as_i64().context("Failed to get => Ac Mod")? as u8;
        let desc = str_to_option_string(value["data"]["attributes"]["ac"]["details"].as_str().context("Failed to get => Ac Desc")?);

        Ok(Self{
            modifier,
            desc,
        })
    }
}

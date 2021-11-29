use crate::{helper_functions::str_to_option_string};
use anyhow::*;
use serde_json::Value;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Saves {
    pub fortitude: i8,
    pub fortitude_desc: Option<String>,
    pub reflex: i8,
    pub reflex_desc: Option<String>,
    pub will: i8,
    pub will_desc: Option<String>,
    pub all_desc: Option<String>,
}

impl Saves {
    pub fn new(value: &Value) -> Result<Self> {
        let fortitude = value["data"]["saves"]["fortitude"]["value"]
            .as_i64()
            .context("Failed to get => Fort Val")? as i8;
        let fortitude_desc = str_to_option_string(
            value["data"]["saves"]["fortitude"]["saveDetail"]
                .as_str()
                .context("Failed to get => Fort Desc")?,
        );
        let reflex = value["data"]["saves"]["reflex"]["value"]
            .as_i64()
            .context("Failed to get => Ref Val")? as i8;
        let reflex_desc = str_to_option_string(
            value["data"]["saves"]["reflex"]["saveDetail"]
                .as_str()
                .context("Failed to get => Ref Des")?,
        );
        let will = value["data"]["saves"]["will"]["value"]
            .as_i64()
            .context("Failed to get => Will Val")? as i8;
        let will_desc = str_to_option_string(
            value["data"]["saves"]["will"]["saveDetail"]
                .as_str()
                .context("Failed to get => Will Dex")?,
        );
        let all_desc = str_to_option_string(
            value["data"]["attributes"]["allSaves"]["value"]
                .as_str()
                .or(Some(""))
                .context("Failed to get => All Save Desc")?,
        );

        Ok(Self {
            fortitude,
            fortitude_desc,
            reflex,
            reflex_desc,
            will,
            will_desc,
            all_desc,
        })
    }
}

use anyhow::*;
use serde_json::Value;


use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct DamageModifier {
    pub resistances: Vec<DmgMod>,
    pub weaknesses: Vec<DmgMod>,
    pub immunities: Vec<String>,
}
#[derive(Serialize, Deserialize)]
pub struct DmgMod {
    pub label: String,
    pub value: Option<i8>,
    pub exception: Option<String>,
}

impl DamageModifier {

    pub fn has_something(&self) -> bool{
        if self.resistances.len() > 0 || self.weaknesses.len() > 0 || self.immunities.len() > 0{
            true
        }else{
            false
        }
    }

    pub fn new(value: &Value) -> Result<Self> {
        let resistances = DmgMod::new(&value["data"]["traits"]["dr"])?;
        let weaknesses = DmgMod::new(&value["data"]["traits"]["dv"])?;
        let immunities = DamageModifier::build_immunities(&value["data"]["traits"]["di"])?;

        Ok(Self {
            resistances,
            weaknesses,
            immunities,
        })
    }

    fn build_immunities(value: &Value) -> Result<Vec<String>> {
        let mut return_vec = Vec::new();

        if ""
            != value["custom"]
                .as_str()
                .context("Failed to get => Custom Immunities")?
        {
            let custom = value["custom"]
                .as_str()
                .context("Failed to get => Custom Immunities")?
                .to_string();
            return_vec.push(custom);
        }

        for str in value["value"]
            .as_str()
            .context("Failed to get => Immunities Value")
        {
            return_vec.push(str.to_string());
        }

        Ok(return_vec)
    }
}


impl DmgMod {
    fn new(value: &Value) -> Result<Vec<Self>> {
        let mut return_vec = Vec::new();
        for value in value
            .as_array()
            .expect("Failed to get => Resistances (dr) of Weaknesses (dv)")
        {
            let label = value["label"]
                .as_str()
                .or(value["type"].as_str())
                .context("Failed to get => Label of Res or Weak")?
                .to_string();
            let exception = match value["exceptions"].as_str() {
                Some(str) => Some(str.to_string()),
                None => None,
            };
            let value = if value["value"].is_i64() {
                Some(
                    value["value"]
                        .as_i64()
                        .context("Failed to get => Value of Res or Weak")?
                        as i8,
                )
            } else if value["value"].is_string() {
                Some(
                    value["value"]
                        .as_str()
                        .context("Failed to get => Value of Res or Weak from string")?
                        .parse::<i8>()?,
                )
            } else {
                None
            };

            let dmg_mod = Self {
                label,
                value,
                exception,
            };

            return_vec.push(dmg_mod);
        }

        Ok(return_vec)
    }
}



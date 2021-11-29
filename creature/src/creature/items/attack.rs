use serde_json::Value;
use anyhow::*;

use serde::{Serialize, Deserialize};

use crate::helper_functions::str_to_option_string;

#[derive(Serialize, Deserialize)]
pub struct Attack{
    pub name: String,
    pub range: WeaponType,
    pub bonus: i8,
    pub traits: Vec<String>,
    pub damage: Vec<Damage>,
    pub attack_fx: Vec<String>
}


#[derive(Serialize, Deserialize)]
pub struct Damage{
    pub damage: Option<String>,
    pub damage_type: Option<String>
}


#[derive(Serialize, Deserialize)]
pub enum WeaponType {
    Melee,
    Ranged
}

impl Attack{
    pub fn new(value: &Value) -> Result<Self> {
        let name = value["name"].as_str().context("Failed to get => Melee Name")?.to_string();
        let range = WeaponType::new(value["data"]["weaponType"]["value"].as_str().context("Failed to get => Melee Weapon Type")?)?;
        let bonus = value["data"]["bonus"]["value"].as_i64().context("Failed to get => Melee Weapon Bonus")? as i8;

        let mut traits = Vec::new();
        if "" != value["data"]["traits"]["custom"].as_str().context("Failed to get => Melee Weapon Trait Custom")?{
            traits.push(value["data"]["traits"]["custom"].as_str().context("Failed to get => Melee Weapon Trait Custom")?.to_string());
        }        
        for val in value["data"]["traits"]["value"].as_array().context("Failed to get => Melee Weapon Trait Array")?{
            traits.push(val.as_str().context("Failed to get => Melee Weapon Trait")?.to_string());
        }

        let mut damage = Vec::new();
        for (_, val) in value["data"]["damageRolls"].as_object().context("Failed to get => Melee Weapon Damage Object")?{
            damage.push(Damage{
                damage: str_to_option_string(val["damage"].as_str().context("Failed to get => Melee Weapon Damage")?),
                damage_type: str_to_option_string(val["damageType"].as_str().context("Failed to get => Melee Weapon Damage Type")?),
            })
        }

        let mut attack_fx = Vec::new();
        let attackfx_array = value["data"]["attackEffects"]["value"].as_array().context("Failed to get => Melee Weapon FX Array")?;
        if attackfx_array.len() > 0{

            if "" != value["data"]["attackEffects"]["custom"].as_str().or(Some("")).context("Failed to get => Melee Weapon FX Custom")?{
                attack_fx.push(value["data"]["attackEffects"]["custom"].as_str().or(Some("")).context("Failed to get => Melee Weapon FX Custom")?.to_string());
            }
            for val in attackfx_array{
                attack_fx.push(val.as_str().context("Failed to get => Melee Weapon FX")?.to_string());
            }
        }

        


        Ok(Self{
            name,
            range,
            bonus,
            traits,
            damage,
            attack_fx,
        })
    }
}

impl WeaponType{
    fn new(range: &str) -> Result<Self>{
        match range {
            "melee" => Ok(WeaponType::Melee),
            "ranged" => Ok(WeaponType::Ranged),
            _ => Err(anyhow!("Failed to get => Melee Range"))
        }
    }
}




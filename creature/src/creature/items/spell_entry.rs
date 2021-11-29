use serde_json::Value;
use anyhow::*;

use super::Spell;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct SpellEntry{
    pub name: String,
    pub id: String,
    pub dc: i8, 
    pub modifier: i8,
    pub spell_lv0: Vec<Spell>,
    pub spell_lv1: Vec<Spell>,
    pub spell_lv2: Vec<Spell>,
    pub spell_lv3: Vec<Spell>,
    pub spell_lv4: Vec<Spell>,
    pub spell_lv5: Vec<Spell>,
    pub spell_lv6: Vec<Spell>,
    pub spell_lv7: Vec<Spell>,
    pub spell_lv8: Vec<Spell>,
    pub spell_lv9: Vec<Spell>,
    pub spell_lv10: Vec<Spell>,
    pub spell_focus: Vec<Spell>
}

impl SpellEntry{    

    pub fn new(value: &Value) -> Result<Self> {
        let name = value["name"].as_str().context("Failed to get => SpellcastEntry Name")?.to_string();
        let id = value["_id"].as_str().context("Failed to get => SpellcastEntry Id")?.to_string();

        let dc_string = value["data"]["spelldc"]["dc"].as_str();
        let dc = match dc_string{
            Some(str) => str.parse::<i64>()?,
            None => value["data"]["spelldc"]["dc"].as_i64().context("Failed to get => SpellcastEntry dc")?,
        } as i8;
        
        let modifier = value["data"]["spelldc"]["value"].as_i64().context("Failed to get => SpellEntry Modifier")? as i8;
        
        

        Ok(Self{
            name,
            id,
            dc,
            modifier,
            spell_lv0: Vec::new(),
            spell_lv1: Vec::new(),
            spell_lv2: Vec::new(),
            spell_lv3: Vec::new(),
            spell_lv4: Vec::new(),
            spell_lv5: Vec::new(),
            spell_lv6: Vec::new(),
            spell_lv7: Vec::new(),
            spell_lv8: Vec::new(),
            spell_lv9: Vec::new(),
            spell_lv10: Vec::new(),
            spell_focus: Vec::new(),
        })
    }
}
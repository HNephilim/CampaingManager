use serde_json::Value;
use anyhow::*;


use serde::{Serialize, Deserialize};

use crate::helper_functions::str_to_option_string;

use super::ActionValue;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ActionCategory{
    None,
    Offensive(Action),
    Defensive(Action),
    Interaction(Action),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Action{
    pub name: String,
    pub traits: Vec<String>,    
    pub action_type: ActionValue,    
    pub desc: Option<String>,
}



impl ActionCategory{
    pub fn new(value: &Value) -> Result<Self> {

        let action = Action::new(value)?;

        let action_category = match value["data"]["actionCategory"]["value"].as_str().context("Failed to get => Action Category")?{
            "defensive" =>{
                ActionCategory::Defensive(action)
            }
            "offensive" =>{
                ActionCategory::Offensive(action)
            }
            "interaction" =>{
                ActionCategory::Interaction(action)
            }
            _ =>{
                ActionCategory::None
            }
        };

        Ok(action_category)
    }
}

impl Action{
    pub fn new(value: &Value) -> Result<Self> {
        let name = value["name"].as_str().context("Failed to get => Action Name")?.to_string();
        
        let mut traits = Vec::new();
        if "" != value["data"]["traits"]["custom"].as_str().context("Failed to get => Action Custom Trait")?{
            traits.push(value["data"]["traits"]["custom"].as_str().context("Failed to get => Action Custom Trait")?.to_string());
        }    
        for val in value["data"]["traits"]["value"].as_array().context("Failed to get => Action Trait Array")?{
            if !val.is_null(){
                traits.push(val.as_str().context("Failed to get => Action Trait value")?.to_string());
            }
            else{
                break;
            }
        }        

        let action_type = match value["data"]["actionType"]["value"].as_str().context("Failed to get => Action Type")?{
            "action" => {
                let action_num =  value["data"]["actions"]["value"].as_i64().context("Failed to get => Action Num").unwrap_or(0) as i8;
                
                match action_num {
                    1 => ActionValue::OneAction,
                    2 => ActionValue::TwoActions,
                    3 => ActionValue::ThreeActions,
                    _ => ActionValue::Other("Invalid Input".to_string()),

                }
            }
            "reaction" => {
                ActionValue::Reaction
            }
            "free" => {
                ActionValue::FreeAction
            }
            "passive" => {
                ActionValue::Passive
            }
            _ => {
                return Err(anyhow!("Failed to get => Action Type"));
            }
        };        

        let desc  = str_to_option_string(value["data"]["description"]["value"].as_str().context("Failed to get => Action Description")?);

        Ok(Self{
            name,
            traits,
            action_type,
            desc,
        })

    }
}

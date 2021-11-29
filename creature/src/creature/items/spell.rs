use anyhow::*;
use parser::UiInstruction;
use serde_json::Value;

use crate::{creature::trait_type::TraitType, helper_functions::option_str_to_option_string};

use super::ActionValue;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Spell {
    pub name: String,
    pub location: String,
    pub traits: Vec<TraitType>,
    pub level: u8,
    pub description_with_level: Vec<UiInstruction>,
    pub description_wout_level: Vec<UiInstruction>,
    pub action_value: ActionValue,
    pub category: SpellCategory,
    pub components: SpellComponents,
    pub duration: Option<String>,
    pub materials: Option<String>,
    pub area: Option<String>,
    pub target: Option<String>,
    pub range: Option<String>,
    pub save: Option<SaveOption>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SpellCategory {
    Focus,
    Spell,
    Ritual,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct SpellComponents {
    pub focus: bool,
    pub material: bool,
    pub somatic: bool,
    pub verbal: bool,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum SaveOption {
    Fortitude(bool),
    Reflex(bool),
    Will(bool),
}

impl Spell {
    pub fn new(value: &Value) -> Result<Self> {
        let name = value["name"]
            .as_str()
            .context("Failed to get => Spell Name")?
            .trim_end_matches(" - Cantrips")
            .trim_end_matches(" - Focus")
            .to_string();
        let location = value["data"]["location"]["value"]
            .as_str()
            .context("Failed to get => Spell Location")?
            .to_string();

        let mut traits = Vec::new();

        traits.push(TraitType::Other(
            value["data"]["school"]["value"]
                .as_str()
                .context("Failed to get => Spell Location")?
                .to_ascii_uppercase(),
        ));

        if ""
            == value["data"]["traits"]["custom"]
                .as_str()
                .context("Failed to get => Spell Custom Trait")?
        {
            traits.push(TraitType::Other(
                value["data"]["traits"]["custom"]
                    .as_str()
                    .context("Failed to get => Spell Custom Trait")?
                    .to_ascii_uppercase(),
            ));
        }

        for val in value["data"]["traits"]["value"]
            .as_array()
            .context("Failed to get => Spell Trait Array")?
        {
            if !val.is_null() {
                traits.push(TraitType::Other(
                    val.as_str()
                        .context("Failed to get => Spell Trait value")?
                        .to_ascii_uppercase(),
                ));
            } else {
                break;
            }
        }

        let level = value["data"]["level"]["value"]
            .as_i64()
            .context("Failed to get => Spell Level")? as u8;

        let description = parser::parse(
            value["data"]["description"]["value"]
                .as_str()
                .context("Failed to get => Spell Description")?
                .to_string(),
            level,
        );

        let action_value = match value["data"]["time"]["value"]
            .as_str()
            .context("Failed to get => Spell Cast Time")?
        {
            "1" => ActionValue::OneAction,
            "2" => ActionValue::TwoActions,
            "3" => ActionValue::ThreeActions,
            "free" => ActionValue::FreeAction,
            "reaction" => ActionValue::Reaction,
            "1 or 2" => ActionValue::OneOrTwoActions,
            "1 to 3" => ActionValue::OneToThreeAction,
            str => ActionValue::Other(str.to_string()),
        };

        let category = match value["data"]["category"]["value"]
            .as_str()
            .context("Failed to get => Spell Category")?
        {
            "focus" => SpellCategory::Focus,
            "spell" => SpellCategory::Spell,
            "ritual" => SpellCategory::Ritual,
            _ => {
                return Err(anyhow!(
                    "Failed to get => Spell Category. Another option found"
                ))
            }
        };

        let components = SpellComponents {
            focus: value["data"]["components"]["focus"]
                .as_bool()
                .unwrap_or(false),
            material: value["data"]["components"]["material"]
                .as_bool()
                .unwrap_or(false),
            somatic: value["data"]["components"]["somatic"]
                .as_bool()
                .unwrap_or(false),
            verbal: value["data"]["components"]["verbal"]
                .as_bool()
                .unwrap_or(false),
        };

        let duration = option_str_to_option_string(value["data"]["duration"]["value"].as_str());

        let materials = option_str_to_option_string(value["data"]["materials"]["value"].as_str());

        let area = option_str_to_option_string(value["data"]["areasize"]["value"].as_str());

        let target = option_str_to_option_string(value["data"]["target"]["value"].as_str());

        let range = option_str_to_option_string(value["data"]["range"]["value"].as_str());

        let basic = match value["data"]["save"]["basic"]
            .as_str()
            .context("Failed to get => Spell Save Basic bool")?
        {
            "" => false,
            _ => true,
        };

        let save = match value["data"]["save"]["value"]
            .as_str()
            .context("Failed to get => Spell Save Value")?
        {
            "will" => Some(SaveOption::Will(basic)),
            "fortitude" => Some(SaveOption::Fortitude(basic)),
            "reflex" => Some(SaveOption::Reflex(basic)),
            _ => None,
        };

        Ok(Self {
            name,
            location,
            traits,
            level,
            description_with_level: description.0,
            description_wout_level: description.1,
            action_value,
            category,
            components,
            duration,
            materials,
            area,
            target,
            range,
            save,
        })
    }
}

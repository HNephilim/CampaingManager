use anyhow::*;
use serde_json::Value;


use serde::{Serialize, Deserialize};

use crate::helper_functions::str_to_option_string;
#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub skill: SkillType,
    pub modifier: i8,
    pub desc: Option<String>,
    pub variants: Option<Vec<String>>,
}


#[derive(Serialize, Deserialize)]
pub enum SkillType {
    None,
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Lore(String),
    Medicine,
    Nature,
    Occultism,
    Perception,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery,
}

impl Skill {
    pub fn new(value: &Value) -> Result<Self> {
        let skill = SkillType::new(
            value["name"]
                .as_str()
                .context("Failed to get => Skill Name")?,
        )?;
        let modifier = value["data"]["mod"]["value"]
            .as_i64()
            .context("Failed to get => Skill Mod")? as i8;

        let desc = str_to_option_string(value["data"]["description"]["value"].as_str().context("Failed to get => Skill Desc")?);

        let variants = if value["data"]["variants"].is_object() {
            let mut variant_labels = Vec::new();
            for (_, value) in value["data"]["variants"].as_object().unwrap() {
                variant_labels.push(
                    value["label"]
                        .as_str()
                        .context("Failed to get => Skill Label")?
                        .to_string(),
                )
            }
            Some(variant_labels)
        } else {
            None
        };

        Ok(Self {
            skill,
            modifier,
            desc,
            variants,
        })
    }
}


impl SkillType {
    fn new(skill: &str) -> Result<Self> {
        let skill = match skill {
            "Acrobatics" => Self::Acrobatics,
            "Arcana" => Self::Arcana,
            "Athletics" => Self::Athletics,
            "Crafting" => Self::Crafting,
            "Deception" => Self::Deception,
            "Diplomacy" => Self::Diplomacy,
            "Intimidation" => Self::Intimidation,
            "Medicine" => Self::Medicine,
            "Nature" => Self::Nature,
            "Occultism" => Self::Occultism,
            "Perception" => Self::Perception,
            "Performance" => Self::Performance,
            "Religion" => Self::Religion,
            "Society" => Self::Society,
            "Stealth" => Self::Stealth,
            "Survival" => Self::Survival,
            "Thievery" => Self::Thievery,
            lore => {
                if lore.contains("Lore") {
                    let lore_type = lore
                        .split(' ')
                        .next()
                        .context("Failed to get => Lore Type")?
                        .to_string();
                    Self::Lore(lore_type)
                } else {
                    Self::None
                }
            }
        };

        match skill {
            Self::None => Err(anyhow!("Failed to get => Skill")),
            _ => Ok(skill),
        }
    }

    pub fn to_str<'skill>(&'skill self) -> &'skill str{
        match self{
            SkillType::None => "",
            SkillType::Acrobatics => "Acrobatics",
            SkillType::Arcana => "Arcana",
            SkillType::Athletics => "Athletics",
            SkillType::Crafting => "Crafting",
            SkillType::Deception => "Deception",
            SkillType::Diplomacy => "Diplomacy",
            SkillType::Intimidation => "Intimidation",
            SkillType::Medicine => "Medicine",
            SkillType::Nature => "Nature",
            SkillType::Occultism => "Occultism",
            SkillType::Perception => "Perception",
            SkillType::Performance => "Performance",
            SkillType::Religion => "Religion",
            SkillType::Society => "Society",
            SkillType::Stealth => "Stealth",
            SkillType::Survival => "Survival",
            SkillType::Thievery => "Thievery",
            SkillType::Lore(lore) => {
                lore.as_str()
            },
        }
    }
}





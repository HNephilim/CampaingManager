use anyhow::*;
use serde_json::Value;

mod trait_type;
pub use trait_type::TraitType;

mod ability_scores;
pub use ability_scores::AbilityScore;

mod saves;
pub use saves::Saves;

mod ac;
pub use ac::Ac;

mod hp;
pub use hp::Hp;

mod damage_mod;
pub use damage_mod::{DamageModifier, DmgMod};

mod focus;
pub use focus::Focus;

pub mod items;
use self::items::{
    Action, ActionCategory, Armor, Attack, Consumable, Damage, Equipment, Items, Skill, Spell,
    SpellEntry, Treasure, Weapon, WeaponType,
};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Creature {
    pub name: String,
    pub level: i8,
    pub perception: i8,
    pub creature_type: String,
    pub traits: Vec<TraitType>,
    pub ability_scores: AbilityScore,
    pub languages: Vec<String>,
    pub saves: Saves,
    pub ac: Ac,
    pub hp: Hp,
    pub damage_mod: DamageModifier,
    pub senses: Option<String>,
    pub focus: Focus,
    pub speeds: Vec<String>,
    pub items: Vec<Items>,
    pub skills: Vec<Skill>,
    pub spellcast_entry: Vec<SpellEntry>,
    pub spells: Vec<Spell>,
    pub attacks: Vec<Attack>,
    pub actions: Vec<ActionCategory>,
}

impl Creature {
    pub fn new(value: &Value) -> Result<Self> {
        let name = value["name"]
            .as_str()
            .context("Failed to get => Name")?
            .to_string();

        let level = value["data"]["details"]["level"]["value"]
            .as_i64()
            .context("Failed to get => Level")? as i8;

        let perception = value["data"]["attributes"]["perception"]["value"]
            .as_i64()
            .context("Failed to get => Perception")? as i8;

        let creature_type = value["data"]["details"]["creatureType"]
            .as_str()
            .context("Failed to get => Creature_Type")?
            .to_string();

        let languages = Creature::build_languages(value)?;

        let traits = Creature::build_traits(value)?;

        let ability_scores = AbilityScore::new(value)?;

        let saves = Saves::new(value)?;

        let ac = Ac::new(value)?;

        let hp = Hp::new(value)?;

        let damage_mod = DamageModifier::new(value)?;

        let senses = match value["data"]["traits"]["senses"]["value"].as_str() {
            Some(str) => Some(str.to_string()),
            None => None,
        };

        let focus = Focus::new(value)?;

        let mut speeds = Vec::new();
        speeds.push(
            value["data"]["attributes"]["speed"]["value"]
                .as_str()
                .context("Failed to get => Speed Value")?
                .to_string(),
        );
        for value in value["data"]["attributes"]["speed"]["otherSpeeds"]
            .as_array()
            .context("Failed to get => Other Speeds Array")?
        {
            let mut speed_type = value["type"]
                .as_str()
                .context("Failed to get => Other Speed Type")?
                .to_string();
            speed_type.push(' ');
            speed_type.push_str(
                value["value"]
                    .as_str()
                    .context("Failed to get => Other Speed Type")?,
            );
            speeds.push(speed_type);
        }

        let mut items = Vec::new();
        let mut skills = Vec::new();
        let mut spellcast_entry = Vec::new();
        let mut spells = Vec::new();
        let mut attacks = Vec::new();
        let mut actions = Vec::new();

        for value in value["items"]
            .as_array()
            .context("Failed to get => Items Array")?
        {
            match value["type"]
                .as_str()
                .context("Failed to get => Item Type")?
            {
                "action" => actions.push(ActionCategory::new(value)?),
                "armor" => items.push(Items::Armor(Armor::new(value)?)),
                "consumable" => items.push(Items::Consumable(Consumable::new(value)?)),
                "equipment" => items.push(Items::Equipment(Equipment::new(value)?)),
                "lore" => skills.push(Skill::new(value)?),
                "melee" => attacks.push(Attack::new(value)?),
                "spell" => spells.push(Spell::new(value)?),
                "spellcastingEntry" => spellcast_entry.push(SpellEntry::new(value)?),
                "treasure" => items.push(Items::Treasure(Treasure::new(value)?)),
                "weapon" => items.push(Items::Weapon(Weapon::new(value)?)),
                _ => {}
            }
        }

        let mut creature = Self {
            name,
            level,
            perception,
            creature_type,
            traits,
            ability_scores,
            saves,
            ac,
            hp,
            damage_mod,
            senses,
            focus,
            speeds,
            items,
            skills,
            spellcast_entry,
            spells,
            attacks,
            actions,
            languages,
        };

        creature.insert_spells();

        Ok(creature)
    }

    fn insert_spells(&mut self) {
        for spell in self.spells.iter() {
            for spellcast_entry in self.spellcast_entry.iter_mut() {
                if spell.location == spellcast_entry.id {
                    match spell.level {
                        1 => {
                            if spell
                                .traits
                                .iter()
                                .any(|trt| "CANTRIP" == trt.get().as_str())
                            {
                                spellcast_entry.spell_lv0.push(spell.clone())
                            } else {
                                spellcast_entry.spell_lv1.push(spell.clone())
                            }
                        }
                        2 => spellcast_entry.spell_lv2.push(spell.clone()),
                        3 => spellcast_entry.spell_lv3.push(spell.clone()),
                        4 => spellcast_entry.spell_lv4.push(spell.clone()),
                        5 => spellcast_entry.spell_lv5.push(spell.clone()),
                        6 => spellcast_entry.spell_lv6.push(spell.clone()),
                        7 => spellcast_entry.spell_lv7.push(spell.clone()),
                        8 => spellcast_entry.spell_lv8.push(spell.clone()),
                        9 => spellcast_entry.spell_lv9.push(spell.clone()),
                        10 => spellcast_entry.spell_lv10.push(spell.clone()),
                        11 => spellcast_entry.spell_focus.push(spell.clone()),
                        num => println!("Not found leve {}", num),
                    }
                }
            }
        }
    }

    fn build_traits(value: &Value) -> Result<Vec<TraitType>> {
        let mut return_vec = Vec::new();

        let alignment = value["data"]["details"]["alignment"]["value"]
            .as_str()
            .context("Failed to get => Alignment")?;
        let rarity = value["data"]["traits"]["rarity"]["value"]
            .as_str()
            .context("Failed to get => Rarity")?;
        let size = value["data"]["traits"]["size"]["value"]
            .as_str()
            .context("Failed to get => Size")?;
        let traits_custom = value["data"]["traits"]["traits"]["custom"]
            .as_str()
            .context("Failed to get => CustomTraits")?;

        let mut other_traits_vec = Vec::new();
        let traits = value["data"]["traits"]["traits"]["value"]
            .as_array()
            .context("Failed to get => traits vec")?;
        for val in traits.iter() {
            let other_trait = val.as_str().context("Failed to get => Traits")?;
            other_traits_vec.push(TraitType::Other(
                other_trait.to_ascii_uppercase().to_string(),
            ))
        }

        return_vec.push(TraitType::Rarity(rarity.to_ascii_uppercase().to_string()));
        return_vec.push(TraitType::Alignment(
            alignment.to_ascii_uppercase().to_string(),
        ));
        return_vec.push(TraitType::Size(Creature::size_fix(size)));
        return_vec.push(TraitType::Other(
            traits_custom.to_ascii_uppercase().to_string(),
        ));
        return_vec.append(&mut other_traits_vec);

        Ok(return_vec)
    }

    fn build_languages(value: &Value) -> Result<Vec<String>> {
        let mut result_vec = Vec::new();

        let custom = value["data"]["traits"]["languages"]["custom"]
            .as_str()
            .context("Failed to get => Languages Custom")?;

        if custom != "" {
            result_vec.push(custom.to_string());
        }

        for val in value["data"]["traits"]["languages"]["value"]
            .as_array()
            .context("Failed to get => Languages Array")?
        {
            let language = val.as_str().context("Failed to get => Language")?;

            if language != "" {
                result_vec.push(language.to_string());
            }
        }

        Ok(result_vec)
    }

    fn size_fix(size: &str) -> String {
        match size {
            "med" => "MEDIUM".to_string(),
            "lg" => "LARGE".to_string(),
            "grg" => "GARGANTUAN".to_string(),
            "huge" => "HUGE".to_string(),
            "sm" => "SMALL".to_string(),
            "tiny" => "TINY".to_string(),
            str => str.to_uppercase().to_string(),
        }
    }
}

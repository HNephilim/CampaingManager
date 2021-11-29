mod action;
pub use action::{ActionCategory, Action};

mod skill;
pub use skill::{Skill, SkillType};

mod consumable;
pub use consumable::Consumable;

mod equipment;
pub use equipment::Equipment;

mod armor;
pub use armor::Armor;

mod weapon;
pub use weapon::Weapon;

mod treasure;
pub use treasure::Treasure;

mod attack;
pub use attack::{Attack, Damage, WeaponType};

mod spell_entry;
pub use spell_entry::SpellEntry;

mod spell;
pub use spell::{Spell, SaveOption, SpellCategory, SpellComponents};

mod action_value;
pub use action_value::ActionValue;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub enum Items {
    
    Consumable(Consumable),
    Equipment(Equipment),
    Armor(Armor),
    Weapon(Weapon),
    Treasure(Treasure),
    
}

impl Items {
    pub fn name(&self) -> &String {
        match self{
            Items::Consumable(item) => {
                &item.name
            },
            Items::Equipment(item) => {
                &item.name
            },
            Items::Armor(item) => {
                &item.name
            },
            Items::Weapon(item) => {
                &item.name
            },
            Items::Treasure(item) => {
                &item.name
            },
        }
    }

    pub fn qtd(&self) -> i16{
        match self{
            Items::Consumable(item) => {
                item.quantity
            },
            Items::Equipment(item) => {
                item.quantity
            },
            Items::Armor(item) => {
                item.quantity as i16
            },
            Items::Weapon(item) => {
                item.quantity as i16
            },
            Items::Treasure(item) => {
                item.quantity
            },
        }
    }
}


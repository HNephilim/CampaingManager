use creature::items::Items;

use crate::{print::Print, state::State};

mod action;

mod skill;

mod consumable;

mod equipment;

mod armor;

mod weapon;

mod treasure;

mod attack;

mod spell_entry;

mod spell;

impl Print for Items {
    fn print(&self, ui: &imgui::Ui, state: &State) {
        match self {
            Items::Consumable(item) => item.print(ui, state),
            Items::Equipment(item) => item.print(ui, state),
            Items::Armor(item) => item.print(ui, state),
            Items::Weapon(item) => item.print(ui, state),
            Items::Treasure(item) => item.print(ui, state),
        }
    }
}

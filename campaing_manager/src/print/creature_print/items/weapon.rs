use creature::items::Weapon;

use crate::{print::Print, state::State};

impl Print for Weapon{
    fn print(&self, ui: &imgui::Ui, _state: &State) {
        ui.text(&self.name);

        ui.text(format!("Potency {}", self.potency));
        
        if self.striking.is_some(){
            ui.text(self.striking.as_ref().unwrap())
        }

        if self.property_1.is_some(){
            ui.text(self.property_1.as_ref().unwrap())
        }

        if self.property_2.is_some(){
            ui.text(self.property_2.as_ref().unwrap())
        }

        if self.property_3.is_some(){
            ui.text(self.property_3.as_ref().unwrap())
        }

        if self.property_4.is_some(){
            ui.text(self.property_4.as_ref().unwrap())
        }
    }
}
use creature::items::Armor;

use crate::{print::Print, state::State};

impl Print for Armor{
    fn print(&self, ui: &imgui::Ui, _state: &State) {
        ui.text(&self.name);

        ui.text(format!("Potency {}", self.potency));
        
        if self.resiliency.is_some(){
            ui.text(self.resiliency.as_ref().unwrap())
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
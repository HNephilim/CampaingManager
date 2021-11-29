use creature::items::Consumable;

use crate::{print::Print, state::State};


impl Print for Consumable{
    fn print(&self, ui: &imgui::Ui, _state: &State) {
        
        ui.text(&self.name);

        ui.text_wrapped(&self.desc);

        
    }
}
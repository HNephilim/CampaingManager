use creature::items::Equipment;

use crate::{print::Print, state::State};



impl Print for Equipment{
    fn print(&self, ui: &imgui::Ui, _state: &State) {
        ui.text(&self.name);

        ui.text_wrapped(&self.desc);
    }
}
use creature::items::Treasure;

use crate::{print::Print, state::State};

impl Print for Treasure{
    fn print(&self, ui: &imgui::Ui, _state: &State) {
        ui.text(&self.name);

        ui.text_wrapped(&self.desc);

        ui.text(format!("Value: {}gp", &self.value));
    }
}
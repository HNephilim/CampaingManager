use creature::Hp;

use crate::{print::Print, state::State};

impl Print for Hp {
    fn print(&self, ui: &imgui::Ui, state: &State) {
        state.print_title(ui, 1, "HP"); 
        ui.same_line();

        ui.text(format!("{}", self.modifier));

        if self.desc.is_some() {
            ui.same_line();
            ui.text(format!("({})", self.desc.as_ref().unwrap()))
        }

        ui.same_line_with_spacing(0. , 0.);
        ui.text(";");
    }
}

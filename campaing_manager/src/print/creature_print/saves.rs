use crate::{print::Print, state::State};

use creature::Saves;

impl Print for Saves {
    fn print(&self, ui: &imgui::Ui, state: &State) {
        state.print_title(ui, 1, "Fort"); 
        ui.same_line();
        if self.fortitude >= 0 {
            ui.text(format!("+{}", self.fortitude));
        } else {
            ui.text(format!("{}", self.fortitude));
        }
        if self.fortitude_desc.is_some() {
            ui.same_line();
            ui.text(format!("({})",self.fortitude_desc.as_ref().unwrap()));
        }
        ui.same_line_with_spacing(0., 0.);
        ui.text(",");

        ui.same_line();

        state.print_title(ui, 1, "Ref"); 
        ui.same_line();
        if self.reflex >= 0 {
            ui.text(format!("+{}", self.reflex));
        } else {
            ui.text(format!("{}", self.reflex));
        }
        if self.reflex_desc.is_some() {
            ui.same_line();
            ui.text(format!("({})",self.reflex_desc.as_ref().unwrap()));
        }
        ui.same_line_with_spacing(0., 0.);
        ui.text(",");

        ui.same_line();

        state.print_title(ui, 1, "Will"); 
        ui.same_line();
        if self.will >= 0 {
            ui.text(format!("+{}", self.will));
        } else {
            ui.text(format!("{}", self.will));
        }
        if self.will_desc.is_some() {
            ui.same_line();
            ui.text(format!("({})",self.will_desc.as_ref().unwrap()));
        }

        if self.all_desc.is_some() {
            ui.same_line_with_spacing(0., 0.);
            ui.text(",");
            ui.same_line();
            ui.text(self.all_desc.as_ref().unwrap())
        }
    }
}

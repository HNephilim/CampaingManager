use creature::Ac;

use crate::{print::Print, state::State};


impl Print for Ac{
    fn print(&self, ui: &imgui::Ui, state: &State) {
        state.print_title(ui, 1, "AC"); 
        ui.same_line();
        
        ui.text(format!("{}", self.modifier));

        if self.desc.is_some(){
            ui.same_line();
            ui.text(format!("({})", self.desc.as_ref().unwrap()))
        }
    }
}
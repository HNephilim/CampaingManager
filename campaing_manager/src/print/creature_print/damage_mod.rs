use creature::{DamageModifier, DmgMod};

use crate::{print::Print, state::State};

impl Print for DamageModifier {
    fn print(&self, ui: &imgui::Ui, state: &State) {
        if self.immunities.len() > 0 {
            state.print_title(ui, 1, "Immunities"); 
            
            for (index, immunit) in self.immunities.iter().enumerate() {
                ui.same_line();
                ui.text(immunit);
                if index != self.immunities.len() - 1 {
                    ui.same_line_with_spacing(0., 0.);
                    ui.text(",");
                } else {
                    ui.same_line_with_spacing(0., 0.);
                    ui.text(";");
                }
                
            }
        }

        if self.weaknesses.len() > 0 {
            ui.same_line();
            state.print_title(ui, 1, "Weaknesses"); 
            
            for (index, dmg_mod) in self.weaknesses.iter().enumerate(){
                ui.same_line();
                dmg_mod.print(ui, state);
                if index != self.weaknesses.len() - 1 {
                    ui.same_line_with_spacing(0., 0.);
                    ui.text(",");
                } else{
                    ui.same_line_with_spacing(0., 0.);
                    ui.text(";");
                }

            }
        }

        if self.resistances.len() > 0 {
            ui.same_line();
            state.print_title(ui, 1, "Resistances"); 
            ui.same_line();
            for (index, dmg_mod) in self.resistances.iter().enumerate(){
                ui.same_line();
                dmg_mod.print(ui, state);
                if index != self.resistances.len() - 1{
                    ui.same_line_with_spacing(0., 0.);
                    ui.text(",");
                } else{
                    ui.same_line_with_spacing(0., 0.);
                    ui.text(";");
                }

            }
        }
    }
}

impl Print for DmgMod {
    fn print(&self, ui: &imgui::Ui, _state: &State) {
        ui.text(format!("{}", self.label));
        if self.value.is_some(){
            ui.same_line();
            ui.text(format!("{}", self.value.as_ref().unwrap()));
        }
        if self.exception.is_some(){
            ui.same_line();
            ui.text(format!("({})", self.exception.as_ref().unwrap()))
        }
    }
}

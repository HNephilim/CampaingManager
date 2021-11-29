use creature::items::Skill;

use crate::{print::Print, state::State};

impl Print for Skill{
    fn print(&self, ui: &imgui::Ui, _state: &State) {
        ui.text(self.skill.to_str());
        ui.same_line();
        
        if self.modifier >= 0{
            ui.text(format!{"+{}", self.modifier})
        }else{
            ui.text(format!("{}", self.modifier));
        }

        if self.desc.is_some(){
            ui.same_line();
            ui.text(self.desc.as_ref().unwrap());
        }

        if self.variants.is_some(){
            for variant in self.variants.as_ref().unwrap(){
                ui.same_line();
                ui.text(format!("({})", variant));
            }
        }

        ui.same_line();
        ui.text(",")
        
    }
}





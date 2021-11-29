use creature::items::Action;

use crate::{print::Print, state::State};

impl Print for Action{
    fn print(&self, ui: &imgui::Ui, state: &State) {
        if ui.small_button( &self.name){
            ui.open_popup(&self.name)
        }
        if let Some(_token) = ui.begin_popup(&self.name){
            
            if self.desc.is_some(){
                
                let token = ui.push_text_wrap_pos_with_pos(300.);
                ui.text(self.desc.as_ref().unwrap());
                token.pop(ui);
            }
        }

        ui.same_line();

        self.action_type.print(ui, state);
        
        
        
        let mut traits_string = String::new();
        for (index, traits) in self.traits.iter().enumerate(){
            traits_string.push_str(traits.as_str());

            if index != self.traits.len() -1 {
                traits_string.push_str(", ")
            }            
        }
        if traits_string.len() > 0{
            ui.same_line();
            ui.text(format!("({})", traits_string))
        }  

        
    }
}
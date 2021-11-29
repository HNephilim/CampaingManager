use creature::items::{SaveOption, Spell, SpellCategory, SpellComponents};

use crate::{helper_functions::print_vec_same_line, print::Print};

impl Print for Spell {
    fn print(&self, ui: &imgui::Ui, state: &crate::state::State) {
        let token = ui.push_font(state.font(1));
        ui.text(&self.name.to_ascii_uppercase());
        ui.same_line_with_spacing(0., 50.);
        match self.category{
            SpellCategory::Focus => {
                if self.traits.iter().any(|trt| trt.get().as_str() == "CANTRIP"){
                    ui.text(format!("CANTRIP {}", &self.level));
                }else{
                    ui.text(format!("FOCUS {}", &self.level));
                }
                
            },
            SpellCategory::Spell => {
                if self.traits.iter().any(|trt| trt.get().as_str() == "CANTRIP"){
                    ui.text(format!("CANTRIP {}", &self.level));
                }else{
                    ui.text(format!("SPELL {}", &self.level));
                }               
                
            },
            SpellCategory::Ritual => {
                ui.text(format!("RITUAL {}", &self.level));
            },
        }
        
        token.pop();

        ui.separator(); //----------------------------------------------------------------

        print_vec_same_line(&self.traits, ui, state);

        state.print_title(ui, 1, "Cast");
        ui.same_line();
        self.action_value.print(ui, state);
        ui.same_line();
        self.components.print(ui, state);

        if self.materials.is_some(){
            ui.same_line_with_spacing(0.,0.);
            ui.text(";");
            ui.same_line();
            state.print_title(ui,1, "Cost");
            ui.same_line();
            ui.text(format!("{}",self.materials.as_ref().unwrap()));
        }

        if self.range.is_some(){
            state.print_title(ui, 1, "Range");
            ui.same_line();
            ui.text(format!("{};",self.range.as_ref().unwrap()));
        }

        if self.target.is_some(){
            if self.range.is_some(){
                ui.same_line();
            }
            state.print_title(ui, 1, "Targets");
            ui.same_line();
            ui.text(format!("{}",self.target.as_ref().unwrap()));
        }

        if self.area.is_some(){
            state.print_title(ui, 1, "Area");
            ui.same_line();
            ui.text(format!("{}",self.area.as_ref().unwrap()));
        }

        if self.save.is_some(){
            state.print_title(ui, 1, "Saving Throw");
            ui.same_line();
            self.save.as_ref().unwrap().print(ui,state);
        }

        if self.duration.is_some(){
            if self.save.is_some(){
                ui.same_line();
            }
            state.print_title(ui, 1, "Duration");
            ui.same_line();
            ui.text(format!("{}",self.duration.as_ref().unwrap()));
        }

        let token = ui.push_text_wrap_pos_with_pos(400.);
        ui.text_wrapped(&self.description);
        token.pop(ui);
        
    }
}


impl Print for SpellComponents{
    fn print(&self, ui: &imgui::Ui, _state: &crate::state::State) {
        let mut string = String::new();

        if self.somatic{
            string.push_str("somatic");
        }
        if self.verbal{
            if string.len() > 0{
                string.push(',')
            }
            string.push_str("verbal");
        }
        if self.material{
            if string.len() > 0{
                string.push(',')
            }
            string.push_str("material");
        }

        if string.len() > 0 {
            ui.text(string);
        }else{
            ui.new_line();
        }
    }
}

impl Print for SaveOption{
    fn print(&self, ui: &imgui::Ui, _state: &crate::state::State) {
        match self{
            SaveOption::Fortitude(bool) => {
                let mut string = String::new();
                if *bool{
                    string.push_str("basic");
                }
                string.push_str("Fortitude");
                
                ui.text(string)
            },
            SaveOption::Reflex(bool) => {
                let mut string = String::new();
                if *bool{
                    string.push_str("basic");
                }
                string.push_str("Reflex");
            
                ui.text(string)
            },
            SaveOption::Will(bool) => {
                let mut string = String::new();
                if *bool{
                    string.push_str("basic");
                }
                string.push_str("Will");
            
                ui.text(string)
            },
        }
    }
}
use creature::items::{Attack, Damage, WeaponType};

use crate::print::Print;


impl Print for Attack{
    fn print(&self, ui: &imgui::Ui, state: &crate::state::State) {
        match self.range {
            WeaponType::Melee => {
                state.print_title(ui, 1, "Melee")
            },
            WeaponType::Ranged => {
                state.print_title(ui, 1, "Ranged")
            },
        }
        
        ui.same_line();
        
        imgui::Image::new(state.texture(1), [15., 15.]).build(ui);
        
        ui.same_line();
        
        ui.text(&self.name);
        
        ui.same_line();

        if self.bonus >= 0 {
            ui.text(format!("+{}", self.bonus))
        }else{
            ui.text(format!("{}", self.bonus))
        }        

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
        
        ui.indent();
        state.print_title(ui, 1, "Damage");
        
        for (index, damage) in self.damage.iter().enumerate(){
            ui.same_line();
            damage.print(ui, state);

            if index != self.damage.len() - 1 {
                ui.same_line();
                ui.text("+")
            }
        }

        if self.attack_fx.len() > 0 {
            ui.same_line();
            ui.text("plus");
            
            for (index, attack_fx) in self.attack_fx.iter().enumerate(){
                ui.same_line();
                ui.text(attack_fx);
    
                if index != self.attack_fx.len() - 1 {
                    ui.same_line();
                    ui.text("and")
                }
            }
        
        }

        ui.unindent();
        
    }
}

impl Print for Damage{
    fn print(&self, ui: &imgui::Ui, _state: &crate::state::State) {
        if self.damage.is_some(){
            ui.text(self.damage.as_ref().unwrap());
        }
        ui.same_line();
        if self.damage_type.is_some(){
            ui.text(self.damage_type.as_ref().unwrap());
        }
    }
}



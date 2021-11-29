use creature::AbilityScore;

use crate::{print::Print, state::State};

impl Print for AbilityScore {
    fn print(&self, ui: &imgui::Ui, state: &State) {
        print_ability(ui, "Str", self.strenght, state);
        ui.same_line();
        print_ability(ui, "Dex", self.dexterity, state);
        ui.same_line();
        print_ability(ui, "Con", self.constituition, state);
        ui.same_line();
        print_ability(ui, "Int", self.inteligency, state);
        ui.same_line();
        print_ability(ui, "Wis", self.wisdom, state);
        ui.same_line();
        print_ability(ui, "Cha", self.charisma, state);      
        
    }
}

fn print_ability(ui: &imgui::Ui, ability: &str, value: i8, state: &State) {
    

    state.print_title(ui, 1, ability); 
    ui.same_line();

    if value >= 0{
        ui.text(format!("+{:?},",value));

    }
    else{
        ui.text(format!("{:?},",value));

    }
    
    
}

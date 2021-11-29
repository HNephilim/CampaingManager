use creature::items::{Spell, SpellEntry};

use crate::print::Print;


impl Print for SpellEntry{
    fn print(&self, ui: &imgui::Ui, state: &crate::state::State) {
        state.print_title(ui, 1, self.name.as_str());
        ui.same_line();
        ui.text(format!{"DC {}", self.dc});
        
        if self.modifier > 0 {
            ui.same_line_with_spacing(0., 0.);
            ui.text(format!(", attack +{}", self.modifier));
        }
        
        ui.indent();
        SpellEntry::print_spell_vec(&self.spell_focus, "Focus", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv10, "10th", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv9, "9th", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv8, "8th", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv7, "7th", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv6, "6th", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv5, "5th", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv4, "4th", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv3, "3rd", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv2, "2nd", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv1, "1st", ui, state);
        SpellEntry::print_spell_vec(&self.spell_lv0, "Cantrips", ui, state);
        ui.unindent();
    }
}

trait PrintHelper{
    fn print_spell_vec(vec: &Vec<Spell>, level: &str, ui: &imgui::Ui, state: &crate::state::State){
        if vec.len() > 0 {
            state.print_title(ui, 1, level);
            for (index, spell) in vec.iter().enumerate(){
                ui.same_line();
                ui.text(&spell.name);

                if ui.is_item_clicked(){
                    ui.open_popup(&spell.name);
                }
                if let Some(_token) = ui.begin_popup(&spell.name) {
                    spell.print(ui, state);                    
                }

                ui.same_line_with_spacing(0., 0.);
                if index != vec.len() - 1{
                    ui.text(",")
                }else{
                    ui.text(";")
                }
            }
        }
    } 
}

impl PrintHelper for SpellEntry{}
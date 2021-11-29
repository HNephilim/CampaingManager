use crate::{helper_functions::print_vec_same_line, print::Print, state::State};

use creature::{Creature, items::ActionCategory};

impl Print for Creature {
    fn print(&self, ui: &imgui::Ui, state: &State) {
        imgui::Window::new("Monster")
            .title_bar(false)
            .always_auto_resize(true)
            .build(ui, || {
                let token = ui.push_font(state.font(1));
                ui.text(&self.name.to_ascii_uppercase());
                ui.same_line_with_spacing(0., 50.);
                ui.text(format!("CREATURE {}", &self.level));
                token.pop();

                ui.separator(); //----------------------------------------------------------------

                print_vec_same_line(&self.traits, ui, state);

                state.print_title(ui, 1, "Perception");
                ui.same_line();
                ui.text(format!("+{}", &self.perception));

                if self.senses.is_some() {
                    ui.same_line();
                    ui.text(&self.senses.as_ref().unwrap());
                }

                if self.languages.len() > 0 {
                    state.print_title(ui, 1, "Languages");                    
                    ui.same_line();
                    for (index, lang) in self.languages.iter().enumerate() {
                        if index % 5 != 0 {
                            ui.same_line();
                        }
                        ui.text(format!("{},", lang))
                    }
                }

                state.print_title(ui, 1, "Skills");
                ui.same_line();
                for (index, skill) in self.skills.iter().enumerate() {
                    if index % 4 != 0 {
                        ui.same_line();
                    }
                    skill.print(ui, state);
                }

                self.ability_scores.print(ui, state);

                if self.items.len() > 0 {
                    state.print_title(ui, 1, "Items"); 
                    ui.same_line();
                    for (index, item) in self.items.iter().enumerate() {
                        if index % 5 != 0 {
                            ui.same_line();
                        }
                        if item.qtd() > 1 {
                            ui.text(format!("{} x{},", item.name(), item.qtd()));
                        } else {
                            ui.text(format!("{},", item.name()));
                        }

                        if ui.is_item_clicked() {
                            ui.open_popup(item.name())
                        }
                        if let Some(_token) = ui.begin_popup(item.name()) {
                            
                            let token = ui.push_text_wrap_pos_with_pos(300.);
                            item.print(ui, state);
                            token.pop(ui);
                        }
                    }
                }

                if self.actions.len() > 0 {
                    for action in self.actions.iter() {
                        if let ActionCategory::Interaction(action) = action {
                            action.print(ui, state)
                        }
                    }
                }

                ui.separator(); //----------------------------------------------------------------

                self.ac.print(ui, state);
                ui.same_line();
                self.saves.print(ui, state);

                self.hp.print(ui, state);
                if self.damage_mod.has_something() {
                    ui.same_line();
                    self.damage_mod.print(ui, state);
                }

                if self.actions.len() > 0 {
                    for action in self.actions.iter() {
                        if let ActionCategory::Defensive(action) = action {
                            action.print(ui, state)
                        }
                    }
                }

                ui.separator(); //----------------------------------------------------------------

                if self.speeds.len() > 0 {
                    state.print_title(ui, 1, "Speed"); 
                    ui.same_line();
                    let mut speed_str = String::new();
                    for (index, speed) in self.speeds.iter().enumerate() {
                        if index == 0 {
                            ui.text(speed);
                            continue;
                        }

                        speed_str.push_str(speed.as_str());
                        if index != self.speeds.len() - 1 {
                            speed_str.push_str(", ");
                        }
                    }
                    if speed_str.len() > 0 {
                        ui.same_line();
                        ui.text(format!("({})", speed_str))
                    }
                }

                if self.attacks.len() > 0{
                    for attack in self.attacks.iter(){
                        attack.print(ui, state)
                    }
                }

                if self.spellcast_entry.len() > 0{
                    for spellcast_entry in self.spellcast_entry.iter(){
                        spellcast_entry.print(ui, state);
                    }
                }

                if self.actions.len() > 0 {
                    for action in self.actions.iter() {
                        if let ActionCategory::Offensive(action) = action {
                            action.print(ui, state)
                        }
                    }
                }

            });
    }
}

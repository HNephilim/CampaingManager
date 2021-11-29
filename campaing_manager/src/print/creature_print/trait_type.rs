use creature::TraitType;

use crate::{print::Print, state::State};

trait PrintHelper{
    fn deploy_button(trait_str: &String, ui: &imgui::Ui) {
        if trait_str.as_str() == ""{
            return;
        }
        ui.button(trait_str);
    }

    fn setup_color<'ui>(color: [f32; 4], ui: &'ui imgui::Ui) -> [imgui::ColorStackToken<'ui>; 3] {
        [
            ui.push_style_color(imgui::StyleColor::Button, color),
            ui.push_style_color(imgui::StyleColor::ButtonActive, color),
            ui.push_style_color(imgui::StyleColor::ButtonHovered, color),
        ]
    }
}

impl PrintHelper for TraitType{}

impl Print for TraitType {
    fn print(&self, ui: &imgui::Ui, _state: &State) {
        match self {
            TraitType::Alignment(trait_str) => {
                let tokens = TraitType::setup_color(TraitType::COLOR_ALIGNMENT, ui);
                TraitType::deploy_button(trait_str, ui);
                tokens.into_iter().for_each(|token| token.pop());
            }
            TraitType::Rarity(trait_str) => {
                let tokens = TraitType::setup_color(TraitType::COLOR_RARITY, ui);
                TraitType::deploy_button(trait_str, ui);
                tokens.into_iter().for_each(|token| token.pop());
            }
            TraitType::Size(trait_str) => {
                let tokens = TraitType::setup_color(TraitType::COLOR_SIZE, ui);
                TraitType::deploy_button(trait_str, ui);
                tokens.into_iter().for_each(|token| token.pop());
            }
            TraitType::Other(trait_str) => {
                let tokens = TraitType::setup_color(TraitType::COLOR_OTHER, ui);
                TraitType::deploy_button(trait_str, ui);
                tokens.into_iter().for_each(|token| token.pop());
            }
        }
    }
}

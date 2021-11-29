use serde::{Deserialize, Serialize};

use crate::state::State;
#[derive(Serialize, Deserialize, Clone)]
pub enum UiInstruction {
    Indent(Vec<UiInstruction>),
    Bold(String),
    Normal(String),
}

impl UiInstruction{
    pub fn print(&self, ui: &imgui::Ui, state: &State){
        match self{
            UiInstruction::Indent(instructions) => {
                ui.indent();
                let token = ui.push_text_wrap_pos_with_pos(state.text_wrap);
                for inst in instructions{
                    inst.print(ui, state);
                }
                token.pop(ui);
                ui.unindent();
                ui.new_line();
            },
            UiInstruction::Bold(str) => {
                ui.same_line();
                let tk = ui.push_font(state.font(1));
                ui.text_colored(State::COLOR_TITLE, str);
                tk.pop();
            },
            UiInstruction::Normal(str) => {
                ui.same_line();
                
                ui.text_wrapped(str);
                
            },
        }
    }
}


pub fn desc_parser(desc: String) -> Vec<UiInstruction> {
    let mut vec = Vec::new();

    for (index, paragraph) in desc.split("</p>").enumerate() {
        let mut paragraph_vec = Vec::new();

        paragraph_parser(paragraph, &mut paragraph_vec);

        vec.push(UiInstruction::Indent(paragraph_vec));
    }

    vec
}

fn paragraph_parser(paragraph: &str, return_vec: &mut Vec<UiInstruction>) {
    if paragraph.starts_with("<p") {
        let index = paragraph.find(">").unwrap();
        let new_paragraph = paragraph.split_at(index + 1).1;

        paragraph_parser(new_paragraph, return_vec);
    } else if paragraph.starts_with("<strong>") {
        let bold_start = paragraph.split_at(8).1;

        let final_bold_index = bold_start.find("</strong>").unwrap();

        let bold = bold_start.split_at(final_bold_index).0;

        return_vec.push(UiInstruction::Bold(bold.to_string()));

        let normal = bold_start.split_at(final_bold_index + 9).1;

        return_vec.push(UiInstruction::Normal(normal.to_string()));
    } else {

        if paragraph != ""{
            return_vec.push(UiInstruction::Normal(paragraph.to_string()));
        }
        
    }
}

pub fn parse(input: String, level: u8) -> (Vec<UiInstruction>, Vec<UiInstruction>) {
    let mut input_with_level = input
        .replace("<hr />", "")
        .replace("\n", "")
        .replace("</span>", "");
    
    let mut input_wout_level = input_with_level.clone();

    loop {
        if let Some(index) = input_with_level.find("@Compendium") {
            let str = input_with_level.split_at(index).1;
            let index_start = str.find("{").unwrap();
            let index_end = str.find("}").unwrap();
            let effect = str
                .split_at(index_start + 1)
                .1
                .split_at((index_end - index_start) - 1)
                .0;

            let replace_str = input_with_level.split_at(index).1.split_at(index_end + 1).0;

            

            input_wout_level = input_wout_level.replace(replace_str, effect);
            input_with_level = input_with_level.replace(replace_str, effect);

        } else if let Some(index) = input_with_level.find("[[") {
            let str = input_with_level.split_at(index).1;
            if let Some(index_start) = str.find("{"){
                let index_end = str.find("}").unwrap();
                let effect = str
                    .split_at(index_start + 1)
                    .1
                    .split_at((index_end - index_start) - 1)
                    .0;
    
                let replace_str = input_with_level.split_at(index).1.split_at(index_end + 1).0;
    
                input_wout_level = input_wout_level.replace(replace_str, effect);
                input_with_level = input_with_level.replace(replace_str, effect);

            }else{
                let index_start = str.find(" ").unwrap();
                let index_end = str.split_at(index_start+1).1.find(" ").unwrap();

                let effect = str
                .split_at(index_start + 1)
                .1
                .split_at(index_end)
                .0;

                let replace_str = input_with_level.split_at(index_start).1.split_at(index_end + 1).0;      
                
                println!("replace => {}", replace_str);
    
                input_wout_level = input_wout_level.replace(replace_str, effect);
                input_with_level = input_with_level.replace(replace_str, effect);
            }

            
            
            
        } else if let Some(index) = input_with_level.find("<span") {
            let str = input_with_level.split_at(index).1;
            if let Some(dc_index_start) = str.find("data-pf2-dc=") {
                let dc_index_end = str.split_at(dc_index_start + 13).1.find("\"").unwrap();
                let dc = str
                    .split_at(dc_index_start + 13)
                    .1
                    .split_at(dc_index_end)
                    .0
                    .parse::<u8>()
                    .unwrap();
                let dc_str = format!("DC {} ", dc);
                

                let dc_minus_level = dc - level;
                let dc_minus_level_str = format!("DC {} ", dc_minus_level);

                let index_end = str.find(">").unwrap();
                let replace_str = input_with_level.split_at(index).1.split_at(index_end + 1).0;

                input_wout_level= input_wout_level.replace(replace_str, &dc_minus_level_str);
                input_with_level = input_with_level.replace(replace_str, &dc_str);
            }
        } else {
            break;
        }
    }

    //println!("{}", input_with_level);

    let with_level = desc_parser(input_with_level);
    let wout_level = desc_parser(input_wout_level);

    (with_level, wout_level)
}

use std::io::Read;
use creature::items::ActionValue;
use creature::Creature;

use crate::print::Print;
use crate::state::State;





impl Print for ActionValue{
    fn print(&self, ui: &imgui::Ui, state: &State) {
        match &self{
            ActionValue::OneAction => {
                imgui::Image::new(state.texture(1), [15., 15.]).build(ui);
            },
            ActionValue::TwoActions => {
                imgui::Image::new(state.texture(2), [15., 15.]).build(ui);
            },
            ActionValue::ThreeActions => {
                imgui::Image::new(state.texture(3), [15., 15.]).build(ui);
            },
            ActionValue::FreeAction => {
                imgui::Image::new(state.texture(0), [15., 15.]).build(ui);
            },
            ActionValue::Reaction => {
                imgui::Image::new(state.texture(6), [15., 15.]).build(ui);
            },
            ActionValue::OneToThreeAction => {
                imgui::Image::new(state.texture(5), [15., 15.]).build(ui);
            },
            ActionValue::OneOrTwoActions => {
                imgui::Image::new(state.texture(4), [15., 15.]).build(ui);
            },
            ActionValue::Passive => {
                //imgui::Image::new(*state.tex_id.as_ref().unwrap().get(&1).unwrap(), [15., 15.]).build(ui);
                ui.text("")
            },
            ActionValue::Other(str) => {
                ui.text(&str)
            },
        }
    }
}


///Builds the ui where all the vectors are inlined
pub fn print_vec_same_line<T: Print>(vec: &Vec<T>, ui: &imgui::Ui, state: &State){
    for item in vec{
        ui.same_line();
        item.print(ui, state);
    }

}


pub fn load_db() -> Vec<(String, Creature)>{
    let mut file = Vec::new();
    std::fs::File::open("data/DB.rot").unwrap().read_to_end(&mut file).unwrap();

    bincode::deserialize(&file).unwrap()

}
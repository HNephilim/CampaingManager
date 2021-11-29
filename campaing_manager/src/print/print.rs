use crate::state::State;

pub trait Print{
    fn print(&self, ui: &imgui::Ui, state: &State);
}


impl Print for String{
    fn print(&self, ui: &imgui::Ui, _state: &State) {
        ui.same_line();
        ui.text(&self);
    }
}
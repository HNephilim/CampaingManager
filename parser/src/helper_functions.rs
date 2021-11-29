use anyhow::*;

use crate::{state::State, traits::{self, Print}};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ActionValue{
    OneAction,
    TwoActions,
    ThreeActions,
    FreeAction,
    Reaction,
    OneToThreeAction,
    OneOrTwoActions,
    Passive,
    Other(String)
}

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
pub fn print_vec_same_line<T: traits::Print>(vec: &Vec<T>, ui: &imgui::Ui, state: &State){
    for item in vec{
        ui.same_line();
        item.print(ui, state);
    }

}


///Converts a Option<&str> to a Option<String> returning None if input is None of &str is ""
pub fn option_str_to_option_string(input: Option<&str>) -> Option<String>{
    match input {
        Some(str) => str_to_option_string(str),
        None => None,
    }
}

///Converts a &str to a Option<String> returning None if input is ""
pub fn str_to_option_string(input: &str) -> Option<String>{
    match input{
        "" => None,
        str => Some(str.to_string())
    }
}

// pub fn load_db() -> (Vec<crate::creature::CreatureBuilder>, Vec<(String, Error)>){
//     let directory = std::fs::read_dir("data");

//     let json_files = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

//     let mut thread_pool = scoped_threadpool::Pool::new(6);

//     let json_files = thread_pool.scoped(move |scope| {
//         match directory {
//             Ok(read_dir) => {
//                 for dir in read_dir {
//                     match dir {
//                         Ok(folder) => {
//                             let dir = std::fs::read_dir(folder.path());
//                             match dir {
//                                 Ok(dir) => {
//                                     for file in dir {
//                                         match file {
//                                             Ok(file) => {
//                                                 let json_vec = std::sync::Arc::clone(&json_files);
//                                                 scope.execute(move || {
//                                                     let archieve =
//                                                         std::fs::File::open(file.path()).unwrap();
//                                                     let json: serde_json::Value =
//                                                         serde_json::from_reader(archieve).unwrap();

//                                                     let mut vec = json_vec.lock().unwrap();
//                                                     vec.push((format!("{:?}", file.path()), json))
//                                                 })
//                                             }
//                                             Err(e) => {
//                                                 println!("{}", e)
//                                             }
//                                         }
//                                     }
//                                 }
//                                 Err(e) => {
//                                     println!("{}", e)
//                                 }
//                             }
//                         }
//                         Err(e) => {
//                             println!("{}", e)
//                         }
//                     }
//                 }
//             }
//             Err(e) => {
//                 println!("{}", e)
//             }
//         }
//         return json_files;
//     });

//     let mut vec_npc = Vec::new();
//     let mut error_vec = Vec::new();

//     for (path, value) in json_files.as_ref().lock().unwrap().clone() {
//         match anyhow::Context::context(value["type"].as_str(), "ABLUBULBRULR").unwrap() {
//             "npc" => {
//                 let npc = crate::creature::CreatureBuilder::new(&value);
//                 match npc {
//                     Ok(creature) => vec_npc.push(creature),
//                     Err(err) => error_vec.push((path.clone(), err)),
//                 }
//             }
//             _ => {}
//         }
//     }

//     (vec_npc, error_vec)
// }
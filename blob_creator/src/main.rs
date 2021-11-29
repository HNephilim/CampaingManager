use std::io::Write;
use rayon::prelude::*;
use serde_json::Value;

use creature::Creature;


fn main() {
    let start = chrono::Local::now();

    let path_iter = glob::glob("data_backup/**/*.json")
        .unwrap()
        .filter_map(|x| x.ok())
        .collect::<Vec<_>>();

    let glob = chrono::Local::now();

    println!("Glob indexing => {}", glob - start);

    let vec_name =  path_iter.par_iter().filter_map(|path|{
        let file = std::fs::File::open(path.clone()).unwrap();
        let value: Value = serde_json::from_reader(file).unwrap();
        if value["type"].as_str().unwrap() == "npc"{
            let name = value["name"].as_str().unwrap().to_string();
            let monster = match Creature::new(&value){
                Ok(monster) => monster,
                Err(e) => {
                    panic!("{:?} => {}", path, e)
                }
            };
            return Some((name, monster));
        }       
        None        
    }).collect::<Vec<_>>(); 
    

    let time_final = chrono::Local::now();

    println!("Loop  => {}", time_final - glob);
        
    let data = bincode::serialize(&vec_name).unwrap();
    std::fs::File::create("data_blob/JustABlob.rot").unwrap().write(&data).unwrap();

    let bincode_complete = chrono::Local::now();

    println!("Bincode Total  => {}", bincode_complete - time_final);


    vec_name.par_iter().for_each(|item|{ 
        let mut name = std::path::PathBuf::from("data_bin");           
        name.push(format!("{}.rot", item.0).replace("'", "").replace("-", "").replace("\"", ""));        
        let mut file = match std::fs::File::create(&name){
            Ok(file) =>file,
            Err(e) => {
                panic!("{:?} => {}", name, e);
            },
        };
        let data = bincode::serialize(&item.1).unwrap();
        file.write(&data).unwrap();
    });


    let bincode_part = chrono::Local::now();

    println!("Bincode Part  => {}", bincode_part - bincode_complete);

    
}

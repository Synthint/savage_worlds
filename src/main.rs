#![allow(unused)]
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt::Debug;
use std::fs::{File, self};
use std::io::Read;
pub mod game_models;

use game_models::{Skill, Edge, Power, Armor, Weapon, Item, Character};

fn main() {

    let read_path = "./gamedata/skills.json";
    let write_path = "./gamedata/characters.json";

    let mut my_dude: Character = {
        let data = fs::read_to_string("./gamedata/characters.json").expect("Read items error");
        serde_json::from_str(&data).expect("Error on read in")
    };

    

    let mut my_items: Vec<Skill> = {
        let data = fs::read_to_string(read_path).expect("Read items error");
        serde_json::from_str(&data).expect("Error on read in")
    };

    

    //println!("{:?}\n\n", my_items);
    my_dude.add_skill(get_contains(my_items,"Boating"
        .to_string())
        .get(0)
        .expect("Skill does not exist"));

    fs::write(write_path, serde_json::to_string_pretty(&my_dude).expect("error on stringify")).expect("Error Writing");


    println!("{}", serde_json::to_string_pretty(&my_dude).expect("error on stringify"));
}

fn get_contains<T: std::fmt::Debug>(items: Vec<T>, search: String) -> Vec<T>{
    let mut output = Vec::new();
    for item in items{
        if format!("{:?}",item).to_lowercase().contains(&search.to_lowercase()){
            output.push(item);
        }
    }
    return output;
}
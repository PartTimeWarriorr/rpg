use crate::characters::*;
use crate::ability::*;
use std::{
    fs::File,
    io::BufReader,
};

pub fn load_friendly_party() -> Vec<Character> {
    let file = File::open("config/friendly_party.json").expect("Config file not found: friendly_party");
    let rdr = BufReader::new(file);

    match serde_json::from_reader(rdr) {
        Ok(json) => json,
        Err(err) => panic!("Error when parsing json: {}", err)
    } 
}

pub fn load_enemies() ->  Vec<Vec<Character>> {
    let file = File::open("config/enemies.json").expect("Config file not found: enemies");
    let rdr = BufReader::new(file);

    match serde_json::from_reader(rdr) {
        Ok(json) => json,
        Err(err) => panic!("Error when parsing json: {}", err)
    } 
}

pub fn load_abilities() -> Vec<Ability> {
    let file = File::open("config/abilities.json").expect("Config file not found: abilities");
    let rdr = BufReader::new(file);

    match serde_json::from_reader(rdr) {
        Ok(json) => json,
        Err(err) => panic!("Error when parsing json: {}", err)
    } 
}
use crate::characters::*;
use crate::ability::*;
use crate::state::AbilityMap;
use std::collections::HashMap;
use std::hash::RandomState;
use std::{
    fs::File,
    io::BufReader,
    error::Error,
};

pub fn load_friendly_party(abilites: &AbilityMap) -> Result<Party, Box<dyn Error>> {
    let file = File::open("config/friendly_party.json")?;
    let rdr = BufReader::new(file);

    let party : Party = serde_json::from_reader(rdr)?;

    if party.characters.values().any(|ch| !ch.is_valid(abilites)) {
        Err(format!("Unknown ability when loading friendly party").into())
    } else {
        Ok(party)
    }
}

pub fn load_enemies(abilites: &AbilityMap) ->  Result<Vec<Vec<Character>>, Box<dyn Error>> {
    let file = File::open("config/enemies.json")?;
    let rdr = BufReader::new(file);

    let en : Vec<Vec<Character>> = serde_json::from_reader(rdr)?;

    if en.iter().flatten().any(|ch| !ch.is_valid(&abilites)) {
        Err(format!("Unknown ability when loading enemy party").into())
    } else {
        Ok(en)
    }
}


pub fn load_abilities() -> Result<AbilityMap, Box<dyn Error>> {
    let file = File::open("config/abilities.json")?;
    let rdr = BufReader::new(file);
    
    let hm = serde_json::from_reader(rdr)?;

    Ok(hm)
}
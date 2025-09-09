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

pub fn load_friendly_party() -> Result<Party, Box<dyn Error>> {
    let file = File::open("config/friendly_party.json")?;
    let rdr = BufReader::new(file);

    let party = serde_json::from_reader(rdr)?;

    Ok(party)
}

pub fn load_enemies() ->  Result<Vec<Vec<Character>>, Box<dyn Error>> {
    let file = File::open("config/enemies.json")?;
    let rdr = BufReader::new(file);

    let en = serde_json::from_reader(rdr)?;

    Ok(en)
}


pub fn load_abilities() -> Result<AbilityMap, Box<dyn Error>> {
    let file = File::open("config/abilities.json")?;
    let rdr = BufReader::new(file);
    
    let hm = serde_json::from_reader(rdr)?;

    Ok(hm)
}
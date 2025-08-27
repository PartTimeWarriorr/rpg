use crate::characters::Character;

use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct StatusEffect {
    attack: i32,
    defense: i32,
    speed: i32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum AbilityType {
    Damage,
    Heal,
    Status
}

#[derive(Clone, Debug, Deserialize)]
pub struct Ability {
    name: String,
    ability_type: AbilityType,
    power: u32,
    #[serde(default)]
    status_effect: Option<StatusEffect>,
}

impl Ability {

    pub fn new_empty(name: &str) -> Self {
        Ability {
            name: String::from(name),
            ability_type: AbilityType::Damage,
            power: 10,
            status_effect: None,
        }
    }

    pub fn new(name: &str, ability_type: AbilityType, power: u32, status_effect: StatusEffect) -> Self {
        Ability {
            name: String::from(name),
            ability_type,
            power,
            status_effect: Some(status_effect),
        }
    }

    pub fn perform(&self, target: &mut Character) {
        match self.ability_type {
            AbilityType::Damage => target.take_damage(self.power),
            AbilityType::Heal => target.heal(self.power),
            AbilityType::Status => target.status_effect(),
        }
    }
}


pub fn load_abilities() {
    let file = File::open("src/abilities.json").unwrap();
    let rdr = BufReader::new(file);

    let abilities : Vec<Ability> = serde_json::from_reader(rdr).expect("Bad formatting");
    dbg!(&abilities);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_abilities_test() {
        load_abilities();
    }
}
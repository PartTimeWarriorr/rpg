use crate::characters::Character;

#[derive(Debug)]
struct StatusEffect {
    attack: i32,
    defense: i32,
    speed: i32,
}

#[derive(Debug)]
enum AbilityType {
    Damage,
    Heal,
    Status
}

#[derive(Debug)]
struct Ability {
    name: String,
    ability_type: AbilityType,
    power: u32,
    status_effect: StatusEffect,
}

impl Ability {
    pub fn new(name: &str, ability_type: AbilityType, power: u32, status_effect: StatusEffect) -> Self {
        Ability {
            name: String::from(name),
            ability_type,
            power,
            status_effect,
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
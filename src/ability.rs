

struct StatusEffect {
    attack: i32,
    defense: i32,
    speed: i32,
}


enum AbilityType {
    Damage,
    Heal,
    Status
}

struct Ability {
    name: String,
    ability_type: AbilityType,
    power: i32,
    status_effect: StatusEffect,
}

impl Ability {
    pub fn new(name: &str, ability_type: AbilityType, power: i32, status_effect: StatusEffect) -> Self {
        Ability {
            name: String::from(name),
            ability_type,
            power,
            status_effect,
        }
    }

    pub fn perform(&self, target: Character) {
        match AbilityType {
            Damage => target.take_damage(self.power),
            Heal => target.heal(self.power),
            Status => target.effect(self.status_effect),
        }
    }
}
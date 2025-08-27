use crate::{ability::*, action, characters::*};

#[derive(Debug)]
pub enum ActionType {
    Fight,
    Guard,
    Item, // Item Id
    Flee
}

#[derive(Debug)]
pub struct Action {
    action_type: ActionType,
    actor: String,
    target: Option<String>,
    ability: Ability
}

impl Action {
    // pub fn new(actor: CharacterId, target: CharacterId, ability: Ability) -> Self {
    //     Action { actor, target, ability }
    // }

    pub fn new() -> PendingAction {
        PendingAction::new()
    }

    pub fn resolve(self, friendly_party: &mut Party, enemy_party: &mut Party) {
        let actor_ch = friendly_party.characters.iter_mut().find(|ch| ch.name == self.actor).unwrap();

        match self.action_type {
            ActionType::Fight => {
                let target_ch = enemy_party.characters.iter_mut().find(|ch| ch.name == self.target.clone().unwrap()).unwrap();
                let power = actor_ch.stats.attack;
                target_ch.take_damage(power);
            },
            ActionType::Guard => {
                actor_ch.stats.defense = actor_ch.stats.defense + 10;
            },
            ActionType::Item => {
                // use_item
                println!("Using item now");
            },
            ActionType::Flee => {
                // flee
                println!("Fleeing now");
            }
        }
    }

}

#[derive(Debug)]
pub struct PendingAction {
    action_type: Option<ActionType>,
    actor: Option<String>,
    target: Option<String>,
    ability: Option<String>
}

impl PendingAction {
    pub fn new() -> Self {
        PendingAction { action_type: None, actor: None, target: None, ability: None }
    }

    pub fn action_type(&mut self, action_type: ActionType) -> &mut Self {
        self.action_type = Some(action_type);
        self
    }
    pub fn actor(&mut self, actor: String) -> &mut Self {
        self.actor = Some(actor);
        self
    }
    pub fn target(&mut self, target: String) -> &mut Self {
        self.target = Some(target);
        self
    }
    pub fn ability(&mut self, ability: String) -> &mut Self {
        self.ability = Some(ability);
        self
    }

    pub fn build(&mut self) -> Action {
        Action {
            action_type: self.action_type.take().unwrap(),
            actor: self.actor.take().unwrap(),
            target: if let Some(t) = self.target.take() {
                Some(t)
            } else {
                None
            },
            ability: Ability::new_empty("")
        } 
        // Action { action_type: self.action_type.unwrap(), actor: self.actor.unwrap(), target: self.target.unwrap(), ability: Ability::new(String::from("s")) }
    }
}

// pub fn create_new_action(actor: CharacterId, target: CharacterId, ability: Ability) {
// }


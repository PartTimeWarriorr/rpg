use crate::{ability, action, characters::*};

#[derive(Debug)]
pub enum ActionType {
    Fight,
    Guard,
    Item, // Item Id
    Flee
}

pub struct Action {
    action_type: ActionType,
    actor: CharacterId,
    target: CharacterId,
    ability: Ability
}

impl Action {
    // pub fn new(actor: CharacterId, target: CharacterId, ability: Ability) -> Self {
    //     Action { actor, target, ability }
    // }

    pub fn new() -> PendingAction {
        PendingAction::new()
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

    // pub fn build(&self) -> Action {
        // Action { action_type: self.action_type.unwrap(), actor: self.actor.unwrap(), target: self.target.unwrap(), ability: self.ability }
    // }
}

// pub fn create_new_action(actor: CharacterId, target: CharacterId, ability: Ability) {
// }


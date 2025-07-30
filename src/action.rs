use crate::{ability, characters::*};
struct Action {
    actor: CharacterId,
    target: CharacterId,
    ability: Ability
}

impl Action {
    pub fn new(actor: CharacterId, target: CharacterId, ability: Ability) -> Self {
        Action { actor, target, ability }
    }

}

pub fn create_new_action(actor: CharacterId, target: CharacterId, ability: Ability) {

}
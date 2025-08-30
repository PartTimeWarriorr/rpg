use std::{fmt, io::BufReader};

use crate::{ability::{self, *}, action, characters::*, ui::DialogueBox};

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
    pub actor: String,
    target: Option<String>,
    ability: Option<Ability>
}

impl Action {

    pub fn new() -> PendingAction {
        PendingAction::new()
    }

    pub fn resolve(self, friendly_party: &mut Party, enemy_party: &mut Party, dialogue_box: &mut DialogueBox) {

        match self.action_type {
            ActionType::Fight => {

                let ability = self.ability.unwrap();

                match ability.ability_type {
                    AbilityType::Damage => {

                        let actor_ch = friendly_party.characters.iter_mut().find(|ch| ch.name == self.actor).unwrap();
                        actor_ch.state = CharacterState::Default;
                        let target_ch = enemy_party.characters.iter_mut().find(|ch| ch.name == self.target.clone().unwrap()).unwrap();

                        let mut attack_power = actor_ch.calculate_attack(); 
                        attack_power += ability.power;
                        target_ch.take_damage(attack_power);

                        dialogue_box.notify(&ability.message.replace("{}", &actor_ch.name));
                        dialogue_box.notify(&format!("{} takes a hit!", target_ch.name));

                    },
                    AbilityType::Status => {
                        let target_ch = enemy_party.characters.iter_mut().find(|ch| ch.name == self.target.clone().unwrap()).unwrap();
                        // TODO
                    },
                    AbilityType::Buff => {

                        let actor_index = friendly_party.characters.iter().position(|ch| ch.name == self.actor).unwrap();
                        let target_index = friendly_party.characters.iter().position(|ch| ch.name == self.target.clone().unwrap()).unwrap();

                        let b = ability.buff.expect("Unexpected buff variant");

                        if actor_index == target_index {

                            let actor_ch = &mut friendly_party.characters[actor_index];
                            actor_ch.state = CharacterState::Default;
                            actor_ch.buffs.push(b);

                            dialogue_box.notify(&ability.message.replace("{}", &actor_ch.name));

                        } else {

                            let (actor_ch, target_ch) = friendly_party.get_two_members_mut(actor_index, target_index);
                            actor_ch.state = CharacterState::Default;
                            target_ch.buffs.push(b);

                            dialogue_box.notify(&ability.message.replace("{}", &actor_ch.name));

                        }
                    },
                    AbilityType::Heal => {

                        let actor_index = friendly_party.characters.iter().position(|ch| ch.name == self.actor).unwrap();
                        let target_index = friendly_party.characters.iter().position(|ch| ch.name == self.target.clone().unwrap()).unwrap();

                        if actor_index == target_index {

                            let actor_ch = &mut friendly_party.characters[actor_index];
                            actor_ch.state = CharacterState::Default;
                            actor_ch.heal(ability.power);
                            dialogue_box.notify(&ability.message.replace("{}", &actor_ch.name));

                        } else {

                            let (actor_ch, target_ch) = friendly_party.get_two_members_mut(actor_index, target_index);
                            actor_ch.state = CharacterState::Default;
                            target_ch.heal(ability.power);
                            dialogue_box.notify(&ability.message.replace("{}", &actor_ch.name));

                        }
                    }
                }
            },
            ActionType::Guard => {

                let actor_ch = friendly_party.characters.iter_mut().find(|ch| ch.name == self.actor).unwrap();
                actor_ch.state = CharacterState::Guarding;
                dialogue_box.notify(&format!("{} is guarding!", actor_ch.name));

            },
            ActionType::Item => {

                // use_item
                let actor_ch = friendly_party.characters.iter_mut().find(|ch| ch.name == self.actor).unwrap();
                actor_ch.state = CharacterState::Default;
                dialogue_box.notify(&format!("{} used an item!", actor_ch.name));

            },
            ActionType::Flee => {

                // flee
                let actor_ch = friendly_party.characters.iter_mut().find(|ch| ch.name == self.actor).unwrap();
                actor_ch.state = CharacterState::Default;
                dialogue_box.notify(&format!("{} is trying to flee!", actor_ch.name));

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

    pub fn build(&mut self, abilities: &Vec<Ability>) -> Action {
        Action {
            action_type: self.action_type.take().unwrap(),
            actor: self.actor.take().unwrap(),
            target: if let Some(t) = self.target.take() {
                Some(t)
            } else {
                None
            },
            ability: if let Some(ab_name) = self.ability.take() {
                Some(abilities.iter().find(|ab| ab.name == ab_name).unwrap().clone())
            } else {
                None
            }
        } 
    }
}

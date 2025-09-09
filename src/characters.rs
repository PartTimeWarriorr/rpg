use std::{cmp::{
    max, min
}, hash::RandomState};

use ggez::{
    Context,
    graphics::{
        self,
        Canvas,
        Color,
        Rect,
    },
    mint::Point2,

};
use ordermap::OrderMap;

use crate::{assets::Assets, characters};

use serde::Deserialize;

// for drawing purposes?
#[derive(Clone, Deserialize, Debug)]
pub enum CharacterState {
    Default, 
    Attacking,
    Guarding,
    Damaged
}

fn default_state() -> CharacterState {
    CharacterState::Default
}

#[derive(Clone, Copy, Deserialize, Debug)]
pub struct Stats {
    max_health: u32,
    pub attack: u32,
    pub defense: u32,
    speed: u32,
}

impl Stats {
    pub fn new(max_health: u32, attack: u32, defense: u32, speed: u32) -> Self {
        Stats { max_health, attack, defense, speed }
    }
}

#[derive(Clone, Copy, Deserialize, Debug)]
pub enum Buff {
    Attack(u32),
    Defense(u32),
    Speed(u32)
}

type CharacterMap = OrderMap<CharacterId, Character, RandomState>;
#[derive(Debug)]
pub struct Party {
    pub characters: CharacterMap,
    // pub items: OrderMap<Item, usize, RandomState>,
}

impl Party {

    pub fn new(characters : CharacterMap) -> Self {
        Party { characters }
    }

    pub fn new_empty() -> Self {
        Party {
            characters: CharacterMap::new(),
        }
    }

    pub fn add_member(&mut self, new_character: Character) {
        self.characters.insert(new_character.id, new_character);
    }

    pub fn update_action_points(&mut self) {
        for ch in self.characters.values_mut() {
            ch.update_action_points(); 
        }
    }

    // pub fn draw(&self, ctx : &Context, canvas: &mut graphics::Canvas, assets: &Assets) {

    //     let mut curr_position = self.position;

    //     for c in &self.characters {
    //         c.draw(ctx, canvas, assets, curr_position);
    //         curr_position = Point2::from_slice(&[curr_position.x, curr_position.y + 50.0]);
    //     }
    // }

    // pub fn get_member_by_id(&self, character_id: CharacterId) -> Character {
    //     if let Some(character) = self.characters.iter().find(|c| c.id == character_id) {
    //         character.clone()
    //     } else {
    //         panic!("No character with id {} found!", character_id);
    //     }
    // }
    pub fn get_member_by_id(&self, character_id: CharacterId) -> Option<&Character> {
        self.characters.get(&character_id)
    }

    // pub fn is_character_name(&self, name: &String) -> bool {
    //     self.characters.iter().any(|ch| ch.name == *name)
    // }
    pub fn is_character_name(&self, name: &String) -> bool {
        self.characters.values().any(|ch| ch.name == *name)
    }

    // pub fn get_two_members_mut(&mut self, actor_index: usize, target_index: usize) -> (&mut Character, &mut Character) {

    //     if actor_index < target_index {
    //         let (left, right) = self.characters.split_at_mut(target_index);
    //         (&mut left[actor_index], &mut right[0])
    //     } else if actor_index > target_index {
    //         let (left, right) = self.characters.split_at_mut(actor_index);
    //         (&mut right[0], &mut left[target_index])
    //     } else {
    //         panic!("How am I targetting myself?");
    //     }

    // }

    pub fn get_two_mut(&mut self, actor_index: CharacterId, target_index: CharacterId) -> [Option<&mut Character>; 2] {
        self.characters.get_disjoint_mut([&actor_index, &target_index])
    }
}


impl<'de> Deserialize<'de> for Party {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
                
        let raw = Vec::<Character>::deserialize(deserializer)?;

        Ok(
            Party { 
                characters: raw
                    .into_iter()
                    .map(|c| (c.id, c))
                    .collect::<CharacterMap>()
            } 
        )
    }
}

pub const MAX_ACTION_POINTS : u32 = 500;

pub type CharacterId = u32;

#[derive(Deserialize)]
struct CharacterRaw {
    pub id: CharacterId,
    pub name: String,
    #[serde(default = "default_state")]
    pub state: CharacterState,
    pub abilities: Vec<String>,
    pub sprite: String,
    pub is_friendly: bool,
    pub stats: Stats,
    #[serde(default)]
    pub action_points: u32,
}

impl<'de> Deserialize<'de> for Character {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let raw = CharacterRaw::deserialize(deserializer)?;

        Ok(
            Character { 
                id: raw.id,
                name: raw.name, 
                state: raw.state, 
                abilities: raw.abilities, 
                sprite: raw.sprite, 
                is_friendly: raw.is_friendly, 
                stats: raw.stats, 
                action_points: raw.action_points, 
                health: raw.stats.max_health,
                buffs: vec![],
            }   
        )
    }
}

#[derive(Clone, Debug)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub state: CharacterState,
    pub abilities: Vec<String>,
    pub sprite: String,
    pub is_friendly: bool,
    pub stats: Stats,
    pub action_points: u32,
    pub health: u32,
    pub buffs: Vec<Buff>,
}        


impl Character {
    pub fn new(id: CharacterId, name: &str, abilities: Vec<String>, sprite: &str, stats: Stats) -> Self {
        Character {
            id,
            name: String::from(name), 
            state: CharacterState::Default,
            abilities,
            sprite: String::from(sprite), 
            is_friendly: true,
            stats,
            action_points: 0,
            health: stats.max_health,
            buffs: vec![],
        }
    }

    pub fn update(&mut self) {
        self.update_action_points(); 
        // health, mana, ... take_damage()??? 
    }

    pub fn calculate_attack(&self) -> u32 {

        let from_buffs : u32 = self.buffs
            .iter()
            .filter_map(|b| {
                match b {
                    Buff::Attack(val) => Some(*val),
                    _ => None 
                }
            })
            .sum();
            
        self.stats.attack + from_buffs

    }

    pub fn calculate_defense(&self) -> u32 {

        let from_buffs : u32 = self.buffs
            .iter()
            .filter_map(|b| {
                match b {
                    Buff::Defense(val) => Some(*val),
                    _ => None
                }
            })
            .sum();

        self.stats.defense + from_buffs

    }

    pub fn calculate_speed(&self) -> u32 {

        let from_buffs : u32 = self.buffs
            .iter()
            .filter_map(|b| {
                match b {
                    Buff::Speed(val) => Some(*val),
                    _ => None
                }
            })
            .sum();

        self.stats.speed + from_buffs

    }

    pub fn draw(&self,ctx : &Context, canvas: &mut graphics::Canvas, assets: &Assets, position: Point2<f32>) {


        canvas.draw(assets.images.get(&self.sprite).unwrap(), position);
    }

    pub fn action_points_charged(&self) -> bool {
        self.action_points == MAX_ACTION_POINTS
    }

    pub fn update_action_points(&mut self) {
        self.action_points = min(self.action_points + self.calculate_speed(), MAX_ACTION_POINTS);
    }

    pub fn use_action_points(&mut self) {
        self.action_points = 0;
    }

    pub fn take_damage(&mut self, raw_damage: u32) {

        let damage = match self.state {
            CharacterState::Guarding => {
                (raw_damage / 2).saturating_sub(self.calculate_defense())
            },
            _ => raw_damage.saturating_sub(self.calculate_defense())
        };

        self.health = self.health.saturating_sub(damage);
    }
    pub fn heal(&mut self, heal_amount: u32) {
        self.health = min(self.health + heal_amount, self.stats.max_health);
    }
    pub fn status_effect(&mut self) {

    }

    pub fn use_ability()
    {

    }

    pub fn guard()
    {

    }

    pub fn use_item()
    {

    }

    pub fn flee()
    {

    }
}
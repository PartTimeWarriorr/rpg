use std::cmp::{
    min,
    max,
};

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

use crate::{assets::Assets, characters};

use serde::Deserialize;

// for drawing purposes?
#[derive(Clone, Deserialize, Debug)]
pub enum CharacterState {
    Default, 
    Attacking,
    Defending,
    Damaged
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

pub struct Party {
    pub characters: Vec<Character>,
    pub position: Point2<f32>,
}

impl Party {

    pub fn new(position: Point2<f32>) -> Self {
        Party {
            characters: Vec::new(),
            position
        }
    }

    pub fn add_member(&mut self, new_character: Character) {
        self.characters.push(new_character);
    }

    pub fn update_action_points(&mut self) {
        for ch in &mut self.characters {
            ch.update_action_points();
        }
    }

    pub fn draw(&self, ctx : &Context, canvas: &mut graphics::Canvas, assets: &Assets) {

        let mut curr_position = self.position;

        for c in &self.characters {
            c.draw(ctx, canvas, assets, curr_position);
            curr_position = Point2::from_slice(&[curr_position.x, curr_position.y + 50.0]);
        }
    }

    pub fn get_member_by_id(&self, character_id: CharacterId) -> Character {
        if let Some(character) = self.characters.iter().find(|c| c.id == character_id) {
            character.clone()
        } else {
            panic!("No character with id {} found!", character_id);
        }
    }

    pub fn is_character_name(&self, name: &String) -> bool {
        self.characters.iter().any(|ch| ch.name == *name)
    }

}

pub const MAX_ACTION_POINTS : u32 = 500;

pub type CharacterId = u32;

#[derive(Clone, Deserialize, Debug)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub state: CharacterState,
    pub abilities: Vec<String>,
    pub sprite: String,
    pub is_friendly: bool,
    pub stats: Stats,
    #[serde(default)]
    pub action_points: u32,
    pub health: u32,
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
        }
    }

    pub fn update(&mut self) {
        self.update_action_points(); 
        // health, mana, ... take_damage()??? 
    }

    pub fn draw(&self,ctx : &Context, canvas: &mut graphics::Canvas, assets: &Assets, position: Point2<f32>) {


        canvas.draw(assets.character_images.get(&self.sprite).unwrap(), position);
    }

    pub fn action_points_charged(&self) -> bool {
        self.action_points == MAX_ACTION_POINTS
    }

    pub fn update_action_points(&mut self) {
        self.action_points = min(self.action_points + self.stats.speed, MAX_ACTION_POINTS);
    }

    pub fn take_damage(&mut self, damage: u32) {
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
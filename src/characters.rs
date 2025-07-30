use std::cmp::min;

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

use crate::assets::Assets;

#[derive(Clone)]
pub enum CharacterState {
    Default, 
    Attacking,
    Defending,
    Damaged
}
#[derive(Clone, Copy)]
pub struct Stats {
    health: u32,
    attack: u32,
    defense: u32,
    speed: u32,
}

impl Stats {
    pub fn new(health: u32, attack: u32, defense: u32, speed: u32) -> Self {
        Stats { health, attack, defense, speed }
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

}

#[derive(Clone)]
pub struct Ability {
    name: String,
}

impl Ability {
    pub fn new(name : String) -> Self {
        Ability {
            name
        }
    }
}

pub const MAX_ACTION_POINTS : u32 = 500;

pub type CharacterId = u32;

#[derive(Clone)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub state: CharacterState,
    pub abilities: Vec<Ability>,
    pub sprite: String,
    pub is_friendly: bool,
    pub stats: Stats,
    pub action_points: u32
}

impl Character {
    pub fn new(id: CharacterId, name: &str, abilities: Vec<Ability>, sprite: &str, stats: Stats) -> Self {
        Character {
            id,
            name: String::from(name), 
            state: CharacterState::Default,
            abilities,
            sprite: String::from(sprite), 
            is_friendly: true,
            stats,
            action_points: 0,
        }
    }

    pub fn update(&mut self) {
        self.update_action_points(); 
        // health, mana, ... take_damage()??? 
    }

    pub fn draw(&self,ctx : &Context, canvas: &mut graphics::Canvas, assets: &Assets, position: Point2<f32>) {


        canvas.draw(assets.character_images.get(&self.sprite).unwrap(), position);
    }

    pub fn update_action_points(&mut self) {
        self.action_points = min(self.action_points + self.stats.speed, MAX_ACTION_POINTS);
    }

    pub fn take_damage(&mut self) {
        
    }
    pub fn heal(&mut self) {

    }
    pub fn status_effect(&mut self) {

    }
}
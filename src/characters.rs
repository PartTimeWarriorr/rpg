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

    // pub fn update_bars(&mut self) {
    //     for c in &mut self.characters {
    //         c.action_bar.update(c.stats.speed);

    //         // TODO! decouple action points and action bars, place action bars into UI struct
    //         c.action_points += c.stats.speed;
    //     }
    // }

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
// pub struct ActionBar {
//     amount: i32,
//     color: String,
// }

// impl ActionBar {
//     fn new(amount: i32, color: &str) -> Self {

//         ActionBar {
//             amount,
//             color: String::from(color),
//         }
//     }

//     pub fn update(&mut self, update_speed: i32) {
//         // self.amount += 10;
//         // self.amount = min(self.amount + 10, 100);
//         self.amount = min(self.amount + update_speed, 500);
//     }

//     pub fn draw(&self, ctx : &Context, canvas: &mut Canvas, position: Point2<f32>) {

//         let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect { x: 0., y: 0., w: self.amount as f32 / 4.0, h: 10. }, Color::GREEN).unwrap();

//         canvas.draw(&rect, position);

//     }

// }

pub const MAX_ACTION_POINTS : u32 = 500;

pub struct Character {
    pub name: String,
    pub state: CharacterState,
    pub abilities: Vec<Ability>,
    // pub action_bar: ActionBar,
    pub sprite: String,
    pub is_friendly: bool,
    pub stats: Stats,
    pub action_points: u32
}

impl Character {
    pub fn new(name: &str, abilities: Vec<Ability>, sprite: &str, stats: Stats) -> Self {
        Character {
            name: String::from(name), 
            state: CharacterState::Default,
            abilities,
            // action_bar: ActionBar::new(0, "Green"),
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
        // if self.is_friendly
        // {
        //     self.action_bar.draw(ctx, canvas, Point2 { x: position.x, y: position.y + 20.0 });
        // }
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
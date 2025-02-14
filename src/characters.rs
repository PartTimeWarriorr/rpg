

use std::cmp::{
    max,
    min
};

use ggez::{
    Context,
    graphics::{
        self,
        Canvas,
        Color,
        Rect,
    },
    mint::{Point2, Vector2},

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
    speed: i32,
}

impl Stats {
    pub fn new(speed: i32) -> Self {
        Stats {
            speed
        }
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

    pub fn update_bars(&mut self) {
        for c in &mut self.characters {
            c.action_bar.update(c.stats.speed);
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

struct Ability {
    name: String,
}

struct ActionBar {
    amount: i32,
    color: String,
}

impl ActionBar {
    fn new(amount: i32, color: &str) -> Self {

        ActionBar {
            amount,
            color: String::from(color),
        }
    }

    pub fn update(&mut self, update_speed: i32) {
        // self.amount += 10;
        // self.amount = min(self.amount + 10, 100);
        self.amount = min(self.amount + update_speed, 500);
    }

    pub fn draw(&self, ctx : &Context, canvas: &mut Canvas, position: Point2<f32>) {

        let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect { x: 0., y: 0., w: self.amount as f32 / 4.0, h: 10. }, Color::GREEN).unwrap();

        canvas.draw(&rect, position);

    }

    // pub fn draw(&self, canvas: &mut graphics::Canvas) {

    //     // let rect = graphics::Mesh::new_rectangle(gfx, mode, bounds, color)
    //     // canvas.draw()
    // }
}


pub struct Character {
    pub name: String,
    pub state: CharacterState,
    pub abilities: Vec<Ability>,
    pub action_bar: ActionBar,
    pub sprite: String,
    pub is_friendly: bool,
    pub stats: Stats,
}

impl Character {
    pub fn new(name: &str, sprite: &str, stats: Stats) -> Self {
        Character {
            name: String::from(name), 
            state: CharacterState::Default,
            abilities: vec![],
            action_bar: ActionBar::new(0, "Green"),
            sprite: String::from(sprite), 
            is_friendly: true,
            stats,
        }
    }

    pub fn draw(&self,ctx : &Context, canvas: &mut graphics::Canvas, assets: &Assets, position: Point2<f32>) {


        // canvas.draw(&assets.character_image, Vec2::new(self.pos.x, self.pos.y));
        // println!("{:?}", assets.character_images.get(&String::from("cad.png")));
        // canvas.draw(assets.character_images.get("cad").unwrap(), Vec2::new(self.pos.x, self.pos.y));
        canvas.draw(assets.character_images.get(&self.sprite).unwrap(), position);
        if self.is_friendly
        {
            self.action_bar.draw(ctx, canvas, Point2 { x: position.x, y: position.y + 20.0 });
        }
    }
}
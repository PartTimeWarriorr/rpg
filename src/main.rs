#![allow(clippy::unnecessary_wraps)]

use ggez::{
    context::Has, event, glam::*, graphics::{self, Canvas, Color, Image, Rect}, mint::{Point2, Vector2}, Context, GameResult
};

use rpg::assets::Assets;
use rpg::menu::MenuNode;

use std::{char, collections::HashMap, env, fmt::format};
use std::path;

use std::cmp::{max, min};

struct MainState {
    // character: Character,
    assets: Assets, 
    friendly_party: Party,
    enemy_party: Party,
    game_state: GameState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        // let circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     vec2(0., 0.),
        //     100.0,
        //     2.0,
        //     Color::WHITE,
        // )?;

        let slow_stats = Stats::new(1);
        let fast_stats = Stats::new(2);
        let v_fast_stats = Stats::new(3);


        let mut fp = Party::new(Point2 { x: 100.0, y: 100.0 });
        let mut ep = Party::new(Point2 { x: 400.0, y: 100.0 });

        let character = Character::new("hero", "char_1", fast_stats.clone());
        let character_2 = Character::new("hero_2", "char_2", v_fast_stats.clone());
        let character_3 = Character::new("hero_3", "char_3", slow_stats.clone());
        let enemy = Character::new("orc", "enem_1", slow_stats.clone());
        let enemy_2 = Character::new("orc_2", "enem_2", fast_stats.clone());

        fp.add_member(character);
        fp.add_member(character_2);
        fp.add_member(character_3);
        ep.add_member(enemy);
        ep.add_member(enemy_2);

        let game_state = GameState::Battle;

        Ok(MainState {assets, friendly_party: fp, enemy_party: ep, game_state})
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        // self.action_bar.update();
        // self.character.action_bar.update();
        
        match &self.game_state {
            GameState::Battle => {
                self.friendly_party.update_bars();
                self.enemy_party.update_bars();
            },
            GameState::Overworld => {}
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &self.game_state {
            GameState::Battle => {

                let mut canvas =
                    graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

                canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 500.0, 500.0));
                canvas.set_sampler(graphics::Sampler::nearest_clamp());
                // canvas.draw(&self.circle, Vec2::new(self.pos_x, 380.0));
                // canvas.draw(&self.rect, Vec2::new(0.,0.));
                // self.character.draw(&mut canvas, &self.assets);
                // self.action_bar.draw(ctx, &mut canvas);
                // self.character.draw(&ctx, &mut canvas, &self.assets);
                self.friendly_party.draw(ctx, &mut canvas, &self.assets);
                self.enemy_party.draw(ctx, &mut canvas, &self.assets);

                canvas.finish(ctx)?;
            },
            GameState::Overworld => {}
        }

        Ok(())
    }
}

pub enum GameState {
    Overworld,
    Battle,
}

pub enum CharacterState {
    Default, 
    Attacking,
    Defending,
    Damaged
}
#[derive(Clone, Copy)]
struct Stats {
    speed: i32,
}

impl Stats {
    fn new(speed: i32) -> Self {
        Stats {
            speed
        }
    }
}

struct Party {
    characters: Vec<Character>,
    position: Point2<f32>,
}

impl Party {

    fn new(position: Point2<f32>) -> Self {
        Party {
            characters: Vec::new(),
            position
        }
    }

    fn add_member(&mut self, new_character: Character) {
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
        canvas.draw(assets.character_images.get(&self.sprite).unwrap(), Vec2::new(position.x, position.y));
        if self.is_friendly
        {
            self.action_bar.draw(ctx, canvas, Point2 { x: position.x, y: position.y + 20.0 });
        }
    }
}

// pub fn main() -> GameResult {

    // let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    // let (mut ctx, event_loop) = cb.build()?;

    // if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    //     let mut path = path::PathBuf::from(manifest_dir);
    //     path.push("resources");
    //     ctx.fs.mount(&path, true);
    // }

    // let state = MainState::new(&mut ctx)?;

    // event::run(ctx, event_loop, state)
// }

pub fn main() {

    let mut root = MenuNode::new("root");
    
    let names_vec = vec!["Fight", "Guard", "Item", "Flee"];

    for name in names_vec {
        let mut ch = MenuNode::new(name);
        let chch = MenuNode::new("i'm a child");
        ch.add_child(chch);
        root.add_child(ch);
    }


    let result = format!("{}", root);
    print!("{}", result);

}
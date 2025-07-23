#![allow(clippy::unnecessary_wraps)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use ggez::{
    event, graphics::{self, Canvas, Color, DrawParam, Drawable, Image, Rect}, input::{self, keyboard::{KeyCode, KeyInput}, }, mint::{Point2, Vector2},  Context, GameResult
};

use rpg::assets::Assets;
use rpg::menu::*;
use rpg::ui::Ui;
use rpg::characters::*;

use std::env;
use std::path;


struct MainState {
    // character: Character,
    assets: Assets, 
    friendly_party: Party,
    enemy_party: Party,
    game_state: GameState,
    ui: Ui,
    // background: Image
}

const FRIENDLY_PARTY_POSITION : Point2<f32> = Point2 { x : 100.0, y: 200.0}; 
const ENEMY_PARTY_POSITION : Point2<f32> = Point2 { x : 400.0, y: 200.0}; 

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;

        let slow_stats = Stats::new(1);
        let fast_stats = Stats::new(2);
        let v_fast_stats = Stats::new(3);

        let mut fp = Party::new(FRIENDLY_PARTY_POSITION);
        let mut ep = Party::new(ENEMY_PARTY_POSITION);

        let abilities = vec![Ability::new(String::from("a1")),Ability::new(String::from("a2"))];

        let character = Character::new("hero", abilities.clone(), "char_1", fast_stats.clone());
        let character_2 = Character::new("hero_2", abilities.clone(), "char_2", v_fast_stats.clone());
        let character_3 = Character::new("hero_3", abilities.clone(),"char_3", slow_stats.clone());
        let enemy = Character::new("orc",abilities.clone(), "enem_1", slow_stats.clone());
        let enemy_2 = Character::new("orc_2",abilities.clone(), "enem_2", fast_stats.clone());

        fp.add_member(character);
        fp.add_member(character_2);
        fp.add_member(character_3);
        ep.add_member(enemy);
        ep.add_member(enemy_2);

        let game_state = GameState::Battle;

        // let root = MenuNode::new("root");

        // let character_nodes : Vec<MenuNodeRef> = fp.characters
        //     .iter()
        //     .map(|c| MenuNode::new(&c.name))
        //     .collect();

        // let action_nodes : Vec<MenuNodeRef> = vec!["Fight", "Guard", "Item", "Flee"]
        //     .iter()
        //     .map(|a| MenuNode::new(*a))
        //     .collect();

        // let ability_nodes : Vec<MenuNodeRef> = vec!["a1", "a2", "a3", "a4"]
        //     .iter()
        //     .map(|ab| MenuNode::new(*ab))
        //     .collect();
        
        // for character in &character_nodes {

        //     for action in &action_nodes {

        //         if action.borrow().name == "Fight" {
        //             for ability in &ability_nodes {
        //                 MenuNode::add_child(&action, &ability);
        //             }
        //         }

        //         MenuNode::add_child(&character, &action);
        //     }

        //     MenuNode::add_child(&root, &character);
        // }

        // let names_vec = fp.characters.iter().map(|ch| ch.name.clone()).collect::<Vec<String>>();
        // let actions_vec = vec!["Fight", "Guard", "Item", "Flee"];

        // for name in names_vec {
        //     let child = MenuNode::new(&name);
        //     for action in &actions_vec {
        //         let chch = MenuNode::new(&action);
        //         if *action == "Fight" {
        //             for ability in &["a1", "a2", "a3", "a4"] {
        //                 let chchch = MenuNode::new(&ability);
        //                 MenuNode::add_child(&chch, &chchch);
        //             }
        //         }
        //         MenuNode::add_child(&child, &chch);
        //     }
        //     MenuNode::add_child(&root, &child);
        // }

        // dbg!("{}", &root);

        // let mut ui = Ui::new(root);
        // ui.load_menu();

        let mut menu = Menu::new();

        for hero_name in vec!["hero_1", "hero_2", "hero_3", "hero_4"] {
            menu.insert_at("root", hero_name);

            for action in vec!["Fight", "Guard", "Item", "Flee"] {
                menu.insert_at(hero_name, action);

            }

            for ability in vec!["a1", "a2", "a3", "a4"] {
                menu.insert_at_path(&["root", hero_name, "Fight"], ability);
            }
        }        

        let mut ui = Ui::new(menu);
        ui.load_menu();

        Ok(MainState {assets, friendly_party: fp, enemy_party: ep, game_state, ui})
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

        if _ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            self.ui.change_selection(1);
            self.ui.load_menu();
        }
        if _ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            self.ui.change_selection(-1);
            self.ui.load_menu();
        }

        if _ctx.keyboard.is_key_just_pressed(KeyCode::Z) {

            // TODO! check if it's a character name and if the character is ready to act

            self.ui.select();
            self.ui.load_menu();
        }

        if _ctx.keyboard.is_key_just_pressed(KeyCode::X) {
            self.ui.go_back();
            self.ui.load_menu();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let image = self.assets.character_images.get("battle_bg").unwrap(); 
        image.draw(&mut canvas, DrawParam::default());

        match &self.game_state {
            GameState::Battle => {

                canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 500.0, 500.0));
                canvas.set_sampler(graphics::Sampler::nearest_clamp());

                // canvas.draw(&self.circle, Vec2::new(self.pos_x, 380.0));
                // canvas.draw(&self.rect, Vec2::new(0.,0.));
                // self.character.draw(&mut canvas, &self.assets);
                // self.action_bar.draw(ctx, &mut canvas);
                // self.character.draw(&ctx, &mut canvas, &self.assets);
                self.friendly_party.draw(ctx, &mut canvas, &self.assets);
                self.enemy_party.draw(ctx, &mut canvas, &self.assets);

                self.ui.draw(&mut canvas, DrawParam::default());

                canvas.finish(ctx)?;
            },
            GameState::Overworld => {

            }
        }

        Ok(())
    }
}

pub enum GameState {
    Overworld,
    Battle,
}

pub fn main() -> GameResult {

    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.fs.mount(&path, true);
    }

    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}

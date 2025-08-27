#![allow(clippy::unnecessary_wraps)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use ggez::{
    context::Has, event, graphics::{self, Canvas, Color, DrawParam, Drawable, Image, Rect}, input::{self, keyboard::{KeyCode, KeyInput}, }, mint::{Point2, Vector2}, Context, GameResult
};

use rpg::{ability::Ability, action::{Action, PendingAction}, assets::Assets, ui};
use rpg::menu::*;
use rpg::ui::*;
use rpg::characters::*;

use std::{collections::HashMap, env, hash::RandomState, iter::Map};
use std::path;

use ordermap::OrderMap;


struct MainState {
    assets: Assets, 
    friendly_party: Party,
    enemy_party: Party,
    game_state: GameState,
    ui: Ui,
    // character_uis: HashMap<CharacterId, Bars>,
    character_uis: OrderMap<CharacterId, Bars, RandomState>,
    current_action: PendingAction,
    player_abilities: Vec<String>,
    character_names: Vec<String>,
}

const FRIENDLY_PARTY_POSITION : Point2<f32> = Point2 { x : 100.0, y: 200.0}; 
const ENEMY_PARTY_POSITION : Point2<f32> = Point2 { x : 400.0, y: 200.0}; 

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;

        let slow_stats = Stats::new(500,1,1,1);
        let fast_stats = Stats::new(500,2,2,2);
        let v_fast_stats = Stats::new(500,3,3,3);

        let mut fp = Party::new(FRIENDLY_PARTY_POSITION);
        let mut ep = Party::new(ENEMY_PARTY_POSITION);

        let abilities = vec![String::from("a1"), String::from("a2")];

        let character = Character::new(0, "hero", abilities.clone(), "char_1", fast_stats.clone());
        let character_2 = Character::new(1, "hero_2", abilities.clone(), "char_2", v_fast_stats.clone());
        let character_3 = Character::new(2, "hero_3", abilities.clone(),"char_3", slow_stats.clone());
        let enemy = Character::new(3, "orc",abilities.clone(), "enem_1", slow_stats.clone());
        let enemy_2 = Character::new(4, "orc_2",abilities.clone(), "enem_2", fast_stats.clone());


        fp.add_member(character);
        fp.add_member(character_2);
        fp.add_member(character_3);
        ep.add_member(enemy);
        ep.add_member(enemy_2);

        let player_abilities = fp.characters
            .iter()
            .flat_map(|ch| ch.abilities.clone())
            .collect();

        let character_names = fp.characters
            .iter()
            .chain(&ep.characters)
            .map(|ch| ch.name.clone())
            .collect::<Vec<String>>();

        let game_state = GameState::Battle;

        let mut menu = Menu::new();

        for hero_name in vec!["hero", "hero_2", "hero_3"] {
            // menu.insert_at("root", hero_name);
            menu.insert_at_path(&["root"], hero_name);

            for action in vec!["Fight", "Guard", "Item", "Flee"] {
                // menu.insert_at(hero_name, action);
                menu.insert_at_path(&["root", hero_name], action);
            }

            // TODO: Ewwwwww
            for ch_name in &character_names {
                menu.insert_at_path(&["root", hero_name, "Item"], &ch_name);
            }

            for ability in vec!["a1", "a2"] {
                menu.insert_at_path(&["root", hero_name, "Fight"], ability);

                // TODO: Ewwwwww
                for ch_name in &character_names {
                    menu.insert_at_path(&["root", hero_name, "Fight", ability], &ch_name);
                }
            }
        }        

        dbg!(&menu);

        let mut ui = Ui::new(menu);
        ui.load_menu();
        
        // let character_uis : HashMap<CharacterId, Bars> = fp.characters
        //     .iter()
        //     .map(|ch| (ch.id, Bars::new(ctx)))
        //     .collect();

        let character_uis = fp.characters
            .iter()
            .map(|ch| (ch.id, Bars::new(ctx)))
            .collect::<OrderMap<CharacterId, Bars>>();

        let current_action = Action::new();


        dbg!(&character_names);

        // dbg!(&player_abilities);

        Ok(MainState {assets, friendly_party: fp, enemy_party: ep, game_state, ui, character_uis, current_action, player_abilities, character_names})
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        match &self.game_state {
            GameState::Battle => {
                
                // self.friendly_party.update_action_points();
                self.friendly_party.characters.iter_mut().find(|ch| ch.name == "hero").unwrap().action_points = 100;
                self.friendly_party.characters.iter_mut().find(|ch| ch.name == "hero_2").unwrap().action_points = 200;
                self.friendly_party.characters.iter_mut().find(|ch| ch.name == "hero_3").unwrap().action_points = 300;
                self.character_uis
                    .iter_mut()
                    .for_each(|(ch_id,bars)| {
                        bars.health_bar.update(_ctx, self.friendly_party.get_member_by_id(*ch_id).health as f32, HERO_HP_COLOR);
                        bars.mana_bar.update(_ctx, self.friendly_party.get_member_by_id(*ch_id).stats.defense as f32, HERO_MP_COLOR);
                        bars.action_bar.update(_ctx, self.friendly_party.get_member_by_id(*ch_id).action_points as f32, HERO_AP_COLOR);
                    });

                self.enemy_party.update_action_points();
                
                
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

            self.ui.select();
            self.ui.load_menu();

            // TODO: check if it's a character name and if the character is ready to act
            let curr_selection : String = self.ui.get_curr_data();

            match self.ui.menu_state {
                MenuState::ChooseActor => {
                    if self.friendly_party.is_character_name(&curr_selection) {

                        if self.friendly_party.characters.iter().find(|ch| ch.name == *curr_selection).unwrap().action_points_charged() {
                            self.current_action.actor(curr_selection);
                            self.ui.menu_state = MenuState::ChooseActionType;
                        } else {
                            self.ui.go_back();
                            self.ui.load_menu();
                            println!("Cannot act yet!, {}", self.friendly_party.characters.iter().find(|ch| ch.name == *curr_selection).unwrap().action_points);
                        }

                    } 
                },
                MenuState::ChooseActionType => {
                    if curr_selection == "Fight" {
                        self.current_action.action_type(rpg::action::ActionType::Fight);
                        self.ui.menu_state = MenuState::ChooseAbility;
                    } else if curr_selection == "Guard" {
                        self.current_action.action_type(rpg::action::ActionType::Guard);
                        self.ui.go_back();
                        self.ui.go_back();
                        self.ui.load_menu();
                        self.ui.menu_state = MenuState::ChooseActor;
                    } else if curr_selection == "Item" {
                        self.current_action.action_type(rpg::action::ActionType::Item);
                        self.ui.menu_state = MenuState::ChooseTarget;
                    } else if curr_selection == "Flee" {
                        self.current_action.action_type(rpg::action::ActionType::Flee);
                        self.ui.go_back();
                        self.ui.go_back();
                        self.ui.load_menu();
                        self.ui.menu_state = MenuState::ChooseActor;
                    }

                },
                MenuState::ChooseAbility => {
                    if self.player_abilities.iter().any(|ab_name| *ab_name == curr_selection) {
                        self.current_action.ability(curr_selection);
                    }
                    self.ui.menu_state = MenuState::ChooseTarget;
                },
                MenuState::ChooseTarget => {
                    if self.character_names.iter().any(|ch_name| *ch_name == curr_selection) {
                        self.current_action.target(curr_selection);
                    }
                    self.ui.go_back();
                    self.ui.go_back();
                    self.ui.go_back();
                    self.ui.go_back();
                    self.ui.load_menu();

                    // Works!!
                    // Action gets built and resolved
                    let complete_action = self.current_action.build(); 
                    println!("Complete action: {:?}", complete_action);
                    println!("Current action: {:?}", self.current_action);
                    println!("Orc stats: {:?}", self.enemy_party.characters.iter().find(|ch| ch.name == "orc_2").unwrap().health);
                    complete_action.resolve(&mut self.friendly_party, &mut self.enemy_party);

                    println!("Orc stats: {:?}", self.enemy_party.characters.iter().find(|ch| ch.name == "orc_2").unwrap().health);

                    self.ui.menu_state = MenuState::ChooseActor;
                }
            }

                        // dbg!(&self.current_action);
                        // dbg!(&self.ui.menu_state);
        }

        if _ctx.keyboard.is_key_just_pressed(KeyCode::X) {
            self.ui.go_back();
            self.ui.load_menu();

            match self.ui.menu_state {
                MenuState::ChooseTarget => {
                    self.ui.menu_state = MenuState::ChooseAbility;
                }
                MenuState::ChooseAbility => {
                    self.ui.menu_state = MenuState::ChooseActionType;
                },
                MenuState::ChooseActionType => {
                    self.ui.menu_state = MenuState::ChooseActor;
                },
                MenuState::ChooseActor => {},
            }
                        dbg!(&self.current_action);
                        dbg!(&self.ui.menu_state);
        }

        if _ctx.keyboard.is_key_just_pressed(KeyCode::D) {
            self.friendly_party.characters.get_mut(0).unwrap().take_damage(10);
        }

        // if _ctx.keyboard.is_key_just_pressed(KeyCode::H) {
        //     self.friendly_party.characters.get_mut(0).unwrap().heal(10);
        // }

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

                self.friendly_party.draw(ctx, &mut canvas, &self.assets);
                self.enemy_party.draw(ctx, &mut canvas, &self.assets);

                self.ui.draw(&mut canvas, DrawParam::default());
                draw_character_uis(&mut canvas, &self.character_uis);

                canvas.finish(ctx)?;
            },
            GameState::Overworld => {
                canvas.finish(ctx)?;
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

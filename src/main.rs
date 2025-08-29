#![allow(clippy::unnecessary_wraps)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use ggez::{
    context::Has, event, graphics::{self, Canvas, Color, DrawParam, Drawable, Image, Rect}, 
    input::{self, keyboard::{KeyCode}, }, 
    mint::{Point2, Vector2}, Context, GameResult
};

use rand::seq::IndexedRandom;
use rpg::{ability::{Ability, AbilityType}, action::{Action, PendingAction}, assets::Assets};
use rpg::menu::*;
use rpg::ui::*;
use rpg::characters::*;

use std::{path, env, hash::RandomState, fs::File, io::BufReader}; 

use ordermap::OrderMap;

struct MainState {
    assets: Assets, 
    friendly_party: Party,
    enemy_party: Party,
    game_state: GameState,
    ui: Ui,
    character_uis: OrderMap<CharacterId, Bars, RandomState>,
    current_action: PendingAction,
    player_abilities: Vec<String>,
    character_names: Vec<String>,
    abilities: Vec<Ability>,
    dialogue_box: DialogueBox,
}

const FRIENDLY_PARTY_POSITION : Point2<f32> = Point2 { x : 100.0, y: 200.0}; 
const ENEMY_PARTY_POSITION : Point2<f32> = Point2 { x : 400.0, y: 200.0}; 

pub fn load_friendly_party() -> Party {
    let file = File::open("src/friendly_party.json").expect("Config file not found: friendly_party");
    let rdr = BufReader::new(file);

    match serde_json::from_reader(rdr) {
        Ok(json) => Party::new(json, FRIENDLY_PARTY_POSITION),
        Err(err) => panic!("Error when parsing json: {}", err)
    } 
}

pub fn load_enemies() ->  Vec<Vec<Character>> {
    let file = File::open("src/enemies.json").expect("Config file not found: enemies");
    let rdr = BufReader::new(file);

    match serde_json::from_reader(rdr) {
        Ok(json) => json,
        Err(err) => panic!("Error when parsing json: {}", err)
    } 
}

pub fn load_abilities() -> Vec<Ability> {
    let file = File::open("src/abilities.json").expect("Config file not found: abilities");
    let rdr = BufReader::new(file);

    match serde_json::from_reader(rdr) {
        Ok(json) => json,
        Err(err) => panic!("Error when parsing json: {}", err)
    } 
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;

        let fp = load_friendly_party();
        let all_enemies = load_enemies();
        let ep = Party::new(all_enemies.get(0).unwrap().clone(), ENEMY_PARTY_POSITION);

        let player_names : Vec<String> = fp.characters.iter().map(|ch| ch.name.clone()).collect();
        let enemy_names : Vec<String> = ep.characters.iter().map(|ch| ch.name.clone()).collect();

        let abilities = load_abilities();

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

        for hero_name in fp.characters.iter().map(|ch| ch.name.clone()) {
            menu.insert_at_path(&["root"], &hero_name);

            for action in vec!["Fight", "Guard", "Item", "Flee"] {
                // menu.insert_at(hero_name, action);
                menu.insert_at_path(&["root", &hero_name], action);
            }

            for ch_name in &character_names {
                menu.insert_at_path(&["root", &hero_name, "Item"], &ch_name);
            } 

            for ability in fp.characters.iter().find(|ch| ch.name == hero_name).unwrap().abilities.clone() {
                menu.insert_at_path(&["root", &hero_name, "Fight"], &ability);
                
                let current_ability = abilities.iter().find(|ab| ab.name == ability).expect("Unknown ability");

                match current_ability.ability_type {
                    AbilityType::Damage | AbilityType::Status => {
                        // Damage and Status abilities have only enemies as targets
                        for ch_name in &enemy_names {
                            menu.insert_at_path(&["root", &hero_name, "Fight", &ability], &ch_name);
                        }
                    },
                    AbilityType::Heal | AbilityType::Buff => {
                        // Heal and Buff abilities have only friends as targets
                        for ch_name in &player_names {
                            menu.insert_at_path(&["root", &hero_name, "Fight", &ability], &ch_name);
                        }
                    }
                }
            }
        }


        let mut ui = Ui::new(menu);
        ui.load_menu();
        
        let character_uis = fp.characters
            .iter()
            .map(|ch| (ch.id, Bars::new(ctx)))
            .collect::<OrderMap<CharacterId, Bars>>();

        let current_action = Action::new();

        let dialogue_box = DialogueBox::new();

        Ok(MainState {assets, friendly_party: fp, enemy_party: ep, game_state, ui, character_uis, current_action, player_abilities, character_names, abilities, dialogue_box})
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        match &self.game_state {
            GameState::Battle => {
                
                self.friendly_party.update_action_points();
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
                            self.ui.reset();
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
                        self.ui.reset();
                        self.ui.load_menu();
                        let complete_action = self.current_action.build(&self.abilities);
                        complete_action.resolve(&mut self.friendly_party, &mut self.enemy_party, &mut self.dialogue_box);
                        // self.friendly_party.characters.iter_mut().find(|ch| ch.name == complete_action.actor).unwrap().use_action_points();
                        self.ui.menu_state = MenuState::ChooseActor;
                    } else if curr_selection == "Item" {
                        self.current_action.action_type(rpg::action::ActionType::Item);
                        self.ui.menu_state = MenuState::ChooseTarget;
                    } else if curr_selection == "Flee" {
                        self.current_action.action_type(rpg::action::ActionType::Flee);
                        self.ui.reset();
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
                    self.ui.reset();
                    self.ui.load_menu();

                    // Works!!
                    // Action gets built and resolved
                    let complete_action = self.current_action.build(&self.abilities); 

                    self.friendly_party.characters.iter_mut().find(|ch| ch.name == complete_action.actor).unwrap().use_action_points();
                
                    complete_action.resolve(&mut self.friendly_party, &mut self.enemy_party, &mut self.dialogue_box);

                    println!("Orc stats: {:?}", self.enemy_party.characters.iter().find(|ch| ch.name == "orc_2").unwrap().health);

                    self.ui.menu_state = MenuState::ChooseActor;
                }
            }
        }

        if let Some(ch) = self.enemy_party.characters.iter_mut().find(|ch| ch.action_points == MAX_ACTION_POINTS) {
            println!("{} the Destroyer", ch.name);
            ch.use_action_points();
            let mut rng = rand::rng();
            let enemy_action = PendingAction::new()
                .actor(ch.name.clone())
                .action_type(rpg::action::ActionType::Fight)
                .ability(ch.abilities.choose(&mut rng).unwrap().clone()) 
                .target(self.friendly_party.characters.choose(&mut rng).unwrap().name.clone())
                .build(&self.abilities)
                .resolve(&mut self.enemy_party, &mut self.friendly_party, &mut self.dialogue_box);

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

                self.friendly_party.draw(ctx, &mut canvas, &self.assets);
                self.enemy_party.draw(ctx, &mut canvas, &self.assets);

                self.ui.draw(&mut canvas, DrawParam::default());
                draw_character_uis(&mut canvas, &self.character_uis);
                self.dialogue_box.draw(&mut canvas, DrawParam::default());

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

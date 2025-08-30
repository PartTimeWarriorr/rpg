use ggez::{
    context::Has, event, graphics::{self, Canvas, Color, DrawParam, Drawable, Image, Rect, Text}, input::{self, keyboard::KeyCode, }, mint::{Point2, Vector2}, timer::TimeContext, Context, GameError, GameResult
};

use rand::seq::{IndexedRandom, IteratorRandom};
use crate::ability::*;
use crate::assets::*;
use crate::action::*;
use crate::menu::*;
use crate::ui::*;
use crate::characters::*;
use crate::load::*;

use std::{path, env, hash::RandomState, fs::File, io::BufReader}; 

use ordermap::OrderMap;

enum GameState {
    OverworldState,
    BattleState,
    WinState,
    LoseState,
}

struct Battle {
    pub character_names: Vec<String>,
    pub player_names: Vec<String>,
    pub enemy_names: Vec<String>,
    pub enemy_party: Party,
    pub action_menu: Ui,
    pub current_action: PendingAction,
    pub player_uis: OrderMap<CharacterId, Bars, RandomState>,
    pub dialogue_box: DialogueBox,
}

impl Battle {

    pub fn new(friendly_party: &Party, all_enemies: &mut Vec<Vec<Character>>, ctx: &mut Context, abilities: &Vec<Ability>) -> Self {

        let enemy_party = Party::new(all_enemies.remove(0).clone(), ENEMY_PARTY_POSITION);

        let character_names = friendly_party.characters
            .iter()
            .chain(&enemy_party.characters)
            .map(|ch| ch.name.clone())
            .collect::<Vec<String>>();

        let player_uis = friendly_party.characters
            .iter()
            .map(|ch| (ch.id, Bars::new(ctx)))
            .collect::<OrderMap<CharacterId, Bars>>();


        let player_names : Vec<String> = friendly_party.characters.iter().map(|ch| ch.name.clone()).collect();
        let enemy_names : Vec<String> = enemy_party.characters.iter().map(|ch| ch.name.clone()).collect();

        Battle {
            character_names: character_names.clone(),
            player_names: player_names.clone(),
            enemy_names: enemy_names.clone(),
            enemy_party,
            action_menu: Battle::load_action_menu(&friendly_party, &character_names, &player_names, &enemy_names, &abilities),
            current_action: Action::new(),
            player_uis,
            dialogue_box: DialogueBox::new(),
        }
    }

    pub fn load_action_menu(fp: &Party, character_names: &Vec<String>, player_names: &Vec<String>, enemy_names: &Vec<String>, abilities: &Vec<Ability>) -> Ui {

        let mut menu = Menu::new();

        for hero_name in fp.characters.iter().map(|ch| ch.name.clone()) {
            menu.insert_at_path(&["root"], &hero_name);

            for action in vec!["Fight", "Guard", "Item", "Flee"] {
                menu.insert_at_path(&["root", &hero_name], action);
            }

            for ch_name in character_names {
                menu.insert_at_path(&["root", &hero_name, "Item"], &ch_name);
            } 

            for ability in fp.characters.iter().find(|ch| ch.name == hero_name).unwrap().abilities.clone() {
                menu.insert_at_path(&["root", &hero_name, "Fight"], &ability);
                
                let current_ability = abilities.iter().find(|ab| ab.name == ability).expect("Unknown ability");

                match current_ability.ability_type {
                    AbilityType::Damage | AbilityType::Status => {
                        // Damage and Status abilities have only enemies as targets
                        for ch_name in enemy_names {
                            menu.insert_at_path(&["root", &hero_name, "Fight", &ability], &ch_name);
                        }
                    },
                    AbilityType::Heal | AbilityType::Buff => {
                        // Heal and Buff abilities have only friends as targets
                        for ch_name in player_names {
                            menu.insert_at_path(&["root", &hero_name, "Fight", &ability], &ch_name);
                        }
                    }
                }
            }
        }


        let mut ui = Ui::new(menu);
        ui.load_menu();
        ui
    }
}

struct Overworld {

}

impl Overworld {
    pub fn new() -> Self {
        Overworld {  }
    }
}

pub struct MainState {
    assets: Assets, 
    game_state: GameState,
    abilities: Vec<Ability>,
    all_enemies: Vec<Vec<Character>>,
    curr_battle: Battle,
    overworld: Overworld,
    friendly_party: Party,
    player_abilities: Vec<String>,
    win_message: Text,
    lose_message: Text,
    error_message: Text,
}

const FRIENDLY_PARTY_POSITION : Point2<f32> = Point2 { x : 100.0, y: 200.0}; 
const ENEMY_PARTY_POSITION : Point2<f32> = Point2 { x : 400.0, y: 200.0}; 


impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {

        let abilities = load_abilities();
        let mut all_enemies = load_enemies(); 
        let friendly_party = Party::new(load_friendly_party(), FRIENDLY_PARTY_POSITION);

        let player_abilities = friendly_party.characters
            .iter()
            .flat_map(|ch| ch.abilities.clone())
            .collect();

        let battle = Battle::new(&friendly_party, &mut all_enemies, ctx, &abilities);
        
        Ok(
            MainState {
                assets: Assets::new(ctx)?,
                game_state: GameState::BattleState,
                abilities,
                all_enemies,
                curr_battle: battle,
                overworld: Overworld::new(),
                friendly_party,
                player_abilities,
                win_message: Text::new("You win! Press Enter for next battle."),
                lose_message: Text::new("All your party members got killed. Game over! (Press Esc to exit)"),
                error_message: Text::new(""),
            }
        )
    }

    fn load_next_battle(&mut self, ctx: &mut Context) {
        
        let next_battle = Battle::new(&self.friendly_party, &mut self.all_enemies, ctx, &self.abilities);
        self.curr_battle = next_battle;

    }

    fn get_dead_characters(&self) -> Vec<Character> {
        self.friendly_party.characters
            .iter()
            .chain(&self.curr_battle.enemy_party.characters)
            .filter(|ch| ch.health == 0)
            .map(|ch| ch.to_owned())
            .collect()
    }

    fn cleanup_dead_characters(&mut self) {

        let dead_vec = self.get_dead_characters();

        // Simply remove characters with 0 health
        self.friendly_party.characters
            .retain(|ch| ch.health > 0);

        self.curr_battle.enemy_party.characters
            .retain(|ch| ch.health > 0);

        // Remove action menu nodes with dead character names
        dead_vec
            .iter()
            .for_each(|dead_ch| 
                self.curr_battle.action_menu.remove_nodes_with_data(dead_ch.name.clone()));

        self.curr_battle.action_menu.load_menu();
        
        // Remove dead character ids from bar uis
        let dead_ids = dead_vec
            .iter()
            .map(|ch| ch.id)
            .collect::<Vec<CharacterId>>();

        self.curr_battle.player_uis
            .retain(|k, v| !dead_ids.contains(k));

    }

    fn friendly_party_died(&self) -> bool {
        self.friendly_party.characters.is_empty()
    }

    fn enemy_party_died(&self) -> bool {
        self.curr_battle.enemy_party.characters.is_empty()
    }

    fn update_player_uis(&mut self, _ctx: &mut Context) {
        self.curr_battle.player_uis
            .iter_mut()
            .for_each(|(ch_id,bars)| {
                bars.health_bar.update(_ctx, self.friendly_party.get_member_by_id(*ch_id).health as f32, HERO_HP_COLOR);
                bars.mana_bar.update(_ctx, self.friendly_party.get_member_by_id(*ch_id).stats.defense as f32, HERO_MP_COLOR);
                bars.action_bar.update(_ctx, self.friendly_party.get_member_by_id(*ch_id).action_points as f32, HERO_AP_COLOR);
            });
    } 

    fn navigate_menu(&mut self, _ctx: &mut Context) {
        if _ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            self.curr_battle.action_menu.change_selection(1);
            self.curr_battle.action_menu.load_menu();
        }
        if _ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            self.curr_battle.action_menu.change_selection(-1);
            self.curr_battle.action_menu.load_menu();
        }
    }

    fn select_player_action(&mut self, _ctx: &mut Context) {

        // Progress action selection
        if _ctx.keyboard.is_key_just_pressed(KeyCode::Z) {

            self.curr_battle.action_menu.select();
            self.curr_battle.action_menu.load_menu();

            let curr_selection : String = self.curr_battle.action_menu.get_curr_data();

            match self.curr_battle.action_menu.menu_state {
                MenuState::ChooseActor => {
                    if self.friendly_party.is_character_name(&curr_selection) {

                        if self.friendly_party.characters.iter().find(|ch| ch.name == *curr_selection).unwrap().action_points_charged() {
                            self.curr_battle.current_action.actor(curr_selection);
                            self.curr_battle.action_menu.menu_state = MenuState::ChooseActionType;
                        } else {
                            self.curr_battle.action_menu.reset();
                            self.curr_battle.action_menu.load_menu();
                        }

                    } 
                },
                MenuState::ChooseActionType => {

                    if curr_selection == "Fight" {

                        self.curr_battle.current_action.action_type(ActionType::Fight);
                        self.curr_battle.action_menu.menu_state = MenuState::ChooseAbility;

                    } else if curr_selection == "Guard" {

                        self.curr_battle.current_action.action_type(ActionType::Guard);
                        self.curr_battle.action_menu.reset();
                        self.curr_battle.action_menu.load_menu();

                        let complete_action = self.curr_battle.current_action.build(&self.abilities);
                        self.friendly_party.characters.iter_mut().find(|ch| ch.name == complete_action.actor).unwrap().use_action_points();
                        complete_action.resolve(&mut self.friendly_party, &mut self.curr_battle.enemy_party, &mut self.curr_battle.dialogue_box);

                        self.curr_battle.action_menu.menu_state = MenuState::ChooseActor;

                    } else if curr_selection == "Item" {

                        self.curr_battle.current_action.action_type(ActionType::Item);
                        self.curr_battle.action_menu.menu_state = MenuState::ChooseTarget;

                    } else if curr_selection == "Flee" {

                        self.curr_battle.current_action.action_type(ActionType::Flee);
                        self.curr_battle.action_menu.reset();
                        self.curr_battle.action_menu.load_menu();
                        self.curr_battle.action_menu.menu_state = MenuState::ChooseActor;

                    }

                },
                MenuState::ChooseAbility => {
                    if self.player_abilities.iter().any(|ab_name| *ab_name == curr_selection) {
                        self.curr_battle.current_action.ability(curr_selection);
                    }
                    self.curr_battle.action_menu.menu_state = MenuState::ChooseTarget;
                },
                MenuState::ChooseTarget => {
                    if self.curr_battle.character_names.iter().any(|ch_name| *ch_name == curr_selection) {
                        self.curr_battle.current_action.target(curr_selection);
                    }
                    self.curr_battle.action_menu.reset();
                    self.curr_battle.action_menu.load_menu();

                    let complete_action = self.curr_battle.current_action.build(&self.abilities); 

                    self.friendly_party.characters.iter_mut().find(|ch| ch.name == complete_action.actor).unwrap().use_action_points();

                
                    complete_action.resolve(&mut self.friendly_party, &mut self.curr_battle.enemy_party, &mut self.curr_battle.dialogue_box);

                    self.cleanup_dead_characters();

                    self.curr_battle.action_menu.menu_state = MenuState::ChooseActor;
                }
            }
        }

        // Go back in selection
        if _ctx.keyboard.is_key_just_pressed(KeyCode::X) {
            self.curr_battle.action_menu.go_back();
            self.curr_battle.action_menu.load_menu();

            match self.curr_battle.action_menu.menu_state {
                MenuState::ChooseTarget => {
                    self.curr_battle.action_menu.menu_state = MenuState::ChooseAbility;
                }
                MenuState::ChooseAbility => {
                    self.curr_battle.action_menu.menu_state = MenuState::ChooseActionType;
                },
                MenuState::ChooseActionType => {
                    self.curr_battle.action_menu.menu_state = MenuState::ChooseActor;
                },
                MenuState::ChooseActor => {},
            }
        }
    }

    fn enemy_action(&mut self, _ctx: &mut Context) -> Result<(), String> {

        // Because borrow-checker
        let enemy_names = self.curr_battle.enemy_party.characters
            .iter()
            .map(|ch| ch.name.clone())
            .collect::<Vec<String>>();

        // If an enemy is ready to act
        if let Some(ch) = self.curr_battle.enemy_party.characters.iter_mut().find(|ch| ch.action_points == MAX_ACTION_POINTS) {
            ch.use_action_points();

            // Use rng to choose random ability
            let mut rng = rand::rng();
            let ab = ch.abilities
                .choose(&mut rng)
                .unwrap()
                .clone();

            // Return error if ability name is unknown
            let valid_ability = match self.abilities.iter().find(|a| a.name == ab) {
                Some(a) => a.clone(),
                None => {
                    return Err("Ability not found".to_owned());
                } 
            };

            let t = match valid_ability.ability_type {
                AbilityType::Damage | AbilityType::Status => {
                    self.friendly_party.characters
                        .iter()
                        .choose(&mut rng)
                        .unwrap()
                        .name
                        .to_owned()
                },
                AbilityType::Heal | AbilityType::Buff => {
                    enemy_names
                        .iter()
                        .choose(&mut rng)
                        .unwrap()
                        .to_owned()
                }
            };

            let enemy_action = PendingAction::new()
                .actor(ch.name.clone())
                .action_type(ActionType::Fight)
                .ability(ab.clone()) 
                .target(t.clone())
                .build(&self.abilities)
                .resolve(&mut self.curr_battle.enemy_party, &mut self.friendly_party, &mut self.curr_battle.dialogue_box);


            self.cleanup_dead_characters();
            
            Ok(())

        } else {
            Ok(())
        }
    }

}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        match &self.game_state {
            GameState::BattleState => {


                self.friendly_party.update_action_points();
                self.curr_battle.enemy_party.update_action_points();

                self.update_player_uis(_ctx);
                
                // Player Input
                self.navigate_menu(_ctx);
                self.select_player_action(_ctx);

                // AI 
                if let Err(err) = self.enemy_action(_ctx) {
                    println!("Error on enemy action: {}", err);
                }

                if self.enemy_party_died() {
                    self.game_state = GameState::WinState;
                }

                if self.friendly_party_died() {
                    self.game_state = GameState::LoseState;
                }
            },
            GameState::OverworldState => {

            },
            GameState::WinState => {

                if _ctx.keyboard.is_key_just_pressed(KeyCode::Return) {

                    if self.all_enemies.is_empty() {

                        self.win_message = Text::new("Nevermind, it looks like you have no more enemies. You beat the game!");

                    } else {

                        self.game_state = GameState::BattleState;
                        self.load_next_battle(_ctx);

                    }

                }

            },
            GameState::LoseState => {

            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let image = self.assets.images.get("battle_bg").unwrap(); 
        image.draw(&mut canvas, DrawParam::default());

        match &self.game_state {
            GameState::BattleState => {

                canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 500.0, 500.0));
                canvas.set_sampler(graphics::Sampler::nearest_clamp());

                self.friendly_party.draw(ctx, &mut canvas, &self.assets);
                self.curr_battle.enemy_party.draw(ctx, &mut canvas, &self.assets);

                self.curr_battle.action_menu.draw(&mut canvas, DrawParam::default());
                draw_character_uis(&mut canvas, &self.curr_battle.player_uis);
                self.curr_battle.dialogue_box.draw(&mut canvas, DrawParam::default());

                canvas.finish(ctx)?;
            },
            GameState::OverworldState => {
                canvas.finish(ctx)?;
            },
            GameState::WinState => {
                self.win_message.draw(&mut canvas, DrawParam::default().dest(Point2{x : 150.0, y : 250.0}));
                canvas.finish(ctx)?;
            },
            GameState::LoseState => {
                self.lose_message.draw(&mut canvas, DrawParam::default().dest(Point2{x : 150.0, y : 250.0}));
                canvas.finish(ctx)?;
            }
        }

        Ok(())
    }
}

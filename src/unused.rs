

use ggez::graphics::{DrawParam, Drawable};
use ggez::event::EventHandler;
use ggez::GameResult;
use ggez::GameError;
use ggez::graphics::*;
use ggez::input::*;
use ggez::glam::*;
use ggez::Context;
use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct MenuNodeKey; }

#[derive(Debug)]
enum MenuNodeKind {
    CharacterSelect,   // Pick a character
    ActionSelect,      // Fight / Guard / Item / Flee
    AbilitySelect,     // Pick ability if Fight
    ItemSelect,        // Pick item if Item
    TargetSelect,      // Pick target
}

#[derive(Debug)]
struct MenuNode {
    kind: MenuNodeKind,
    label: String,              // e.g. "Fight", "Guard", "Potion", "Goblin"
    parent: Option<MenuNodeKey>,
    children: Vec<MenuNodeKey>, // child menu nodes
}

fn build_menu(
    characters: &[String],
    abilities: &[String],
    items: &[String],
    enemies: &[String],
) -> (SlotMap<MenuNodeKey, MenuNode>, MenuNodeKey) {
    let mut nodes = SlotMap::with_key();

    // Root: Character select
    let root = nodes.insert(MenuNode {
        kind: MenuNodeKind::CharacterSelect,
        label: "Choose Character".into(),
        parent: None,
        children: vec![],
    });

    // Add character options
    for char_name in characters {
        let char_node = nodes.insert(MenuNode {
            kind: MenuNodeKind::ActionSelect,
            label: char_name.clone(),
            parent: Some(root),
            children: vec![],
        });
        nodes[root].children.push(char_node);

        // Action options
        for action in ["Fight", "Guard", "Item", "Flee"] {
            let action_node = nodes.insert(MenuNode {
                kind: MenuNodeKind::ActionSelect,
                label: action.into(),
                parent: Some(char_node),
                children: vec![],
            });
            nodes[char_node].children.push(action_node);

            match action {
                "Fight" => {
                    for ability in abilities {
                        let ab_node = nodes.insert(MenuNode {
                            kind: MenuNodeKind::AbilitySelect,
                            label: ability.clone(),
                            parent: Some(action_node),
                            children: vec![],
                        });
                        nodes[action_node].children.push(ab_node);

                        // Each ability targets enemies
                        for enemy in enemies {
                            let tgt = nodes.insert(MenuNode {
                                kind: MenuNodeKind::TargetSelect,
                                label: enemy.clone(),
                                parent: Some(ab_node),
                                children: vec![],
                            });
                            nodes[ab_node].children.push(tgt);
                        }
                    }
                }
                "Item" => {
                    for item in items {
                        let item_node = nodes.insert(MenuNode {
                            kind: MenuNodeKind::ItemSelect,
                            label: item.clone(),
                            parent: Some(action_node),
                            children: vec![],
                        });
                        nodes[action_node].children.push(item_node);

                        // Item can target enemies too
                        for enemy in enemies {
                            let tgt = nodes.insert(MenuNode {
                                kind: MenuNodeKind::TargetSelect,
                                label: enemy.clone(),
                                parent: Some(item_node),
                                children: vec![],
                            });
                            nodes[item_node].children.push(tgt);
                        }
                    }
                }
                "Guard" | "Flee" => {
                    // terminal, no children
                }
                _ => {}
            }
        }
    }

    (nodes, root)
}

#[derive(Debug)]
struct MenuState {
    current: MenuNodeKey,
    selected_index: usize,
}

impl MenuState {
    fn move_up(&mut self, nodes: &SlotMap<MenuNodeKey, MenuNode>) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self, nodes: &SlotMap<MenuNodeKey, MenuNode>) {
        let node = &nodes[self.current];
        if self.selected_index + 1 < node.children.len() {
            self.selected_index += 1;
        }
    }

    fn select(&mut self, nodes: &SlotMap<MenuNodeKey, MenuNode>) {
        let node = &nodes[self.current];
        if let Some(&child) = node.children.get(self.selected_index) {
            self.current = child;
            self.selected_index = 0;
        }
    }

    fn go_back(&mut self, nodes: &SlotMap<MenuNodeKey, MenuNode>) {
        if let Some(parent) = nodes[self.current].parent {
            self.current = parent;
            self.selected_index = 0;
        }
    }
}


pub struct DummyState {
    state : MenuState,
    map : SlotMap<MenuNodeKey, MenuNode>,
    root: MenuNodeKey,
}

impl DummyState {
    pub fn new() -> DummyState {
        let (nodes, root) = build_menu(&["hero_1".into(), "hero_2".into()], &["suck".into(), "peenor".into()], &["potion_1".into(), "potion_2".into()], &["hulk".into(), "orc".into(), "troll".into()]);
        DummyState { 
            state: MenuState { current: root, selected_index: 0 },
            map : nodes,
            root
        }
    }
}

impl EventHandler<GameError> for DummyState {

    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {

        if _ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Up) {
            self.state.move_up(&self.map);
        }
        if _ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Down) {
            self.state.move_down(&self.map);
        }
        if _ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Z) {
            self.state.select(&self.map);
        }
        if _ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::X) {
            self.state.go_back(&self.map);
        }
        
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas =
            Canvas::from_frame(_ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 500.0, 500.0));
        canvas.set_sampler(Sampler::nearest_clamp());
        draw_menu(&self.state, &self.map, &mut canvas);
        canvas.finish(_ctx)?;
        Ok(())
    }
}

fn draw_menu(state: &MenuState, nodes: &SlotMap<MenuNodeKey, MenuNode>, canvas: &mut Canvas) -> GameResult {
    let node = &nodes[state.current];

    for (i, &child_key) in node.children.iter().enumerate() {
        let child = &nodes[child_key];
        let color = if i == state.selected_index {
            Color::YELLOW
        } else {
            Color::WHITE
        };

        // let text = Text::new((child.label.clone(), 24.0));
        // let text = Text::new("Foo");
        let text = Text::new(TextFragment::new(child.label.as_str()).color(color));
        let pos = Vec2::new(50.0, 50.0 + i as f32 * 30.0);
        // graphics::draw(ctx, &text, (pos, color))?;
        text.draw(canvas, DrawParam::default().dest(pos));
    }

    Ok(())
}
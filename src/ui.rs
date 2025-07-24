use ggez::{
    Context,
    graphics::{Canvas, Color, DrawParam, DrawMode, Drawable, Rect, Text, TextFragment, Mesh}, mint::Point2
};
use crate::{characters, menu::{self, NodeHandle}};
use crate::menu::Menu;
use std::cmp::min;

const DEFAULT_POSITION : Point2<f32> = Point2{x: 300.0, y : 400.0}; 

const SELECTED_COLOR : Color = Color::new(1.0, 1.0, 0.0, 1.0);
const DEFAULT_COLOR : Color = Color::new(1.0, 1.0, 1.0, 1.0);

const BOX_PADDING : f32 = 10.0;
const BOX_SIZE : f32 = 10.0;

pub struct Ui {
    position: Point2<f32>,
    text_boxes: Vec<Text>,    
    menu: Menu,
    curr_node: NodeHandle,
    selected_box: i32,
}

impl Ui {
    pub fn new(menu: Menu) -> Self {
        Ui {
            position: DEFAULT_POSITION,
            text_boxes: vec![Text::new(""); 4],
            menu,
            curr_node: 0,
            selected_box: 0,
        }
    }

    pub fn load_menu(&mut self) {
        let mut new_boxes: Vec<Text> = vec![];

        for (i, menu_node) in self.menu.nodes[self.curr_node].children.iter().enumerate() {
            let mut text: Text = Default::default();

            if i as i32 == self.selected_box {
                text.add(TextFragment{
                    text: self.menu.nodes[*menu_node].data.clone(), 
                    color : Some(SELECTED_COLOR),
                    font : None,
                    scale: None,
                });
            } else {
                text.add(TextFragment{
                    text: self.menu.nodes[*menu_node].data.clone(), 
                    color : Some(DEFAULT_COLOR),
                    font : None,
                    scale: None,
                });
            }
            new_boxes.push(text);
        }

        self.text_boxes = new_boxes;
    }

    pub fn select(&mut self) {
        if let Some(new_root) = self.menu.nodes[self.curr_node].children.get(self.selected_box as usize) {
            // Set new "root" to display its children
            self.curr_node = *new_root;
            // Set selected_box to the first node of the new menu
            self.selected_box = 0;
        } else {
            println!("No node found!");
        }
    }

    pub fn go_back(&mut self) {
        self.curr_node = self.menu.nodes[self.curr_node].parent;
    }

    pub fn change_selection(&mut self, diff: i32) {

        self.selected_box += diff; 

        if self.selected_box < 0 {
            self.selected_box = self.text_boxes.len() as i32 - 1;
        }

        if self.selected_box > self.text_boxes.len() as i32 - 1 {
            self.selected_box = 0;
        }

    }

}

impl Drawable for Ui {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {

        let mut curr_dest = self.position;

        for text_box in &self.text_boxes {
            curr_dest = Point2{x: curr_dest.x, y: curr_dest.y + BOX_PADDING};
            text_box.draw(canvas, DrawParam::default().dest(curr_dest));
        }
    }

    fn dimensions(&self, gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>) -> Option<ggez::graphics::Rect> {
        
        Some(Rect::new(BOX_SIZE, BOX_SIZE, BOX_SIZE, BOX_SIZE))
    }
}

const BAR_WIDTH_SCALE : f32 = 5.0;
const BAR_HEIGHT : f32 = 10.0;
pub const HERO_AP_COLOR : Color = Color::GREEN;
pub const ENEMY_AP_COLOR : Color = Color::YELLOW;

#[derive(Clone)]
pub struct ActionBar {
    mesh: Mesh,
    dimensions: Rect,
}

impl ActionBar {

    pub fn new(ctx: &mut Context, color: Color) -> Self {
        let width = characters::MAX_ACTION_POINTS as f32 / BAR_WIDTH_SCALE;
        let height = BAR_HEIGHT;
        let rect = Rect{ x: 0., y:0., w: width, h: height };
        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color).unwrap();

        ActionBar { 
            mesh,
            dimensions: rect 
        }
    }

    pub fn update(&mut self, ctx: &mut Context, value: f32, color: Color) {
        let width = value / BAR_WIDTH_SCALE;
        let height = BAR_HEIGHT;
        let rect = Rect{ x: 0., y:0., w: width, h: height };

        self.mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color).unwrap();
        self.dimensions = rect;
    }

}


impl Drawable for ActionBar {

    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        canvas.draw(&self.mesh, param);
    }

    fn dimensions(&self, gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>) -> Option<Rect> {
        Some(self.dimensions)
    }
}


const DEFAULT_BAR_POSITION : Point2<f32> = Point2{ x : 100., y: 350. };
const BAR_PADDING : f32 = 50.0;

pub struct Bars {
    pub action_bars: Vec<ActionBar>
}

impl Bars {
    pub fn new(action_bars: Vec<ActionBar>) -> Self {
        Bars {
            action_bars
        }
    }

    pub fn draw_action_bars(&self, canvas: &mut Canvas) {
        for (i, bar) in self.action_bars.iter().enumerate() {
            let offset = i as f32 * BAR_PADDING;
            let dest = Point2{ x : DEFAULT_BAR_POSITION.x, y : DEFAULT_BAR_POSITION.y + offset };
            bar.draw(canvas, DrawParam::default().dest(dest));
        }
    }
}
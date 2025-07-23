use ggez::{
    graphics::{Canvas, Color, DrawParam, Drawable, Rect, Text, TextFragment}, mint::Point2
};
use crate::{menu, menu::NodeHandle};
use crate::menu::Menu;

const SELECTED_COLOR : Color = Color::new(1.0, 1.0, 0.0, 1.0);
const DEFAULT_COLOR : Color = Color::new(1.0, 1.0, 1.0, 1.0);

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
            position: Point2{x: 300.0, y: 400.0},
            text_boxes: vec![Text::new("test"); 4],
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
        if let Some(first_node) = self.menu.nodes[self.curr_node].children.get(self.selected_box as usize) {
            self.curr_node = *first_node;
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
            curr_dest = Point2{x: curr_dest.x, y: curr_dest.y + 10.0};
            text_box.draw(canvas, DrawParam::default().dest(curr_dest));
        }
    }

    fn dimensions(&self, gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>) -> Option<ggez::graphics::Rect> {
        
        Some(Rect::new(10.0, 10.0, 10.0, 10.0))
    }
}
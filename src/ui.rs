

use ggez::{
    graphics::{Canvas, Color, DrawParam, Drawable, Rect, Text, TextFragment}, mint::Point2
};

use std::rc::{Rc, Weak};

use crate::menu::MenuNodeRef;


pub struct Ui {
    position: Point2<f32>,
    text_boxes: Vec<Text>,    
    curr_menu: MenuNodeRef,
    selected_node: i32,
}

impl Ui {
    pub fn new(root_menu_node: MenuNodeRef) -> Self {

        // TODO: load menu nodes into UI somehow
        Ui {
            position: Point2{x: 300.0, y: 400.0},
            text_boxes: vec![Text::new("test"); 4],
            curr_menu: root_menu_node,
            selected_node: 0,
        }
    }

    pub fn load_menu(&mut self) {
        // let mut menu_text: Vec<String> = vec![];
        let mut new_boxes: Vec<Text> = vec![];

        for (i, menu_node) in self.curr_menu.borrow().children.iter().enumerate() {

            let mut text: Text = Default::default();

            if i as i32 == self.selected_node {
                text.add(TextFragment{text: menu_node.borrow().name.clone(), color: Some(Color::new(1.0, 1.0, 0.0, 1.0)), font: None, scale: None});
            } else {
                text.add(TextFragment{text: menu_node.borrow().name.clone(), color: Some(Color::new(1.0, 1.0, 1.0, 1.0)), font: None, scale: None});
            } 

            new_boxes.push(text);
        }

        self.text_boxes = new_boxes;
    }

    pub fn select_child(&mut self) {

        if let Some(first_child) = self.curr_menu.clone().borrow().children.get(self.selected_node as usize) {
            self.curr_menu = Rc::clone(first_child);
            // println!("{}", self.curr_menu.borrow().clone().name);
            // println!("{}", self.curr_menu.borrow().clone().parent.unwrap().upgrade().unwrap().borrow().clone().name);
        } else {
            println!("няма деца");
        }        
    }

    pub fn go_back(&mut self) {

        if let Some(parent) = &self.curr_menu.clone().borrow().parent {

            self.curr_menu = parent.upgrade().unwrap();
        } else {
            println!("няма деца");
        }        
    }

    pub fn change_selection(&mut self, diff: i32) {

        self.selected_node += diff; 

        if self.selected_node < 0 {
            self.selected_node = self.text_boxes.len() as i32 - 1;
        }

        if self.selected_node > self.text_boxes.len() as i32 - 1 {
            self.selected_node = 0;
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
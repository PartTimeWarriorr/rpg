
use ggez::{
    graphics::{Rect, DrawParam, Drawable, Text, Canvas, Transform}, mint::{Point2, Vector2}
};

use crate::menu::MenuNode;


pub struct Ui {
    position: Point2<f32>,
    text_boxes: Vec<Text>,    
    curr_menu: MenuNode,
}

impl Ui {
    pub fn new() -> Self {

        // TODO: load menu nodes into UI somehow
        Ui {
            position: Point2{x: 100.0, y: 100.0},
            text_boxes: vec![Text::new("test"); 3],
            curr_menu: MenuNode::new("oh"),
        }
    }

}

impl Drawable for Ui {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {

        let mut curr_dest = Point2{x:0.0, y:0.0};

        for text_box in &self.text_boxes {
            curr_dest = Point2{x: curr_dest.x, y: curr_dest.y + 10.0};
            text_box.draw(canvas, DrawParam::default().dest(curr_dest));
        }
    }

    fn dimensions(&self, gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>) -> Option<ggez::graphics::Rect> {
        
        Some(Rect::new(10.0, 10.0, 10.0, 10.0))
    }
}
use core::fmt;
use std::{cell::{Ref, RefCell}, fmt::Formatter, rc::{Rc, Weak}};

pub type MenuNodeRef = Rc<RefCell<MenuNode>>;

#[derive(Clone, Debug)]
pub struct MenuNode {
    pub name: String,
    pub parent: Option<Weak<RefCell<MenuNode>>>,
    pub children: Vec<MenuNodeRef>,
}

impl MenuNode {

    pub fn new(name: &str, ) -> MenuNodeRef {
        Rc::new(RefCell::new(MenuNode {
            name: String::from(name), 
            parent: None, 
            children: vec![]
        }))
    }

    fn format_node(&self, f: &mut Formatter, depth: i32) -> fmt::Result {

        if self.children.is_empty() {
            writeln!(f, "{}", self.name)?;
        } else {
            writeln!(f, "{} ->", self.name)?;
        }

        for child in &self.children {
            for _i in 0..=depth {
                write!(f, "|  ")?;
            }
            child.borrow().format_node(f, depth + 1)?;
        }

        Ok(())
    }

    pub fn add_child(parent: &MenuNodeRef, child: MenuNodeRef) {
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push(child);
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

}

impl fmt::Display for MenuNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.format_node(f, 0)
    }
}


#[cfg(test)]
mod tests {
    use std::{fmt::format, result};

    use super::*;

    #[test]
    fn test_format_node() {

        let root = MenuNode::new("root");

        let names_vec = vec!["Fight", "Guard", "Item", "Flee"];

        for name in names_vec {
            let ch = MenuNode::new(name);
            let chch = MenuNode::new("i'm a child");
            MenuNode::add_child(&ch, chch);
            MenuNode::add_child(&root, ch);
        }

        let result = format!("{}", root.borrow());

        assert_eq!(result, "root ->\n\
                            |  Fight ->\n\
                            |  |  i'm a child\n\
                            |  Guard ->\n\
                            |  |  i'm a child\n\
                            |  Item ->\n\
                            |  |  i'm a child\n\
                            |  Flee ->\n\
                            |  |  i'm a child\n")

    }
}
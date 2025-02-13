use core::fmt;
use std::fmt::Formatter;


pub struct MenuNode {
    name: String,
    children: Vec<MenuNode>
}

impl MenuNode {

    pub fn new(name: &str) -> Self {
        MenuNode { 
            name: String::from(name),
            children: Vec::new() 
        }
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
            child.format_node(f, depth + 1)?;
        }

        Ok(())
    }

    pub fn add_child(&mut self, child: MenuNode) {
        self.children.push(child); 
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
    use super::*;

    #[test]
    fn test_format_node() {

        let mut root = MenuNode::new("root");
        
        let names_vec = vec!["Fight", "Guard", "Item", "Flee"];

        for name in names_vec {
            let mut ch = MenuNode::new(name);
            let chch = MenuNode::new("i'm a child");
            ch.add_child(chch);
            root.add_child(ch);
        }

        let result = format!("{}", root);

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
pub type NodeHandle = usize;

#[derive(Debug)]
pub struct Menu {
    pub root: NodeHandle,
    pub nodes: Vec<Node>
}

#[derive(Debug)]
pub struct Node {
    pub data: String,
    pub children: Vec<NodeHandle>,
    pub parent: NodeHandle
}

impl Node {
    pub fn new(data: &str) -> Self {
        Node {
            data: String::from(data),
            children: vec![],
            parent: 0
        }
    }

    pub fn new_with_parent(data: &str, parent: NodeHandle) -> Self {
        Node {
            data: String::from(data),
            children: vec![],
            parent
        }
    }

    // pub fn get_children() -> Vec<NodeHandle> {
        
    // }
}

impl Menu {

    pub fn new() -> Self {
        Menu {
            root: 0,
            nodes: vec![Node::new("root")],
        }
    }

    fn alloc_node(&mut self, data: &str, parent: NodeHandle) -> NodeHandle {
        // Creates new child Node to parent and returns the child NodeHandle
        self.nodes.push(Node::new_with_parent(data, parent));
        self.nodes.len() - 1
    }

    pub fn insert(&mut self, data: &str) {
        if self.root == 0 {
            self.root = self.alloc_node(data, 0);
        }
    }

    fn find_node_handle(&self, name: &str) -> Option<NodeHandle> {
        self.nodes.iter().position(|node| node.data == name )
    }

    pub fn insert_at(&mut self, parent_name: &str, data: &str) {

        if let Some(parent_handle) = self.find_node_handle(parent_name) {
            let child_handle = self.alloc_node(data, parent_handle);
            self.nodes[parent_handle].children.push(child_handle);
        } else {
            panic!("Parent name {} not found!", parent_name);
        }

    }

    fn find_node_handle_by_path(&self, path: &[&str]) -> Option<NodeHandle> {
        let mut current = 0;

        // Remove root from path
        let sliced_path = &path[1..];

        for name in sliced_path {
            let next = self.nodes[current].children
            .iter()
            .find(|&&child_handle| self.nodes[child_handle].data == *name)?;

            current = *next;
        }

        Some(current)
    }

    // For when we have multiple nodes with the same name
    pub fn insert_at_path(&mut self, path_name: &[&str], data: &str) {

        if let Some(parent_handle) = self.find_node_handle_by_path(path_name) {
            let child_handle = self.alloc_node(data, parent_handle);
            self.nodes[parent_handle].children.push(child_handle);
        } else {
            panic!("Incorrect path: {:?}", path_name);
        }

    }

}

#[cfg(test)]
mod tests {
    use crate::characters::Ability;

    use super::*;

    #[test]
    fn test_insert_root() {
        let mut menu = Menu::new();
        menu.insert_at("root", "node");

        assert!(menu.nodes.len() == 2 as usize);
    }

    #[test]
    fn test_insert_at() {
        let mut menu = Menu::new();

        for hero_name in vec!["hero_1", "hero_2", "hero_3", "hero_4"] {
            menu.insert_at("root", hero_name);

            for action in vec!["Fight", "Guard", "Item", "Flee"] {
                menu.insert_at(hero_name, action);

            }

            for ability in vec!["a1", "a2", "a3", "a4"] {
                menu.insert_at_path(&["root", hero_name, "Fight"], ability);
            }
        }        
        dbg!(&menu);

        // 1 root + 4 heroes + 4 * 4 actions + 4 * 4 abilities = 5 + 32 = 37
        assert!(menu.nodes.len() == 37 as usize);
    }
}
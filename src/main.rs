use std::cmp::Ordering;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Behavior {
    Play,
    Purchase,

}
#[derive(Debug, Clone)]
struct GameData {
    id: u32,
    game_name: String,
    behaviour: Behavior,
    play_purchase: f64,
    identical: bool,
}
#[derive(Debug)]
struct TreeNode {
    data: GameData,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {

    fn new(data: GameData) -> Self {
        return TreeNode {
            data,
            left: None,
            right: None,
        }
    }


    fn insert(&mut self, data: GameData) {
        match data.id.cmp(&self.data.id) {
             // If the ID is lesser, put it to the left
            Ordering::Less => {
                match &mut self.left {
                    Some(left_node) => left_node.insert(data),
                    None => self.left = Some(Box::new(TreeNode::new(data))),

                }
                
            } // If the ID is greater, put it to the right
            Ordering::Greater => {
                match &mut self.right {
                    Some(right_node) => right_node.insert(data),
                    None => self.right = Some(Box::new(TreeNode::new(data))),
    
                }
            } // If they are equal we need to go further down
            Ordering::Equal => {
                match data.game_name.cmp(&self.data.game_name) {
                    // If the game_name is lesser, put it to the left
                    Ordering::Less => match &mut self.left {
                        Some(left_node) => left_node.insert(data),
                        None => self.left = Some(Box::new(TreeNode::new(data))),
                    }, // If the game_name is greater, put it to the right
                    Ordering::Greater => match &mut self.right {
                        Some(right_node) => right_node.insert(data),
                        None => self.right = Some(Box::new(TreeNode::new(data))),
                    },
                    Ordering::Equal => { // If the names are equal, we need to look at behaviour
                        match data.behaviour.cmp(&self.data.behaviour) {
                            // If the behaviour is lesser, put it to the left
                            Ordering::Less => match &mut self.left {
                                Some(left_node) => left_node.insert(data),
                                None => self.left = Some(Box::new(TreeNode::new(data))),
                            }, // If the behaviour is greater, put it to the right
                            Ordering::Greater => match &mut self.right {
                                Some(right_node) => right_node.insert(data),
                                None => self.right = Some(Box::new(TreeNode::new(data))),
                            },
                            Ordering::Equal => {
                                // Do nothing. Look at the readme.md. This shouldn't happen...
                                println!("Major error");
                            },
                        }
                    }
                }
            }
        }
    }
    fn print_tree(&self) {
        if let Some(left) = &self.left {
            left.print_tree();
        }
        println!("{:?}", self.data);
        if let Some(right) = &self.right {
            right.print_tree();
        }
    }

}



fn main() {
    println!("Hello, world!");
}

use std::cmp::Ordering;
use std::fs;
use std::env;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Behavior {
    Play,
    Purchase,
    Error,
}

#[derive(Debug, Clone)]
struct GameData {
    id: u32,
    game_name: String,
    behaviour: Behavior,
    play_purchase: f64,
}

#[derive(Debug)]
struct TreeNode {
    data: GameData,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(data: GameData) -> Self {
        TreeNode {
            data,
            left: None,
            right: None,
        }
    }

    // Insert with duplicate detection
    fn insert(&mut self, data: GameData, duplicates: &mut Vec<GameData>) {
        match data.id.cmp(&self.data.id) {
            Ordering::Less => {
                if let Some(left_node) = &mut self.left {
                    left_node.insert(data, duplicates);
                } else {
                    self.left = Some(Box::new(TreeNode::new(data)));
                }
            }
            Ordering::Greater => {
                if let Some(right_node) = &mut self.right {
                    right_node.insert(data, duplicates);
                } else {
                    self.right = Some(Box::new(TreeNode::new(data)));
                }
            }
            Ordering::Equal => {
                // If `id` matches, compare `game_name`
                match data.game_name.cmp(&self.data.game_name) {
                    Ordering::Less => {
                        if let Some(left_node) = &mut self.left {
                            left_node.insert(data, duplicates);
                        } else {
                            self.left = Some(Box::new(TreeNode::new(data)));
                        }
                    }
                    Ordering::Greater => {
                        if let Some(right_node) = &mut self.right {
                            right_node.insert(data, duplicates);
                        } else {
                            self.right = Some(Box::new(TreeNode::new(data)));
                        }
                    }
                    Ordering::Equal => {
                        // If both `id` and `game_name` match, compare `behaviour`
                        match data.behaviour.cmp(&self.data.behaviour) {
                            Ordering::Less => {
                                if let Some(left_node) = &mut self.left {
                                    left_node.insert(data, duplicates);
                                } else {
                                    self.left = Some(Box::new(TreeNode::new(data)));
                                }
                            }
                            Ordering::Greater => {
                                if let Some(right_node) = &mut self.right {
                                    right_node.insert(data, duplicates);
                                } else {
                                    self.right = Some(Box::new(TreeNode::new(data)));
                                }
                            }
                            Ordering::Equal => {
                                // Check `play_purchase` since all other fields match
                                if data.play_purchase == self.data.play_purchase {
                                    // Duplicate detected
                                    duplicates.push(data.clone());
                                    println!("Duplicate found: {:?}", data);
                                } else if data.play_purchase < self.data.play_purchase {
                                    if let Some(left_node) = &mut self.left {
                                        left_node.insert(data, duplicates);
                                    } else {
                                        self.left = Some(Box::new(TreeNode::new(data)));
                                    }
                                } else {
                                    if let Some(right_node) = &mut self.right {
                                        right_node.insert(data, duplicates);
                                    } else {
                                        self.right = Some(Box::new(TreeNode::new(data)));
                                    }
                                }
                            }
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

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut columns = Vec::new();
    let mut comma_indices = Vec::new();
    
    // Remove all quotes from the line
    let line = line.replace('"', "");

    // Find the positions of all commas
    for (i, c) in line.char_indices() {
        if c == ',' {
            comma_indices.push(i);
        }
    }

    if comma_indices.len() < 4 {
        // If there are not enough commas, return the whole line as a single column
        columns.push(line);
        return columns;
    }

    // Extract the ID
    let id_end = comma_indices[0];
    columns.push(line[..id_end].to_string());

    // Extract the game name (from after the first comma to before the third-to-last comma)
    let game_name_start = id_end + 1;
    let game_name_end = comma_indices[comma_indices.len() - 3];
    columns.push(line[game_name_start..game_name_end].to_string());

    // Extract the behaviour (from after the third-to-last comma to before the second-to-last comma)
    let behaviour_start = game_name_end + 1;
    let behaviour_end = comma_indices[comma_indices.len() - 2];
    columns.push(line[behaviour_start..behaviour_end].to_string());

    // Extract the play_purchase (from after the second-to-last comma to before the last comma)
    let play_purchase_start = behaviour_end + 1;
    let play_purchase_end = comma_indices[comma_indices.len() - 1];
    columns.push(line[play_purchase_start..play_purchase_end].to_string());

    // Extract the none (from after the last comma to the end)
    let none_start = play_purchase_end + 1;
    columns.push(line[none_start..].to_string());

    columns
}
fn parse_game_data(columns: Vec<String>) -> GameData {
    if columns.len() != 5 {  // Now check for exactly 5 parts, including the trailing zero
        println!("Insufficient data in columns: {:?}", columns);
        return GameData {
            id: 0,
            game_name: String::from("Unknown"),
            behaviour: Behavior::Error,
            play_purchase: 0.0,
        };
    }

    let id = columns[0].parse().unwrap_or(0); // Default to 0 if parsing fails
    let game_name = columns[1].to_string();
    let behaviour = match columns[2].to_lowercase().as_str() {
        "play" => Behavior::Play,
        "purchase" => Behavior::Purchase,
        _ => Behavior::Error, // Handle unknown behavior
    };

    if behaviour == Behavior::Error {
        println!("Major Error: Invalid behavior in row: {:?}", columns);
    }

    let play_purchase = columns[3].parse().unwrap_or(0.0); // Default to 0.0 if parsing fails

    GameData {
        id,
        game_name,
        behaviour,
        play_purchase,
    }
}
fn main() -> std::io::Result<()> {
    // Get the current working directory
    let cwd = env::current_dir().expect("Failed to get current working directory");

    // Construct the full path to the CSV file 
    let mut path = cwd;
    path.push("csv_files");
    path.push("algorithms part dataset.csv");

    // Convert the path to a string for printing or other uses (if needed)
    let path_str = path.to_str().expect("Failed to convert path to string");

    // Read the file content into a string
    let csv_content = fs::read_to_string(&path_str)?;
    let mut skipped = false;

    let lines: Vec<&str> = csv_content.lines().collect();
    // Initialize the tree with the first line of data (skipping the header)
    let mut tree: Option<TreeNode> = None;
    let mut duplicates: Vec<GameData> = Vec::new();

    // Process each line
    for line in lines {
        if !skipped {
            skipped = true;
            continue; // Skip the header
        }

        let columns: Vec<String> = parse_csv_line(line);
        let game_data = parse_game_data(columns);

        if tree.is_none() {
            // Initialize the tree with the first valid game data
            tree = Some(TreeNode::new(game_data));
        } else {
            // Insert subsequent game data into the tree
            tree.as_mut().unwrap().insert(game_data, &mut duplicates);
        }
    }

    // Print the tree if it was initialized
    if let Some(tree) = tree {
        println!("Tree contents:");
        tree.print_tree();
    } else {
        println!("No data available to create the tree.");
    }
    
    // Print duplicates
    if !duplicates.is_empty() {
        println!("Duplicates found:");
        for duplicate in duplicates {
            println!("{:?} is a duplicate", duplicate);
        }
    } else {
        println!("No duplicates found.");
    }

    Ok(())
}
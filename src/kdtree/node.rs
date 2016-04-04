use kdtree::dimension::Dimension;
use kdtree::particle::Particle;
#[derive(Clone)]
pub struct Node {
    pub split_dimension: Dimension, // Dimension that this node splits at.
    pub split_value: f64, // Value that this node splits at.
    pub left: Option<Box<Node>>, // Left subtree.
    pub right: Option<Box<Node>>, // Right subtree.
    pub points: Option<Vec<Particle>>, // Vector of the points if this node is a Leaf.
    pub center_of_mass: (f64, f64, f64), /* The center of mass for this node and it's children all
                                          * together. (x, y, z). */
    pub total_mass: f64, // Total mass of all particles under this node.
    pub r_max: f64, // Maximum radius that is a child of this node.
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
}

impl Node {
    // Some convenience functions.
    /// Returns a node with all 0.0 or "None/Null" values.
    pub fn new() -> Node {
        return Node {
            split_dimension: Dimension::Null,
            split_value: 0.0,
            left: None,
            right: None,
            points: None,
            center_of_mass: (0.0, 0.0, 0.0), // (pos * mass) + (pos * mass) / sum of masses
            total_mass: 0.0,
            r_max: 0.0,
            x_max: 0.0,
            x_min: 0.0,
            y_max: 0.0,
            y_min: 0.0,
            z_max: 0.0,
            z_min: 0.0,
        };
    }
    // Used when treating a node as the sum of its parts in gravity calculations.
    /// Converts a node into a particle with the x, y, z, and mass being derived from the center of
    /// mass and the total mass of the particles it contains.
    pub fn to_particle(&self) -> Particle {
        return Particle {
            x: self.center_of_mass.0,
            y: self.center_of_mass.1,
            z: self.center_of_mass.2,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            mass: self.total_mass,
            radius: 0.0,
        };
    }
    // Function that is not being used anymore. Returns a vector of the node and
    // all of its subnodes.
    pub fn max_distance(&self) -> f64 {
        let x_distance = (self.x_max - self.x_min);
        let y_distance = (self.y_max - self.y_min);
        let z_distance = (self.z_max - self.z_min);
        return  f64::max(x_distance, f64::max(y_distance, z_distance));
    }
    /// Returns a vector of this node and all subnodes.
    fn iterate_over_nodes(&self) -> Vec<Node> {
        let node = self.clone();
        let mut to_return: Vec<Node> = vec![node.clone()];
        match node.left {
            Some(ref node) => {
                let node_left = node.left.clone().expect("");
                let unboxed_node: Node = *node_left;
                to_return.append(&mut unboxed_node.iterate_over_nodes());
            }
            None => (),
        }
        match node.right {
            Some(ref node) => {
                let node_right = node.right.clone().expect("");
                let unboxed_node: Node = *node_right;
                to_return.append(&mut unboxed_node.iterate_over_nodes());
            }
            None => (),
        }
        return to_return;
    }
    // A helpful function that is called in the tests. Prints out the contents of
    // the tree in a rather ugly manner.
    /// Recursively prints the node and all nodes within it.
    pub fn display_tree(&self) {
        let mut to_display = Node::display_tree_helper(self, 0);
        to_display.sort_by(|a, b| (a.2).cmp(&b.2));
        let mut to_display_string: String = "".to_string();
        let mut prev: i32 = -1;
        for element in to_display {
            let info = format!("split on: {}{}    ", element.0.as_string(), element.1);
            println!("info: {}\n", info);
            to_display_string = format!("{} {}", to_display_string, info);
            if element.2 > prev {
                to_display_string = format!("{}\n", to_display_string);
            }
            prev = element.2;
        }
        println!("{}", to_display_string);
    }
    // Thank you Steve Klabnik for your help with this function.
    // Recursive helper for display_tree()
    /// Recursive helper function for display_tree().
    fn display_tree_helper(node: &Node, level: i32) -> Vec<(Dimension, f64, i32)> {
        let dim = node.split_dimension.clone();
        let split_val = node.split_value;
        let mut to_return: Vec<(Dimension, f64, i32)> = vec![(dim, split_val, level)];
        match node.left {
            Some(ref node) => {
                let mut tmp_vec = Node::display_tree_helper(node, level + 1);
                to_return.append(&mut tmp_vec);
            }
            None => (),
        }
        match node.right {
            Some(ref node) => {
                let mut tmp_vec = Node::display_tree_helper(node, level + 1);
                to_return.append(&mut tmp_vec);
            }
            None => (),
        }
        to_return
    }
}

use kdtree::dimension::Dimension;
use kdtree::particle::Particle;
#[derive(Clone)]
pub enum Node {
    Leaf {
        points: Vec<Particle>,
        properties: Properties,
    },
    Interior {
        split_dimension: Dimension,
        split_value: f64,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        properties: Properties,
    },
}

#[derive(Clone, Default)]
pub struct Properties {
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
// pub struct Node {
// pub split_dimension: Dimension, // Dimension that this node splits at.
// pub split_value: f64, // Value that this node splits at.
// pub left: Option<Box<Node>>, // Left subtree.
// pub right: Option<Box<Node>>, // Right subtree.
// pub points: Option<(i32, i32)>, // Vector of the points if this node is a Leaf.
// pub center_of_mass: (f64, f64, f64), /* The center of mass for this node and it's children all
// together. (x, y, z). */
// pub total_mass: f64, // Total mass of all particles under this node.
// pub r_max: f64, // Maximum radius that is a child of this node.
// pub x_min: f64,
// pub x_max: f64,
// pub y_min: f64,
// pub y_max: f64,
// pub z_min: f64,
// pub z_max: f64,
// }
//
impl Node {
    // Some convenience functions.
    /// Returns a node with all 0.0 or "None/Null" values.
    pub fn new_leaf() -> Node {
        Node::Leaf {
            points: Vec::new(),
            properties: Properties::default(),
        }
    }

    // Since this field is in both elements of the Enum, this is an accessor function.
    pub fn properties(&self) -> &Properties {
        match *self {
            Node::Leaf { ref properties, .. } |
            Node::Interior { ref properties, .. } => properties,

        }
    }

    pub fn new_interior() -> Node {
        Node::Interior {
            split_dimension: Dimension::None,
            split_value: 0.0,
            left: None,
            right: None,
            properties: Properties::default(),
        }
    }
    // Used when treating a node as the sum of its parts in gravity calculations.
    /// Converts a node into a particle with the x, y, z, and mass being derived from the center of
    /// mass and the total mass of the particles it contains.
    pub fn to_particle(&self) -> Particle {
        let center_of_mass = self.properties().center_of_mass;
        let total_mass = self.properties().total_mass;
        Particle {
            x: center_of_mass.0,
            y: center_of_mass.1,
            z: center_of_mass.2,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            mass: total_mass,
            radius: 0.0,
        }
    }
    // Function that is not being used anymore. Returns a vector of the node and
    // all of its subnodes.
    pub fn max_distance(&self) -> f64 {
        let properties = self.properties();
        let x_distance = properties.x_max - properties.x_min;
        let y_distance = properties.y_max - properties.y_min;
        let z_distance = properties.z_max - properties.z_min;
        f64::max(x_distance, f64::max(y_distance, z_distance))
    }
    //    /// Returns a vector of this node and all subnodes.
    //
    // fn iterate_over_nodes(&self) -> Vec<Node> {
    // let node = self.clone();
    // let mut to_return: Vec<Node> = vec![node.clone()];
    // match node.left {
    // Some(ref node) => {
    // let node_left = node.left.clone().expect("");
    // let unboxed_node: Node = *node_left;
    // to_return.append(&mut unboxed_node.iterate_over_nodes());
    // }
    // None => (),
    // }
    // match node.right {
    // Some(ref node) => {
    // let node_right = node.right.clone().expect("");
    // let unboxed_node: Node = *node_right;
    // to_return.append(&mut unboxed_node.iterate_over_nodes());
    // }
    // None => (),
    // }
    // return to_return;
    // }
    // A helpful function that is called in the tests. Prints out the contents of
    // the tree in a rather ugly manner.
    // Recursively prints the node and all nodes within it.
    // pub fn display_tree(&self) {
    // let mut to_display = Node::display_tree_helper(self, 0);
    // to_display.sort_by(|a, b| (a.2).cmp(&b.2));
    // let mut to_display_string: String = "".to_string();
    // let mut prev: i32 = -1;
    // for element in to_display {
    // let info = format!("split on: {}{}    ", element.0.as_string(), element.1);
    // println!("info: {}\n", info);
    // to_display_string = format!("{} {}", to_display_string, info);
    // if element.2 > prev {
    // to_display_string = format!("{}\n", to_display_string);
    // }
    // prev = element.2;
    // }
    // println!("{}", to_display_string);
    // }
    // Thank you Steve Klabnik for your help with this function.
    // Recursive helper for display_tree()
    // Recursive helper function for display_tree().
    // fn display_tree_helper(node: &Node, level: i32) -> Vec<(Dimension, f64, i32)> {
    // let dim = node.split_dimension.clone();
    // let split_val = node.split_value;
    // let mut to_return: Vec<(Dimension, f64, i32)> = vec![(dim, split_val, level)];
    // match node.left {
    // Some(ref node) => {
    // let mut tmp_vec = Node::display_tree_helper(node, level + 1);
    // to_return.append(&mut tmp_vec);
    // }
    // None => (),
    // }
    // match node.right {
    // Some(ref node) => {
    // let mut tmp_vec = Node::display_tree_helper(node, level + 1);
    // to_return.append(&mut tmp_vec);
    // }
    // None => (),
    // }
    // to_return
    // }
}

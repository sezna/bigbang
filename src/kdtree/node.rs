use kdtree::dimension::Dimension;
use kdtree::particle::Particle;
#[derive(Clone)]
pub struct Node {
    pub split_dimension: Option<Dimension>, // Dimension that this node splits at.
    pub split_value: f64,                   // Value that this node splits at.
    pub left: Option<Box<Node>>,            // Left subtree.
    pub right: Option<Box<Node>>,           // Right subtree.
    pub points: Option<Vec<Particle>>,      // Vector of the points if this node is a Leaf.
    pub center_of_mass: (f64, f64, f64), /* The center of mass for this node and it's children all
                                          * together. (x, y, z). */
    pub total_mass: f64, // Total mass of all particles under this node.
    pub r_max: f64,      // Maximum radius that is a child of this node.
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
            split_dimension: None,
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
    pub fn as_particle(&self) -> Particle {
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
        let x_distance = self.x_max - self.x_min;
        let y_distance = self.y_max - self.y_min;
        let z_distance = self.z_max - self.z_min;
        return f64::max(x_distance, f64::max(y_distance, z_distance));
    }
}

use crate::{Node, Particle};

/// The main struct. Contains a root node and the total number of particles. Sort of a wrapper for
/// the recursive node structure.
pub struct KDTree {
    pub root: Node,                 // The root Node.
    pub number_of_particles: usize, // The number of particles in the tree.
}

impl KDTree {
    pub fn new(pts: &mut Vec<Particle>) -> KDTree {
        let size_of_vec = pts.len();
        return KDTree {
            root: Node::new_root_node(pts),
            number_of_particles: size_of_vec,
        };
    }

    /// Traverses the tree and returns a vector of all particles in the tree.
    pub fn as_vec(&self) -> Vec<Particle> {
        let node = self.root.clone();
        let mut to_return: Vec<Particle> = Vec::new();
        match node.left {
            Some(ref node) => {
                println!("appended a particle left");
                to_return.append(&mut node.traverse_tree_helper());
            }
            None => (),
        }
        match node.right {
            Some(ref node) => {
                println!("appended a particlei right");
                to_return.append(&mut node.traverse_tree_helper());
            }
            None => {
                to_return.append(
                    &mut (node
                        .points
                        .as_ref()
                        .expect("unexpected null node #9")
                        .clone()),
                );
            }
        }
        return to_return;
        // return node.points.as_ref().expect("unexpected null vector of points");
    }

    /// This function creates a vector of all particles from the tree and applies gravity to them.
    /// Returns a new KDTree.
    // of note: The c++ implementation of this just stores a vector of
    // accelerations and matches up the
    // indexes with the indexes of the particles, and then applies them. That way
    // some memory is saved.
    // I am not sure if this will be necessary or very practical in the rust
    // implementation (I would have to implement indexing in my kdtree struct).
    pub fn tree_after_gravity(&self) -> KDTree {
        // TODO currently there is a time when the particles are stored twice.
        // Store only accelerations perhaps?
        let mut post_gravity_particle_vec: Vec<Particle> = self.root.traverse_tree_helper();
        for i in &mut post_gravity_particle_vec {
            *i = i.apply_gravity_from(&self.root);
        }
        return KDTree::new(&mut post_gravity_particle_vec);
    }
}

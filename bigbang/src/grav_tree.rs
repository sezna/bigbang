use crate::as_entity::AsEntity;
use crate::responsive::Responsive;
use crate::Node;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// The main struct you will interact with. This is a k-d tree containing all of your gravitational
/// entities.
#[derive(Serialize, Deserialize)]
pub struct GravTree<T: AsEntity + Responsive + Clone> {
    /// A GravTree consists of a root [[Node]]. A [[Node]] is a recursive binary tree data structure.
    /// Tragically must be public for now for testing reasons. Perhaps could be replaced by various
    /// getter methods later.
    pub(crate) root: Node<T>,
    /// This is just the number of entities in the tree. This is used in testing to verify that no
    /// entities are being dropped.
    number_of_entities: usize,
    /// This coefficient determines the granularity of the simulation, i.e. how much each frame of
    /// the simulation actually moves the individual entities.
    time_step: f64, // the time coefficient; how large each simulation frame is time-wise.
    /// The maximum number of entities to be contained within any leaf node. Defaults to 3 but is
    /// configurable. This is _not_ the maximum number of entities in the simulation. A higher
    /// number here will result in lower simulation granularity.
    max_entities: i32,
    /// `theta` is how far away a node has to be before the simulation starts approximating its
    /// contained entities by treating them as one large node instead of individually addressing
    /// them.
    /// More specifically, this is the tolerance for the distance from an entity to the center of mass of an entity
    /// If the distance is beyond this threshold, we treat the entire node as one giant
    /// entity instead of recursing into it.
    theta: f64,
}

impl<T: AsEntity + Responsive + Clone + Send + Sync> GravTree<T> {
    pub fn new(pts: &[T], time_step: f64, max_entities: i32, theta: f64) -> GravTree<T>
    where
        T: AsEntity,
    {
        let size_of_vec = pts.len();
        // Handle the case where a grav tree is initialized without any points...
        if size_of_vec == 0 {
            return GravTree {
                root: Node::new(),
                number_of_entities: size_of_vec,
                time_step,
                max_entities,
                theta,
            };
        }

        // Because of the tree's recursive gravity calculation, there needs to be a parent node
        // that "contains" the _real_ root node. This "phantom_parent" serves no purpose other than
        // to hold a pointer to the real root node. Perhaps not the most ideal situation for now,
        // and can be made more elegant in the future, if need be.
        // The real root of the tree is therefore tree.root.left
        let mut phantom_parent = Node::new();
        phantom_parent.left = Some(Box::new(Node::<T>::new_root_node(&pts[..], max_entities)));
        phantom_parent.points = Some(Vec::new());

        GravTree {
            root: phantom_parent,
            number_of_entities: size_of_vec,
            time_step,
            max_entities,
            theta,
        }
    }
    /// Sets the `theta` value of the simulation.
    pub fn set_theta(&mut self, theta: f64) {
        self.theta = theta;
    }

    /// Traverses the tree and returns a vector of all entities in the tree.
    pub fn as_vec(&self) -> Vec<T> {
        let node = self.root.clone();
        let mut to_return: Vec<T> = Vec::new();
        if let Some(node) = &node.left {
            to_return.append(&mut node.traverse_tree_helper());
        }
        if let Some(node) = &node.right {
            to_return.append(&mut node.traverse_tree_helper());
        } else {
            to_return.append(
                &mut (node
                    .points
                    .as_ref()
                    .expect("unexpected null node #9")
                    .clone()),
            );
        }
        to_return
    }
    /// Gets the total number of entities contained by this tree.
    pub fn get_number_of_entities(&self) -> usize {
        self.number_of_entities
    }

    /// This function creates a vector of all entities from the tree and applies gravity to them.
    /// Returns a new GravTree.
    // of note: The c++ implementation of this just stores a vector of
    // accelerations and matches up the
    // indexes with the indexes of the entities, and then applies them. That way
    // some memory is saved.
    // I am not sure if this will be necessary or very practical in the rust
    // implementation (I would have to implement indexing in my GravTree struct).
    pub fn time_step(&self) -> GravTree<T> {
        // TODO currently there is a time when the entities are stored twice.
        // Store only accelerations perhaps?
        // First, we extract the entities out into a vector
        let post_gravity_entity_vec: Vec<T> = self.root.traverse_tree_helper();
        // Then, we construct a new grav tree after the gravitational acceleration for each
        // entity has been calculated.
        GravTree::<T>::new(
            &post_gravity_entity_vec
                .par_iter()
                .map(|x| {
                    x.respond(
                        x.as_entity()
                            .get_acceleration_and_collisions(&self.root, self.theta),
                        self.time_step,
                    )
                })
                .collect::<Vec<_>>(),
            self.time_step,
            self.max_entities,
            self.theta,
        )
    }
}

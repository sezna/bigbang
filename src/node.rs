use crate::dimension::Dimension;
use crate::entity::{AsEntity, Entity};
use crate::utilities::{find_median, max_min_xyz, xyz_distances};
const MAX_PTS: i32 = 3;
#[derive(Clone)]

/// This is internal to the tree and is not exposed to the consumer.
///
/// A [[Node]] is either a leaf or an internal node. If it is an internal node, it contains _no_ entities. Instead,
/// it is purely structural and contains aggregate statistics of its children. An internal node will sometimes be treated
/// as an [[Entity]] of its own via `as_entity()` (not to be confused with the trait [[AsEntity]] -- this is just a method on
/// [[Node]].
///
/// If a [[Node]] is a leaf, then it contains up to `MAX_PTS` particles, as swell as the aggregate values of these particles.
/// These aggregate values are the center of mass, the total mass, and max/min values for each dimension.
pub struct Node<T: AsEntity + Clone> {
    split_dimension: Option<Dimension>, // Dimension that this node splits at.
    split_value: f64,                   // Value that this node splits at.
    pub left: Option<Box<Node<T>>>,     // Left subtree.
    pub right: Option<Box<Node<T>>>,    // Right subtree.
    pub points: Option<Vec<T>>,         // Vector of the points if this node is a Leaf.
    pub center_of_mass: (f64, f64, f64), /* The center of mass for this node and it's children all
                                         * together. (x, y, z). */
    total_mass: f64, // Total mass of all entities under this node.
    r_max: f64,      // Maximum radius that is a child of this node.
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
}

impl<T: AsEntity + Clone> Node<T> {
    pub fn new() -> Node<T> {
        Node {
            split_dimension: None,
            split_value: 0.0,
            left: None,
            right: None,
            points: None,
            center_of_mass: (0.0, 0.0, 0.0),
            total_mass: 0.0,
            r_max: 0.0,
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            z_min: 0.0,
            z_max: 0.0,
        }
    }
    /// Looks into its own children's maximum and minimum values, setting its own
    /// values accordingly.
    pub fn set_max_mins(&mut self) {
        let xmin = f64::min(
            self.left.as_ref().unwrap().x_min,
            self.right.as_ref().unwrap().x_min,
        );
        let xmax = f64::max(
            self.left.as_ref().unwrap().x_max,
            self.right.as_ref().unwrap().x_max,
        );
        let ymin = f64::min(
            self.left.as_ref().unwrap().y_min,
            self.right.as_ref().unwrap().y_min,
        );
        let ymax = f64::max(
            self.left.as_ref().unwrap().y_max,
            self.right.as_ref().unwrap().y_max,
        );
        let zmin = f64::min(
            self.left.as_ref().unwrap().z_min,
            self.right.as_ref().unwrap().z_min,
        );
        let zmax = f64::max(
            self.left.as_ref().unwrap().z_max,
            self.right.as_ref().unwrap().z_max,
        );
        let left_r_max = self.left.as_ref().expect("unexpected null node #7").r_max;
        let right_r_max = self.right.as_ref().expect("unexpected null node #8").r_max;
        self.r_max = f64::max(left_r_max, right_r_max);
        self.x_min = xmin;
        self.x_max = xmax;
        self.y_min = ymin;
        self.y_max = ymax;
        self.z_min = zmin;
        self.z_max = zmax;
    }
    // Used when treating a node as the sum of its parts in gravity calculations.
    /// Converts a node into an entity with the x, y, z, and mass being derived from the center of
    /// mass and the total mass of the entities it contains.
    pub fn as_entity(&self) -> Entity {
        // Construct a "super radius" of the largest dimension / 2 + a radius.
        let (range_x, range_y, range_z) = (
            self.x_max - self.x_min,
            self.y_max - self.y_min,
            self.z_max - self.z_min,
        );
        let max_dimension_range = f64::max(range_x, f64::max(range_y, range_z));
        let super_radius = max_dimension_range / 2f64 + self.r_max;
        // Center of mass is NaN a lot
        Entity {
            x: self.center_of_mass.0,
            y: self.center_of_mass.1,
            z: self.center_of_mass.2,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            mass: self.total_mass,
            radius: super_radius,
        }
    }

    pub fn max_distance(&self) -> f64 {
        let x_distance = self.x_max - self.x_min;
        let y_distance = self.y_max - self.y_min;
        let z_distance = self.z_max - self.z_min;
        f64::max(x_distance, f64::max(y_distance, z_distance))
    }

    /// Traverses tree and returns first child found with points.
    pub fn traverse_tree_helper(&self) -> Vec<T> {
        let mut to_return: Vec<T> = Vec::new();
        if let Some(node) = &self.left {
            to_return.append(&mut node.traverse_tree_helper());
        }
        if let Some(node) = &self.right {
            to_return.append(&mut node.traverse_tree_helper());
        } else {
            to_return.append(
                &mut (self
                    .points
                    .as_ref()
                    .expect("unexpected null node #10")
                    .clone()),
            );
        }
        to_return
    }

    /// Takes in a mutable slice of entities and creates a recursive 3d tree structure.
    pub fn new_root_node(pts: &[T]) -> Node<T> {
        // Start and end are probably 0 and pts.len(), respectively.
        let length_of_points = pts.len() as i32;
        let mut entities = pts.iter().map(|x| x.as_entity()).collect::<Vec<Entity>>();
        let (xdistance, ydistance, zdistance) = xyz_distances(entities.as_slice());
        // If our current collection is small enough to become a leaf (it has less than MAX_PTS points)
        if length_of_points <= MAX_PTS {
            // then we convert it into a leaf node.

            // we calculate the center of mass and total mass for each axis and store it as a three-tuple.
            // This admittedly terse `fold` used to be a for loop. I refactored it for the sake of immutability.
            // I'm still unsure if this was optimal.
            let (x_total, y_total, z_total, max_radius, total_mass) =
                // making this iterator parallel negatively impacts performance, at least for
                // bench_05 and bench_10
                pts.iter().fold((0.0, 0.0, 0.0, 0.0, 0.0), |acc, pt| {
                    let pt = pt.as_entity();
                    (
                        acc.0 + (pt.x * pt.mass),
                        acc.1 + (pt.y * pt.mass),
                        acc.2 + (pt.z * pt.mass),
                        if acc.3 > pt.radius { acc.3 } else { pt.radius },
                        acc.4 + pt.mass,
                    )
                });

            let (x_max, x_min, y_max, y_min, z_max, z_min) = max_min_xyz(&entities);
            Node {
                center_of_mass: (
                    x_total / total_mass as f64,
                    y_total / total_mass as f64,
                    z_total / total_mass as f64,
                ),
                total_mass,
                r_max: max_radius,
                points: Some(pts.to_vec()),
                left: None,
                right: None,
                split_dimension: None,
                split_value: 0.0,
                x_max: *x_max,
                x_min: *x_min,
                y_max: *y_max,
                y_min: *y_min,
                z_max: *z_max,
                z_min: *z_min,
            }
        // So the objective here is to find the median value for whatever axis has the greatest disparity in distance.
        // It is more efficient to pick three random values and pick the median of those as the pivot point, so that is
        // done if the vector has enough points. Otherwise, it picks the first element. FindMiddle just returns the middle
        // value of the three f64's given to it. Hopefully there is a more idomatic way to do this.
        } else {
            let mut root_node = Node::new();
            let split_index;
            let (split_dimension, split_value) = if zdistance > ydistance && zdistance > xdistance {
                // "If the z distance is the greatest"
                // split on Z
                let (split_value, tmp) = find_median(Dimension::Z, &mut entities);
                split_index = tmp;
                (Dimension::Z, split_value)
            } else if ydistance > xdistance && ydistance > zdistance {
                // "If the y distance is the greatest"
                // split on Y
                let (split_value, tmp) = find_median(Dimension::Y, &mut entities);
                split_index = tmp;
                (Dimension::Y, split_value)
            } else {
                // "If the x distance is the greatest"
                // split on X
                let (split_value, tmp) = find_median(Dimension::X, &mut entities);
                split_index = tmp;
                (Dimension::X, split_value)
            };
            root_node.split_dimension = Some(split_dimension);
            root_node.split_value = *split_value;
            let (below_split, above_split) = pts.split_at(split_index);

            // Now we construct the left and right children based on this split into lower and upper halves.
            let left = Node::new_root_node(&below_split);
            let right = Node::new_root_node(&above_split);
            // The center of mass is a recursive definition. This finds the average COM for
            // each node.
            let left_mass = left.total_mass;
            let right_mass = right.total_mass;
            let (left_x, left_y, left_z) = left.center_of_mass;
            let (right_x, right_y, right_z) = right.center_of_mass;
            let total_mass = left_mass + right_mass;
            assert!(total_mass != 0., "invalid mass of 0");

            let (center_x, center_y, center_z) = (
                ((left_mass * left_x) + (right_mass * right_x)) / total_mass,
                ((left_mass * left_y) + (right_mass * right_y)) / total_mass,
                ((left_mass * left_z) + (right_mass * right_z)) / total_mass,
            );
            root_node.left = Some(Box::new(left));
            root_node.right = Some(Box::new(right));
            root_node.center_of_mass = (center_x, center_y, center_z);
            root_node.set_max_mins();
            root_node.total_mass = total_mass;
            root_node
        }
    }
}

/// This tests the recursive node construction used to create a new gravtree. It tests some private
/// fields so it is located within the same module as the node itself.
#[test]
fn test() {
    let mut test_vec: Vec<Entity> = Vec::new();
    for i in 0..10 {
        test_vec.push(Entity {
            x: i as f64,
            y: (10 - i) as f64,
            z: i as f64,
            vx: i as f64,
            vy: i as f64,
            vz: i as f64,
            mass: i as f64,
            radius: i as f64,
        });
    }

    let check_vec = test_vec.clone();
    let tree = crate::GravTree::new(&test_vec, 0.2);
    let root_node = tree.root.clone();

    let mut nodes: Vec<Node<Entity>> = Vec::new();
    let mut traversal_stack: Vec<Option<Box<Node<Entity>>>> = Vec::new();
    let mut rover = Some(Box::new(root_node));
    while !traversal_stack.is_empty() || rover.is_some() {
        if rover.is_some() {
            traversal_stack.push(rover.clone());
            nodes.push(*rover.clone().unwrap());
            rover = rover.unwrap().left;
        } else {
            rover = traversal_stack.pop().unwrap();
            rover = rover.unwrap().right;
        }
    }

    let post_tree_vec = tree.as_vec();
    // The tree should contain all of the elements we put into it.
    for i in check_vec.iter() {
        assert!(post_tree_vec.contains(i));
    }

    // In this example, there should be exactly 7 nodes.
    assert_eq!(7, nodes.len());

    // No node should have zero mass.
    for node in nodes {
        assert!(node.total_mass > 0.);
    }

    // The total mass of the root node should be the sum of all of their masses.
    let total_mass = check_vec.iter().fold(0., |acc, x| acc + x.mass);
    assert_eq!(total_mass, tree.root.total_mass);
}

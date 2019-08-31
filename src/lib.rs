#![feature(test)]
extern crate either;
extern crate test;
// TODO list
// speed check compare the mutated accel value vs the recursive addition
// function that takes the acceleration on a particle and applies it
// function that puts all of the new particles into a new kdtree
pub mod dimension;
pub mod io;
pub mod node;
pub mod particle;
mod utilities;
use dimension::Dimension;
use node::Node;
use particle::Particle;
use utilities::*;
extern crate rand;
/// MAX_PTS represents the maximum amount of points allowed in a node.
const MAX_PTS: i32 = 3;

/// The main struct. Contains a root node and the total number of particles. Sort of a wrapper for
/// the recursive node structure.
pub struct KDTree {
    pub root: Node,                 // The root Node.
    pub number_of_particles: usize, // The number of particles in the tree.
}

pub fn new_kdtree(pts: &mut Vec<Particle>) -> KDTree {
    let size_of_vec = pts.len();
    return KDTree {
        root: new_root_node(pts),
        number_of_particles: size_of_vec,
    };
}

/// This function creates a vector of all particles from the tree and applies gravity to them.
/// Returns a new KDTree.
// of note: The c++ implementation of this just stores a vector of
// accelerations and matches up the
// indexes with the indexes of the particles, and then applies them. That way
// some memory is saved.
// I am not sure if this will be necessary or very practical in the rust
// implementation (I would have to implement indexing in my kdtree struct).
pub fn tree_after_gravity(node: &Node) -> KDTree {
    // TODO currently there is a time when the particles are stored twice.
    // Store only accelerations perhaps?
    let mut post_gravity_particle_vec: Vec<Particle> = traverse_tree_helper(node);
    for i in &mut post_gravity_particle_vec {
        *i = i.apply_gravity_from(node);
    }
    return new_kdtree(&mut post_gravity_particle_vec);
}

/// Takes in a mutable slice of particles and creates a recursive 3d tree structure.
fn new_root_node(pts: &mut [Particle]) -> Node {
    // Start and end are probably 0 and pts.len(), respectively.
    let length_of_points = pts.len() as i32;
    let (xdistance, ydistance, zdistance) = xyz_distances(pts);
    if length_of_points <= MAX_PTS {
        // Here we calculate the center of mass and total mass for each axis and store
        // it as a three-tuple.
        let mut count = 0;
        let mut total_mass = 0.0;
        let mut max_radius = 0.0;
        let (mut x_total, mut y_total, mut z_total) = (0.0, 0.0, 0.0);
        for point in pts.iter() {
            x_total = x_total + (point.x * point.mass); // add up the vector and weight it by mass
            y_total = y_total + (point.y * point.mass);
            z_total = z_total + (point.z * point.mass);
            total_mass = total_mass + point.mass;
            if point.radius > max_radius {
                max_radius = point.radius;
            }
            count = count + 1;
        }

        let (x_max, x_min, y_max, y_min, z_max, z_min) = max_min_xyz(pts);
        Node {
            center_of_mass: (
                x_total / total_mass as f64,
                y_total / total_mass as f64,
                z_total / total_mass as f64,
            ),
            total_mass: total_mass,
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
            let (split_value, tmp) = find_median(Dimension::Z, pts);
            split_index = tmp;
            (Dimension::Z, split_value)
        } else if ydistance > xdistance && ydistance > zdistance {
            // "If the x distance is the greatest"
            // split on Y
            let (split_value, tmp) = find_median(Dimension::Y, pts);
            split_index = tmp;
            (Dimension::Y, split_value)
        } else {
            // "If the y distance is the greatest"
            // split on X
            let (split_value, tmp) = find_median(Dimension::X, pts);
            split_index = tmp;
            (Dimension::X, split_value)
        };
        root_node.split_dimension = Some(split_dimension);
        root_node.split_value = *split_value;
        let (mut lower_vec, mut upper_vec) = pts.split_at_mut(split_index);
        root_node.left = Some(Box::new(new_root_node(&mut lower_vec)));
        root_node.right = Some(Box::new(new_root_node(&mut upper_vec)));
        // The center of mass is a recursive definition. This finds the average COM for
        // each node.
        let left_mass = root_node
            .left
            .as_ref()
            .expect("unexpected null node #3")
            .total_mass;
        let right_mass = root_node
            .right
            .as_ref()
            .expect("unexpected null node #4")
            .total_mass;
        let (left_x, left_y, left_z) = root_node
            .left
            .as_ref()
            .expect("unexpected null node #5")
            .center_of_mass;
        let (right_x, right_y, right_z) = root_node
            .right
            .as_ref()
            .expect("unexpected null node #6")
            .center_of_mass;
        let total_mass = left_mass + right_mass;
        let (center_x, center_y, center_z) = (
            ((left_mass * left_x) + (right_mass * right_x)) / total_mass,
            ((left_mass * left_y) + (right_mass * right_y)) / total_mass,
            ((left_mass * left_z) + (right_mass * right_z)) / total_mass,
        );
        root_node.center_of_mass = (center_x, center_y, center_z);
        // TODO refactor the next two lines, as they are a bit ugly
        root_node.set_max_mins();
        return root_node;
    }
}

/// Traverses the tree and returns a vector of all particles in the tree.
pub fn traverse_tree(tree: &KDTree) -> Vec<Particle> {
    let node = tree.root.clone();
    let mut to_return: Vec<Particle> = Vec::new();
    match node.left {
        Some(ref node) => {
            println!("appended a particle left");
            to_return.append(&mut traverse_tree_helper(node));
        }
        None => (),
    }
    match node.right {
        Some(ref node) => {
            println!("appended a particlei right");
            to_return.append(&mut traverse_tree_helper(node));
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
// Traverses tree and returns first child found with points.
pub fn traverse_tree_helper(node: &Node) -> Vec<Particle> {
    let mut to_return: Vec<Particle> = Vec::new();
    match node.left {
        Some(ref node) => {
            to_return.append(&mut traverse_tree_helper(node));
        }
        None => (),
    }
    match node.right {
        Some(ref node) => {
            to_return.append(&mut traverse_tree_helper(node));
        }
        None => {
            to_return.append(
                &mut (node
                    .points
                    .as_ref()
                    .expect("unexpected null node #10")
                    .clone()),
            );
        }
    }
    return to_return;
}

#[test]
fn test_traversal() {
    let mut vec: Vec<Particle> = Vec::new();
    for _ in 0..100 {
        let particle = Particle::random_particle();
        vec.push(particle);
    }
    let vec_clone = vec.clone();
    let tree = new_kdtree(&mut vec);
    let traversed_vec = traverse_tree(&tree);
    let mut all_found = true;
    for i in vec_clone {
        if !traversed_vec.contains(&i) {
            println!("particle not found");
            all_found = false;
        }
    }

    assert!(all_found);
}

#[test]
/// Really lame test for Particle::new()
fn test() {
    let test_particle = Particle::new();
    assert!(
        Particle {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            radius: 0.0,
            mass: 0.0,
        } == test_particle
    );
}
#[test]
#[allow(dead_code)]
fn test_tree() {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Particle> = Vec::new();
    for _ in 0..100 {
        for _ in 0..100 {
            for _ in 0..10 {
                let particle = Particle::random_particle();
                vec_that_wants_to_be_a_kdtree.push(particle);
            }
        }
    }
    let kdtree_test = new_kdtree(&mut vec_that_wants_to_be_a_kdtree);
    assert!(kdtree_test.number_of_particles == 100000);
    // kdtree_test.display_tree();
    println!("testing integrity of the big tree\n");
    go_to_edges(kdtree_test);
    let mut smaller_vec: Vec<Particle> = Vec::new();
    println!("displaying a smaller tree\n");
    for _ in 0..50 {
        let particle = Particle::random_particle();
        smaller_vec.push(particle);
    }
    // Testing center of mass assignment
    let mut vector = vec![
        Particle {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            x: 1.0,
            y: 2.0,
            z: 3.0,
            mass: 2.0,
            radius: 1.0,
        },
        Particle {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            x: 2.0,
            y: 1.0,
            z: 3.0,
            mass: 2.0,
            radius: 1.0,
        },
    ];
    let center_of_mass_test = new_kdtree(&mut vector);
    assert!(center_of_mass_test.root.center_of_mass == (1.5, 1.5, 3.0));
}
#[allow(dead_code)]
fn go_to_edges(kdtree: KDTree) {
    let mut count_of_nodes = 0;
    let mut node = kdtree.root.left.expect("null root node\n");
    let mut node2 = node.clone();
    while node.left.is_some() {
        count_of_nodes = count_of_nodes + 1;
        node = node.left.expect("unexpected null node #1\n");
    }
    println!("number of nodes on left: {}\n", count_of_nodes);
    assert!(count_of_nodes == 14);
    count_of_nodes = 0;
    while node2.right.is_some() {
        count_of_nodes = count_of_nodes + 1;
        node2 = node2.right.expect("unexpected null node #2\n");
    }
    println!("number of nodes on right: {}\n", count_of_nodes);
    assert!(count_of_nodes == 15);
}
#[test]
fn test_input() {
    let test_vec = io::open_data_file("test_files/test_input.txt".to_string());
    println!("test_vec len: {}", test_vec.len());
    assert!(test_vec.len() == 3601);
}
#[test]
fn test_output() {
    let mut test_vec: Vec<Particle> = Vec::new();
    for _ in 0..1000 {
        test_vec.push(Particle::random_particle());
    }
    let kd = new_kdtree(&mut test_vec);
    io::write_data_file(kd, "test_files/test_output.txt".to_string());
    let read_file = io::open_data_file("test_files/test_output.txt".to_string());
    println!(
        "test_vec.len() = {} read_file.len() = {}",
        test_vec.len(),
        read_file.len()
    );
    assert!(test_vec.len() == read_file.len());
}

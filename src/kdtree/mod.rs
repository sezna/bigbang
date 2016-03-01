//TODO list
// speed check compare the mutated accel value vs the recursive addition
mod particle;
mod test;
mod utilities;
mod node;
mod dimension;
use kdtree::utilities::*;
use kdtree::particle::Particle;
use kdtree::dimension::Dimension;
use kdtree::node::Node;
extern crate rand;
const max_pts:i32 = 3;
const theta: f64 = 0.2;

pub struct KDTree {
    root: Node, // The root Node.
    number_of_particles: usize, // The number of particles in the tree.
}
impl KDTree {
    pub fn display_tree(&self) {
        self.root.display_tree();
    }
    pub fn iterate_over_nodes(&self) -> Vec<Node> {
        let node = self.root.clone();
        return node.iterate_over_nodes();
    }
}
pub fn new_kdtree(pts: &mut Vec<Particle>) -> KDTree {
    let size_of_vec = pts.len();
    return KDTree {
        root: new_root_node(pts),
        number_of_particles: size_of_vec,
    };
}
/// Returns a boolean representing whether or node the node is within the theta range
/// of the particle. 
fn theta_exceeded(particle:&Particle, node: &Node) -> bool {
// 1) distance from particle to COM of that node
// 2) if 1) * theta > size (max diff) then
    let node_as_particle = node.to_particle();
    let dist = particle.distance(&node_as_particle);
    let x_max_min = max_min_x(&node.points.as_ref().expect(""));
    let y_max_min = max_min_y(&node.points.as_ref().expect(""));
    let z_max_min = max_min_z(&node.points.as_ref().expect(""));
    let x_distance = (x_max_min.0 - x_max_min.1).abs();
    let y_distance = (x_max_min.0 - x_max_min.1).abs();
    let z_distance = (x_max_min.0 - x_max_min.1).abs();
    let max_dist = f64::max(x_distance, f64::max(y_distance, z_distance));
    return dist * theta  > max_dist;
}

/// Applies force to a node.
fn get_gravitational_acceleration_node(particle: &Particle, other: &Node) -> (f64, f64, f64) { 
    let node_as_particle = other.to_particle();
    let d_magnitude = particle.distance(&node_as_particle);
    let d_vector = particle.distance_vector(&node_as_particle);
    let d_over_d_cubed = (d_vector.0 / d_magnitude.powf(2.0),
                          d_vector.1 / d_magnitude.powf(2.0),
                          d_vector.2 / d_magnitude.powf(2.0));
    let acceleration = (d_over_d_cubed.0 * node_as_particle.mass,
                        d_over_d_cubed.1 * node_as_particle.mass,
                        d_over_d_cubed.2 * node_as_particle.mass);
    return acceleration;
}
fn get_gravitational_acceleration_particle(particle: &Particle, other: &Particle) -> (f64, f64, f64) { 
    let d_magnitude = particle.distance(other);
    let d_vector = particle.distance_vector(other);
    let d_over_d_cubed = (d_vector.0 / d_magnitude.powf(2.0),
                          d_vector.1 / d_magnitude.powf(2.0),
                          d_vector.2 / d_magnitude.powf(2.0));
    let acceleration = (d_over_d_cubed.0 * other.mass,
                        d_over_d_cubed.1 * other.mass,
                        d_over_d_cubed.2 * other.mass);
    return acceleration;

}

pub fn apply_gravity(tree: KDTree) -> KDTree { //TODO - avoid having three copies of the particles at once.
    let mut vec = traverse_tree(&tree);
    let mut return_vec:Vec<Particle> = Vec::new();
    let mut tmp_accel = (0.0,0.0,0.0);
    let mut acceleration = (0.0,0.0,0.0);
    for particle in vec {
        for node in tree.iterate_over_nodes() {
            if theta_exceeded(&particle, &node) { // If the theta value has been exceeded, take the
                                                  // acceleration from the node. Else, if there are
                                                  // particles, go through those particles.
                    tmp_accel = get_gravitational_acceleration_node(&particle, &node);
                    acceleration.0 = acceleration.0 + tmp_accel.0;
                    acceleration.1 = acceleration.1 + tmp_accel.1;
                    acceleration.2 = acceleration.2 + tmp_accel.2;
                }
            else if node.points.is_some(){
                for i in node.points.expect("") {
                    tmp_accel = get_gravitational_acceleration_particle(&particle, &i);
                    acceleration.0 = acceleration.0 + tmp_accel.0;
                    acceleration.1 = acceleration.1 + tmp_accel.1;
                    acceleration.2 = acceleration.2 + tmp_accel.2;
                }
            }
        }
        let mut new_particle = particle.clone();
        new_particle.x = particle.x + acceleration.0;
        new_particle.y = particle.y + acceleration.1;
        new_particle.z = particle.z + acceleration.2;
        return_vec.push(new_particle);
    }
    
    return new_kdtree(&mut return_vec);
}

/// Returns the particle after it has had gravity from the tree applied to it.
fn particle_gravity(node: &Node, particle: &Particle) -> Particle {
	match node.left {
		Some(ref node) => {
		    if theta_exceeded(&particle, &node) {  //TODO
                    let tmp_accel = get_gravitational_acceleration_node(&particle, &node);
                    acceleration.0 = acceleration.0 + tmp_accel.0;
                    acceleration.1 = acceleration.1 + tmp_accel.1;
                    acceleration.2 = acceleration.2 + tmp_accel.2;
            }
            else if node.points.is_some() {
                for i in node.points.expect("") {
                    tmp_accel - get_gravitational_acceleration_particle(&particle, &node);
                    acceleration.0 = acceleration.0 + tmp_accel.0;
                    acceleration.1 = acceleration.1 + tmp_accel.1;
                    acceleration.2 = acceleration.2 + tmp_accel.2;
                }
            }
		}

		None => (),
	}
	match node.right {
		Some(ref node) => {
            println!("appended a particlei right");
		    to_return.append(&mut traverse_tree_helper(node));
		}
		None => {
            to_return.append(&mut (node.points.as_ref().expect("unexpected null node #something").clone()));
        }
    }
    return to_return;
}

fn new_root_node(pts: &mut [Particle]) -> Node {
    // Start and end are probably 0 and pts.len(), respectively.
    // Should this function recurse by splitting the vectors, or by
    // passing pointers to areas in the vector without mutating it?
    let start = 0 as usize;
    let end = pts.len();
    let length_of_points = pts.len() as i32;
    let (xmax, xmin) = max_min_x(pts);
    let (ymax, ymin) = max_min_y(pts);
    let (zmax, zmin) = max_min_z(pts);
    let xdistance = (xmax - xmin).abs();
    let ydistance = (ymax - ymin).abs();
    let zdistance = (zmax - zmin).abs();
    if length_of_points <= max_pts {
        let mut root_node = Node::new();
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
        root_node.center_of_mass = (x_total / total_mass as f64,
                                    y_total / total_mass as f64,
                                    z_total / total_mass as f64);
        root_node.total_mass = total_mass;
        root_node.r_max = max_radius;
        root_node.points = Some(pts.to_vec());
        return root_node;
        // So the objective here is to find the median value for whatever axis has the
        // greatest
        // disparity in distance. It is more efficient to pick three random values and
        // pick the
        // median of those as the pivot point, so that is done if the vector has enough
        // points.
        // Otherwise, it picks the first element. FindMiddle just returns the middle
        // value of the
        // three f64's given to it. Hopefully there is a more idomatic way to do this.
    } else {
        let mut root_node = Node::new();
        let split_index;
        let mid = (start + end) / 2 as usize;
        if zdistance > ydistance && zdistance > xdistance {
            // "If the z distance is the greatest"
            // split on Z
            let (split_value, tmp) = find_median_z(pts, start, end, mid);
            split_index = tmp;
            root_node.split_dimension = Dimension::Z;
            root_node.split_value = split_value;
        } else if ydistance > xdistance && ydistance > zdistance {
            // "If the x distance is the greatest"
            // split on Y
            let (split_value, tmp) = find_median_y(pts, start, end, mid);
            split_index = tmp;
            root_node.split_dimension = Dimension::Y;
            root_node.split_value - split_value;
        } else {
            // "If the y distance is the greatest"
            // split on X
            let (split_value, tmp) = find_median_x(pts, start, end, mid);
            split_index = tmp;
            root_node.split_dimension = Dimension::X;
            root_node.split_value = split_value;
        }
        let (mut lower_vec, mut upper_vec) = pts.split_at_mut(split_index);
//        pts.shrink_to_fit(); // Memory efficiency!
        root_node.left = Some(Box::new(new_root_node(&mut lower_vec)));
        root_node.right = Some(Box::new(new_root_node(&mut upper_vec)));
        // The center of mass is a recursive definition. This finds the average COM for
        // each node.
        let left_mass = root_node.left
                                 .as_ref()
                                 .expect("unexpected null node #1")
                                 .total_mass;
        let right_mass = root_node.right
                                  .as_ref()
                                  .expect("unexpected null node #2")
                                  .total_mass;
        let (left_x, left_y, left_z) = root_node.left
                                                .as_ref()
                                                .expect("unexpected null node #3")
                                                .center_of_mass;
        let (right_x, right_y, right_z) = root_node.right
                                                   .as_ref()
                                                   .expect("unexpected null node #4")
                                                   .center_of_mass;
        let total_mass = left_mass + right_mass;
        let (center_x, center_y, center_z) = (((left_mass * left_x) + (right_mass * right_x)) /
                                              total_mass,
                                              ((left_mass * left_y) + (right_mass * right_y)) /
                                              total_mass,
                                              ((left_mass * left_z) + (right_mass * right_z)) /
                                              total_mass);
        root_node.center_of_mass = (center_x, center_y, center_z);
        // TODO refactor the next two lines, as they are a bit ugly
        let left_r_max = root_node.left.as_ref().expect("unexpected null node #9").r_max;
        let right_r_max = root_node.right.as_ref().expect("unexpected null node #10").r_max;
        let max_r_max = f64::max(left_r_max, right_r_max);
        root_node.r_max = max_r_max;
        return root_node;
    }
}


pub fn traverse_tree(tree:&KDTree) -> Vec<Particle>{
	let node = tree.root.clone();
	let mut to_return:Vec<Particle> = Vec::new();
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
            to_return.append(&mut (node.points.as_ref().expect("unexpected null node #something").clone()));
        }
    }
    return to_return;
//	return node.points.as_ref().expect("unexpected null vector of points");

}
// Traverses tree and returns first child found with points. 
pub fn traverse_tree_helper(node: &Node) -> Vec<Particle> {
    let mut to_return:Vec<Particle> = Vec::new();
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
            to_return.append(&mut (node.points.as_ref().expect("unexpected null node #something").clone()));
        }
    }
    return to_return;
}
#[test]
fn test_traversal() {
    let mut vec:Vec<Particle> = Vec::new();
    for x in 0..100 {
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
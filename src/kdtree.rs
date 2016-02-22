extern crate rand;
const theta: f64 = 0.2;
#[derive(Clone, PartialEq)]
enum Dimension {
    X,
    Y,
    Z,
    Null,
}
impl Dimension {
    pub fn as_string(&self) -> &str {
        match self {
            &Dimension::X => return "X",
            &Dimension::Y => return "Y",
            &Dimension::Z => return "Z",
            _ => return "Null",
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct Particle {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
    pub mass: f64,
}
impl Particle {
    pub fn random_particle() -> Particle {
        return Particle {
            vx: rand::random::<f64>(),
            vy: rand::random::<f64>(),
            vz: rand::random::<f64>(),
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
            radius: rand::random::<f64>(),
            mass: rand::random::<f64>(),
        };

    }
}
#[derive(Clone)]
pub struct Node {
    split_dimension: Dimension,      // Dimension that this node splits at.
    split_value: f64,                // Value that this node splits at.
    left: Option<Box<Node>>,         // Left subtree.
    right: Option<Box<Node>>,        // Right subtree.
    points: Option<Vec<Particle>>,   // Vector of the points if this node is a Leaf.
    center_of_mass: (f64, f64, f64), /* The center of mass for this node and it's children all
                                      * together. (x, y, z). */
    total_mass: f64,                 // Total mass of all particles under this node.
    r_max: f64,                      // Maximum radius that is a child of this node.
}

impl Node {
    fn new() -> Node {
        return Node {
            split_dimension: Dimension::Null,
            split_value: 0.0,
            left: None,
            right: None,
            points: None,
            center_of_mass: (0.0, 0.0, 0.0), // (pos * mass) + (pos * mass) / sum of masses
            total_mass: 0.0,
            r_max: 0.0,
        };
    }
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
// pub fn that takes in a particle and then returns an acceleration
// delta p vector is equal to position of particle minus center of mass
// 1) distance from particle to COM of that node
// 2) if 1) * theta > size (max diff) then
//      return delta p vector times (m_i * m_com) / (delta p) ^ 3
//    else if leaf node then
//      loop through particles, summing acceleration
//      else if not leaf node
//          recurse to left and right
//
// 2 new) if leaf, loop through contents
//        else if not leaf, then distance check
//
// speed check compare the mutated accel value vs the recursive addition
//


pub struct KDTree {
    root: Node, // The root Node.
    number_of_particles: usize, // The number of particles in the tree.
    max_points: i32, // The maximum number of particles in one Node.
}
impl KDTree {
    pub fn display_tree(&self) {
        self.root.display_tree();
    }
}
pub fn new_kdtree(pts: &mut Vec<Particle>, max_pts: i32) -> KDTree {
    let size_of_vec = pts.len();
    return KDTree {
        root: new_root_node(pts, max_pts),
        number_of_particles: size_of_vec,
        max_points: max_pts,
    };
}
pub fn apply_gravity(tree: KDTree) -> KDTree { //TODO
    let mut father_node = Some(Box::new(tree.root));
    while father_node.is_some() { // Iterate through the tree until a leaf is reached.
        
    }
    return new_kdtree(&mut vec![Particle::random_particle()], 2);
}


fn new_root_node(pts: &mut [Particle], max_pts: i32) -> Node {
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
        root_node.left = Some(Box::new(new_root_node(&mut lower_vec, max_pts)));
        root_node.right = Some(Box::new(new_root_node(&mut upper_vec, max_pts)));
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


// The following three functions just return a tuple of the maximum
// and minimum values in the dimensions. Perhaps it could use a
// refactor, as there is a lot of copied code.
fn max_min_x(particles: &[Particle]) -> (f64, f64) {
    let mut to_return_max = 0.0;
    let mut to_return_min = particles[0].x;
    for i in particles {
        if i.x > to_return_max {
            to_return_max = i.x;
        }
        if i.x < to_return_min {
            to_return_min = i.x;
        }
    }
    return (to_return_max, to_return_min);
}

fn max_min_y(particles: &[Particle]) -> (f64, f64) {
    let mut to_return_max = 0.0;
    let mut to_return_min = particles[0].y;
    for i in particles {
        if i.y > to_return_max {
            to_return_max = i.y;
        }
        if i.y < to_return_min {
            to_return_min = i.y;
        }
    }
    return (to_return_max, to_return_min);
}

fn max_min_z(particles: &[Particle]) -> (f64, f64) {
    let mut to_return_max = 0.0;
    let mut to_return_min = particles[0].z;
    for i in particles {
        if i.z > to_return_max {
            to_return_max = i.z;
        }
        if i.z < to_return_min {
            to_return_min = i.z;
        }
    }
    return (to_return_max, to_return_min);
}
// The following three functions just find median points  for the x, y, or z
// dimension. Perhaps it could use a refactor, because there is a lot of copied
// code. They return a tuple of the value being split at and the index being
// split at.
fn find_median_z(pts: &mut [Particle], start: usize, end: usize, mid: usize) -> (f64, usize) {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].z < pts[start].z {
            low = low + 1;
        } else {
            let tmp = pts[low].clone();
            pts[low] = pts[high].clone();
            pts[high] = tmp;
            high -= 1;
        }
    }
    let tmp = pts[high].clone();
    pts[high] = pts[start].clone();
    pts[start] = tmp;
    if start == mid {
        return (pts[start].z, start);
    } else if high < mid {
        return find_median_z(pts, high + 1, end, mid);
    } else {
        return find_median_z(pts, start, high, mid);
    }
}
fn find_median_y(pts: &mut [Particle], start: usize, end: usize, mid: usize) -> (f64, usize) {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].y < pts[start].y {
            low = low + 1;
        } else {
            let tmp = pts[low].clone();
            pts[low] = pts[high].clone();
            pts[high] = tmp;
            high -= 1;
        }
    }
    let tmp = pts[high].clone();
    pts[high] = pts[start].clone();
    pts[start] = tmp;
    if start == mid {
        return (pts[start].y, start);
    } else if high < mid {
        return find_median_y(pts, high + 1, end, mid);
    } else {
        return find_median_y(pts, start, high, mid);
    }
}
fn find_median_x(pts: &mut [Particle], start: usize, end: usize, mid: usize) -> (f64, usize) {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].x < pts[start].x {
            low = low + 1;
        } else {
            let tmp = pts[low].clone();
            pts[low] = pts[high].clone();
            pts[high] = tmp;
            high -= 1;
        }
    }
    let tmp = pts[high].clone();
    pts[high] = pts[start].clone();
    pts[start] = tmp;
    if start == mid {
        return (pts[start].x, start);
    } else if high < mid {
        return find_median_x(pts, high + 1, end, mid);
    } else {
        return find_median_x(pts, start, high, mid);
    }
}


pub fn traverse_tree(tree:KDTree) -> Vec<Particle>{
	let node = tree.root;
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
    let tree = new_kdtree(&mut vec, 2);
    let traversed_vec = traverse_tree(tree);
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
#[allow(dead_code)]
fn test_tree() {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Particle> = Vec::new();
    for x in 0..100 {
        for y in 0..100 {
            for z in 0..10 {
                let particle = Particle::random_particle();
                vec_that_wants_to_be_a_kdtree.push(particle);
            }
        }
    }
    let kdtree_test = new_kdtree(&mut vec_that_wants_to_be_a_kdtree, 3);
    assert!(kdtree_test.number_of_particles == 100000);
    assert!(kdtree_test.max_points == 3);
    // kdtree_test.display_tree();
    println!("testing integrity of the big tree\n");
    go_to_edges(kdtree_test);
    let mut smaller_vec: Vec<Particle> = Vec::new();
    println!("displaying a smaller tree\n");
    for z in 0..50 {
        let particle = Particle::random_particle();
        smaller_vec.push(particle);
    }
    let smaller_kdtree = new_kdtree(&mut smaller_vec, 10);
    smaller_kdtree.display_tree();
    // Testing center of mass assignment
    let mut vector = vec![Particle {
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
                      }];
    let center_of_mass_test = new_kdtree(&mut vector, 2);
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

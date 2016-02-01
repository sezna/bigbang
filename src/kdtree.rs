extern crate rand;
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
            _             => return "Null",
        };
    }
}
#[derive(Clone)]
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
/*    pub fn clone(&self) -> Particle {
        return Particle {
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            x: self.x,
            y: self.y,
            z: self.z,
        };
    }*/
    pub fn random_particle() -> Particle {
        let mut rng = rand::thread_rng();
        return Particle {
            vx: rand::random::<f64>(),
            vy: rand::random::<f64>(),
            vz: rand::random::<f64>(),
            x:  rand::random::<f64>(),
            y:  rand::random::<f64>(),
            z:  rand::random::<f64>(),
            radius: rand::random::<f64>(),
            mass: rand::random::<f64>(),
        }

    }
}
#[derive(Clone)]
pub struct Node {
    split_dimension: Dimension,             // Dimension that this node splits at.
    split_value:     f64,                   // Value that this node splits at.
    left:            Option<Box<Node>>,     // Left subtree.
    right:           Option<Box<Node>>,     // Right subtree.
    points:          Option<Vec<Particle>>, // Vector of the points if this node is a Leaf.
    center_of_mass:  (f64, f64, f64),       // The center of mass for this node and it's children all together. (x, y, z).
    total_mass:      f64,                   // Total mass of all particles under this node.
    r_max:           f64,                   // Maximum radius that is a child of this node.
}
impl Node {
    fn new() -> Node {
        return Node {
        split_dimension: Dimension::Null,
        split_value: 0.0,
        left: None,
        right: None,
        points: None,
        center_of_mass: (0.0, 0.0, 0.0),
        total_mass: 0.0,
        r_max: 0.0
        }
    }
    
    
    pub fn display_tree(&self) {
        let mut to_display = Node::display_tree_helper(self, 0);
        to_display.sort_by(|a, b| (a.2).cmp(&b.2));
        let mut to_display_string:String = "".to_string();
        let mut prev:i32 = -1;
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
        let mut dim = node.split_dimension.clone();
        let mut split_val = node.split_value;
        let mut to_return:Vec<(Dimension, f64, i32)> = vec![(dim, split_val, level)];
        match node.left {
            Some(ref node) => {
                let mut tmp_vec =  Node::display_tree_helper(node, level + 1);
                to_return.append(&mut tmp_vec);
            },
            None => (),
        }
        
        match node.right {
            Some(ref node) => {
                let mut tmp_vec =  Node::display_tree_helper(node, level + 1);
                to_return.append(&mut tmp_vec);
            },
            None => (),
        }
        to_return
    }

}
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
pub fn new_kdtree(pts: Vec<Particle>, max_pts: i32) -> KDTree {
    let size_of_vec = pts.len();
    return KDTree {
        root: new_root_node(pts, max_pts),
        number_of_particles: size_of_vec,
        max_points: max_pts,
    };
}
fn new_root_node(mut pts: Vec<Particle>, max_pts: i32) -> Node {
    // Start and end are probably 0 and pts.len(), respectively.
    let start = 0 as usize;
    let end = pts.len();
    let length_of_points = pts.len() as i32;
    let (xmax, xmin) = max_min_x(&pts);
    let (ymax, ymin) = max_min_y(&pts);
    let (zmax, zmin) = max_min_z(&pts);
    let xdistance = (xmax - xmin).abs();
    let ydistance = (ymax - ymin).abs();
    let zdistance = (zmax - zmin).abs();
    if length_of_points <= max_pts {
        let mut root_node = Node::new();
        // Here we calculate the center of mass and total mass for each axis and store it as a three-tuple.
        let mut count = 0;
        let mut total_mass = 0.0;
        let mut max_radius = 0.0;
        let (mut x_total, mut y_total, mut z_total) = (0.0, 0.0, 0.0);
        for point in &pts {
            x_total = x_total + point.x;
            y_total = y_total + point.y;
            z_total = z_total + point.z;
            total_mass = total_mass + point.mass;
            if point.radius > max_radius {
                max_radius = point.radius;
            }
            count = count + 1;
        }
        // TODO weight center of mass by actual mass
        root_node.center_of_mass = (x_total / count as f64, y_total / count as f64, z_total / count
                                    as f64);
        root_node.total_mass = total_mass;
        root_node.r_max = max_radius;
        root_node.points = Some(pts);
        return root_node;
        // So the objective here is to find the median value for whatever axis has the greatest
        // disparity in distance. It is more efficient to pick three random values and pick the
        // median of those as the pivot point, so that is done if the vector has enough points.
        // Otherwise, it picks the first element. FindMiddle just returns the middle value of the
        // three f64's given to it.
    } else {
        let mut root_node = Node::new();
        let split_index;
        let mid = (start + end) / 2 as usize; 
        if zdistance > ydistance && zdistance > xdistance {
            // "If the z distance is the greatest"
            // split on Z
            let (split_value, tmp) = find_median_z(&mut pts, start, end, mid);
            split_index = tmp;
            root_node.split_dimension = Dimension::Z;
            root_node.split_value = split_value;
        } else if ydistance > xdistance && ydistance > zdistance {
            // "If the x distance is the greatest"
            // split on Y
            let (split_value, tmp) = find_median_y(&mut pts, start, end, mid);
            split_index = tmp;
            root_node.split_dimension = Dimension::Y;
            root_node.split_value - split_value;
        } else {
            // "If the y distance is the greatest"
            // split on X
            let (split_value, tmp) = find_median_x(&mut pts, start, end, mid);
            split_index = tmp;
            root_node.split_dimension = Dimension::X;
            root_node.split_value = split_value;
        }
        let upper_vec = pts.split_off(split_index);
        pts.shrink_to_fit(); // Memory efficiency!
        root_node.left = Some(Box::new(new_root_node(pts, max_pts)));
        root_node.right = Some(Box::new(new_root_node(upper_vec, max_pts)));
        // The center of mass is a recursive definition. This finds the average COM for each node.
        let center_of_mass_x = (root_node.left.as_ref().expect("unexpected null node #1 ").center_of_mass.0 + root_node.right.as_ref().expect("unexpected null node #4").center_of_mass.0) / 2.0;
        let center_of_mass_y = (root_node.left.as_ref().expect("unexpected null node #2").center_of_mass.1 + root_node.right.as_ref().expect("unexpected null node #5").center_of_mass.1) / 2.0;
        let center_of_mass_z = (root_node.left.as_ref().expect("unexpected null node #3").center_of_mass.2 + root_node.right.as_ref().expect("unexpected null node #6").center_of_mass.2) / 2.0;
        root_node.center_of_mass = (center_of_mass_x, center_of_mass_y, center_of_mass_z);
        root_node.total_mass = root_node.left.as_ref().expect("unexpected null node #7").total_mass + root_node.right.as_ref().expect("unexpected null node #8").total_mass;
        // TODO refactor the next two lines, as they are a bit ugly
        root_node.r_max = if root_node.left.as_ref().expect("unexpected null node #9").r_max >
            root_node.right.as_ref().expect("unexpected null node #10").r_max { root_node.left.as_ref().expect("unexpected null node #9").r_max} else  {root_node.right.as_ref().expect("unexpected null node #10").r_max};
        return root_node;
    }
}


// The following three functions just return a tuple of the maximum and minimum
// values in the
// dimensions. Perhaps it could use a refactor, as there is a lot of copied
// code.
fn max_min_x(particles: &Vec<Particle>) -> (f64, f64) {
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
fn max_min_y(particles: &Vec<Particle>) -> (f64, f64) {
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

fn max_min_z(particles: &Vec<Particle>) -> (f64, f64) {
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
fn find_median_z(pts: &mut Vec<Particle>, start: usize, end: usize, mid: usize) -> (f64, usize) {
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
fn find_median_y(pts: &mut Vec<Particle>, start: usize, end: usize, mid: usize) -> (f64, usize) {
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
fn find_median_x(pts: &mut Vec<Particle>, start: usize, end: usize, mid: usize) -> (f64, usize) {
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



#[test]
#[allow(dead_code)]
fn test_tree() {
   let mut rng = rand::thread_rng();
   let mut vec_that_wants_to_be_a_kdtree:Vec<Particle> = Vec::new();
   for x in 0..100 {
        for y in 0..100 {
            for z in 0..10 {
                let particle = Particle{
                vx: rand::random::<f64>(),
                vy: rand::random::<f64>(),
                vz: rand::random::<f64>(),
                x: x as f64,
                y: y as f64,
                z: z as f64,
                radius: rand::random::<f64>(),
                mass: rand::random::<f64>(),
                };
                vec_that_wants_to_be_a_kdtree.push(particle);
            }
        }
   }
   let kdtree_test = new_kdtree(vec_that_wants_to_be_a_kdtree, 3);
   assert!(kdtree_test.number_of_particles == 100000);
   assert!(kdtree_test.max_points == 3);
   //kdtree_test.display_tree();
   println!("testing integrity of the big tree\n");
   go_to_edges(kdtree_test);
   let mut smaller_vec:Vec<Particle> = Vec::new();
   println!("displaying a smaller tree\n");
            for z in 0..50 {
                let particle = Particle{
                vx: rand::random::<f64>(),
                vy: rand::random::<f64>(),
                vz: rand::random::<f64>(),
                x: rand::random::<f64>(),
                y: rand::random::<f64>(),
                z: rand::random::<f64>(),
                mass: rand::random::<f64>(),
                radius: rand::random::<f64>(),
                };
                smaller_vec.push(particle);
   }
    let smaller_kdtree = new_kdtree(smaller_vec, 10);
    smaller_kdtree.display_tree();
    // Testing center of mass assignment
    let vector = vec![Particle{vx: 0.0, vy: 0.0, vz: 0.0, x: 1.0, y: 2.0, z: 3.0, mass: 2.0, radius: 1.0}, Particle{vx: 0.0, vy: 0.0, vz: 0.0, x: 2.0, y: 1.0, z: 3.0, mass: 2.0, radius: 1.0}];
    let center_of_mass_test = new_kdtree(vector, 2);
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


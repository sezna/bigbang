extern crate rand;
enum Dimension {
    X,
    Y,
    Z,
    Null,
}
pub struct Particle {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Particle {
    pub fn clone(&self) -> Particle {
        return Particle {
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            x: self.x,
            y: self.y,
            z: self.z,
        };
    }
    pub fn random_particle() -> Particle {
        let mut rng = rand::thread_rng();
        return Particle {
            vx: rand::random::<f64>(),
            vy: rand::random::<f64>(),
            vz: rand::random::<f64>(),
            x:  rand::random::<f64>(),
            y:  rand::random::<f64>(),
            z:  rand::random::<f64>(),
        }

    }
}
pub struct Node {
    split_dimension: Dimension,
    split_value: f64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    points: Option<Vec<Particle>>,
}
impl Node {
    fn new() -> Node {
        return Node {
        split_dimension: Dimension::Null,
        split_value: 0.0,
        left: None,
        right: None,
        points: None
        }
    }
}
pub struct KDTree {
    root: Node, // The root Node.
    number_of_particles: usize, // The number of particles in the tree.
    max_points: i32, // The maximum number of particles in one Node.
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
        let root_node = Node {
            split_dimension: Dimension::Null,
            split_value: 0.0,
            left: None,
            right: None,
            points: Some(pts),
        };
        return root_node;
        // So the objective here is to find the median value for whatever axis has the greatest
        // disparity in distance. It is more efficient to pick three random values and pick the
        // median of those as the pivot point, so that is done if the vector has enough points.
        // Otherwise, it picks the first element. FindMiddle just returns the middle value of the
        // three f64's given to it.
    } else {

        let mut root_node = Node {
            split_dimension: Dimension::Null,
            split_value: 0.0,
            left: None,
            right: None,
            points: None,
        };
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
        // i should split the vec here, and pass that in instead.
        let upper_vec = pts.split_off(split_index);
        pts.shrink_to_fit();
/*
        println!("points going to right-hand side(length: {}): \n",
                 upper_vec.len());
        for i in 0..upper_vec.len() {
            println!("x: {}, y: {}, z: {}\n",
                     upper_vec[i].x,
                     upper_vec[i].y,
                     upper_vec[i].z);
        }
        println!("points going to left-hand side(length: {}): \n", pts.len());
        for i in 0..pts.len() {
            println!("x: {}, y: {}, z: {}\n", pts[i].x, pts[i].y, pts[i].z);
        }
*/
        root_node.left = Some(Box::new(new_root_node(pts, max_pts)));
        root_node.right = Some(Box::new(new_root_node(upper_vec, max_pts)));
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
                z: z as f64
                };
                vec_that_wants_to_be_a_kdtree.push(particle);
            }
        }
   }
   let kdtree_test = new_kdtree(vec_that_wants_to_be_a_kdtree, 3);
   assert!(kdtree_test.number_of_particles == 100000);
   assert!(kdtree_test.max_points == 3);
   go_to_left(kdtree_test);
}

fn go_to_left(kdtree: KDTree) {
    let mut count_of_nodes = 0;
    let mut node = kdtree.root.left.expect("null root node at 301\n");
    while node.left.is_some() {
        count_of_nodes = count_of_nodes + 1;
        node = node.left.expect("null node at 30\n");
    }
    println!("number of nodes on left: {}\n", count_of_nodes);
    assert!(count_of_nodes == 14);
}


pub struct Particle {
    vx: f64,
    vy: f64,
    vz: f64,
    x: f64,
    y: f64,
    z: f64,
}
impl Particle {
    pub fn clone(&self)->Particle {
        return Particle{vx: self.vx, vy: self.vy, vz: self.vz, x: self.x, y: self.y, z: self.z};
    }
}
pub struct Node {
    splitDim: i32, // 0 is x, 1 is y, 2 is z
    splitVal: f64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    points:  Option<Vec<Particle>>,
}

pub struct KDTree {
    root: Node,         // The root node.
    numOfParticles: i32,    // The number of particles in the tree.
    maxPoints: i32,         // The maximum number of particles in one node.
}
pub fn newRootNode(mut pts: Vec<Particle>, max_pts: i32, start:usize, end:usize) -> Node { // Start and end are probably 0 and pts.len(), respectively.
    let length_of_points = pts.len() as i32;
    let (xmax, xmin) = maxMinX(&pts);
    let (ymax, ymin) = maxMinY(&pts);
    let (zmax, zmin) = maxMinZ(&pts);
    let xdistance = (xmax - xmin).abs();
    let ydistance = (ymax - ymin).abs();
    let zdistance = (zmax - zmin).abs();
    if length_of_points <= max_pts {
        let root_node = Node {
            splitDim: 0,
            splitVal: 0.0,
            left: None,
            right: None,
            points: Some(pts),
        };
        return root_node;
    } else {
        // So the objective here is to find the median value for whatever axis has the greatest
        // disparity in distance. It is more efficient to pick three random values and pick the
        // median of those as the pivot point, so that is done if the vector has enough points.
        // Otherwise, it picks the first element. FindMiddle just returns the middle value of the
        // three f64's given to it.

        let mut root_node = Node {
            splitDim: 0,
            splitVal: 0.0,
            left: None,
            right: None,
            points: None
        };

        let mid = (start + end) / 2 as usize;
        if zdistance > ydistance && zdistance > xdistance { // "If the z distance is the greatest"
            // split on Z
            let split_value = findMedianZ(&mut pts, start, end, mid);
            root_node.splitDim = 2;
            root_node.splitVal = split_value;
        } else if ydistance > xdistance && ydistance > zdistance { // "If the x distance is the greatest"
            // split on Y
            let split_value = findMedianY(&mut pts, start, end, mid);
            root_node.splitDim = 1;
            root_node.splitVal - split_value;
        } else { // "If the y distance is the greatest"
            // split on X
            let split_value = findMedianX(&mut pts, start, end, mid);
            root_node.splitDim = 0;
            root_node.splitVal = split_value;
        }
//        i should split the vec here, and pass that in instead.
        root_node.left = Some(Box::new(newRootNode(pts, max_pts, start, mid)));
        root_node.right = Some(Box::new(newRootNode(pts, max_pts, mid, end)));
        return root_node;
    }
}


//The following three functions just return a tuple of the maximum and minimum values in the
//dimensions. Perhaps it could use a refactor, as there is a lot of copied code.
fn maxMinX(particles: &Vec<Particle>) -> (f64, f64) {
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
fn maxMinY(particles: &Vec<Particle>) -> (f64, f64) {
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

fn maxMinZ(particles: &Vec<Particle>) -> (f64, f64) {
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

fn findMiddle(first: f64, second: f64, third: f64) -> f64 {
    if second < first && second > third {
        return second;
    }
    else if first < second && first > third {
        return first;
    }
    else {
        return third;
    }
}



//The following three functions just find median points  for the x, y, or z dimension. Perhaps it could use a refactor, because there is a lot of copied code.
fn findMedianZ(pts: &mut Vec<Particle>, start: usize, end: usize, mid:usize) -> f64 {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].z < pts[start].z {
            low = low + 1;
        }
        else {
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
        return pts[start].z;
    }
    else if high < mid {
        return findMedianZ(pts, high + 1, end, mid);
    }
    else {
        return findMedianZ(pts, start, high, mid);
    }
}
fn findMedianY(pts: &mut Vec<Particle>, start: usize, end: usize, mid: usize) -> f64 {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].y < pts[start].y {
            low = low + 1;
        }
        else {
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
        return pts[start].y;
    }
    else if high < mid {
        return findMedianY(pts, high + 1, end, mid);
    }
    else {
        return findMedianY(pts, start, high, mid);
    }
}
fn findMedianX(pts: &mut Vec<Particle>, start: usize, end: usize, mid: usize) -> f64 {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].x < pts[start].x {
            low = low + 1;
        }
        else {
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
        return pts[start].x;
    }
    else if high < mid {
        return findMedianX(pts, high + 1, end, mid);
    }
    else {
        return findMedianX(pts, start, high, mid);
    }
}

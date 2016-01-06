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
pub struct Node<'a> {
    splitDim: i32, // 0 is x, 1 is y, 2 is z
    splitVal: f64,
    left: Option<Box<&'a Node<'a>>>,
    right: Option<Box<&'a Node<'a>>>,
    points: Option<Vec<Particle>>,
}

pub struct KDTree<'a> {
    root: Node<'a>,         // The root node.
    numOfParticles: i32,    // The number of particles in the tree.
    maxPoints: i32,         // The maximum number of particles in one node.
}
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
pub fn newTree<'a>(pts: Vec<Particle>, maxPts: i32) -> KDTree<'a> {
    let length_of_points = pts.len() as i32;
    let (xmax, xmin) = maxMinX(&pts);
    let (ymax, ymin) = maxMinY(&pts);
    let (zmax, zmin) = maxMinZ(&pts);
    let xdistance = (xmax - xmin).abs();
    let ydistance = (ymax - ymin).abs();
    let zdistance = (zmax - zmin).abs();
    if length_of_points <= maxPts {
        let rootNode = Node {
            splitDim: 0,
            splitVal: 0.0,
            left: None,
            right: None,
            points: Some(pts),
        };
        return KDTree {
            root: rootNode,
            numOfParticles: length_of_points,
            maxPoints: maxPts,
        };
    } else {
        // So the objective here is to find the median value for whatever axis has the greatest
        // disparity in distance. It is more efficient to pick three random values and pick the
        // median of those as the pivot point, so that is done if the vector has enough points.
        // Otherwise, it picks the first element. FindMiddle just returns the middle value of the
        // three f64's given to it.

        if zdistance > ydistance && zdistance > xdistance { // "If the z distance is the greatest"
            let mut pivot = 0.0;
            if length_of_points < 3 {
                pivot = pts[0].z;
            }
            else {
                pivot = findMiddle(pts[0].z, pts[1].z, pts[2].z);
            }
            //DEBUG print
            println!("the pivot value is {} on the z axis", pivot);
            // split on Z
        } else if xdistance > ydistance && xdistance > zdistance { // "If the x distance is the greatest"
            let mut pivot = 0.0;
            if length_of_points < 3 {
                pivot = pts[0].x;
            }
            else {
                pivot = findMiddle(pts[0].x, pts[1].x, pts[2].x);
            }
            //DEBUG print
            println!("the pivot value is {} on the x axis", pivot);
            // split on X
        } else { // "If the y distance is the greatest"
            let mut pivot = 0.0;
            if length_of_points < 3 {
                pivot = pts[0].y;
            }
            else {
                pivot = findMiddle(pts[0].y, pts[1].y, pts[2].y);
            }
            //DEBUG print
            println!("the pivot value is {} on the y axis", pivot);
            // split on Y
        }
        let rootNode = Node {
            splitDim: 0,
            splitVal: 0.0,
            left: None,
            right: None,
            points: Some(pts),
        };
        // check x dimension's distance


        return KDTree {
            root: rootNode,
            numOfParticles: length_of_points,
            maxPoints: maxPts,
        };
    }
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

fn findMedianZ(pts: &mut Vec<Particle>, start: usize, end: usize) -> f64 {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].z <= pts[start].z {
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
    if start == pts.len() / 2 {
        return pts[start].z;
    }
    else if high < pts.len() / 2 {
        return findMedianZ(pts, high + 1, end);
    }
    else {
        return findMedianZ(pts, start, high);
    }
}
fn findMedianY(pts: &mut Vec<Particle>, start: usize, end: usize) -> f64 {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].y <= pts[start].y {
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
    if start == pts.len() / 2 {
        return pts[start].y;
    }
    else if high < pts.len() / 2 {
        return findMedianY(pts, high + 1, end);
    }
    else {
        return findMedianY(pts, start, high);
    }
}
fn findMedianX(pts: &mut Vec<Particle>, start: usize, end: usize) -> f64 {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].x <= pts[start].x {
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
    if start == pts.len() / 2 {
        return pts[start].x;
    }
    else if high < pts.len() / 2 {
        return findMedianX(pts, high + 1, end);
    }
    else {
        return findMedianX(pts, start, high);
    }
}

pub struct Particle {
    vx: f64,
    vy: f64,
    vz: f64,
    x: f64,
    y: f64,
    z: f64,
}

pub struct Node<'a> {
    splitDim: i32,
    splitVal: f64,
    left: Option<Box<&'a Node<'a>>>,
    right: Option<Box<&'a Node<'a>>>,
    points: Option<Vec<Particle>>,
}

///In the KDTree struct, the root is the root node, the number of particles is the total amount of
///particles, and maxPoints is the amount of points in one space before splitting. I think.
pub struct KDTree<'a> {
    root: Node<'a>,
    numOfParticles: i32,
    maxPoints: i32,
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

extern crate rand;
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
    /// Returns the distance between the two particles
    pub fn distance(&self, other: &Particle) -> f64 {
        // sqrt((x2 - x1) + (y2 - y1) + (z2 - z1))
        // all dist variables  are squared
        let x_dist = (other.x - self.x).powf(2.0);
        let y_dist = (other.y - self.y).powf(2.0);
        let z_dist = (other.z - self.z).powf(2.0);
        let distance = f64::sqrt(x_dist + y_dist + z_dist);
        return distance;
    }
    pub fn distance_vector(&self, other: &Particle) -> (f64, f64, f64) {
        let x_dist = (other.x - self.x).powf(2.0);
        let y_dist = (other.y - self.y).powf(2.0);
        let z_dist = (other.z - self.z).powf(2.0);
        return (x_dist, y_dist, z_dist);
    }
    pub fn new() -> Particle {
        return Particle {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            radius: 0.0,
            mass: 0.0,
        } 
    }   
}
#[test]
fn test() {
    let test_particle = Particle::new();
    assert!(Particle {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        radius: 0.0,
        mass: 0.0,
    } == test_particle);
}

extern crate bigbang;
use bigbang::{collisions::soft_body, AsEntity, GravTree, Responsive, SimulationResult};

#[derive(Clone, PartialEq)]
struct Entity {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    radius: f64,
    mass: f64,
}

impl AsEntity for Entity {
    fn as_entity(&self) -> bigbang::Entity {
        bigbang::Entity {
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            radius: self.radius,
            mass: self.mass,
        }
    }
}

impl Responsive for Entity {
    fn respond(&self, simulation_result: SimulationResult<Entity>, time_step: f64) -> Self {
        let (ax, ay, _az) = simulation_result.gravitational_acceleration;
        let (x, y, z) = (self.x, self.y, self.z);
        let (mut vx, mut vy, mut vz) = (self.vx, self.vy, self.vz);
        let self_mass = if self.radius < 1. { 0.5 } else { 105. };
        // calculate the collisions
        for other in simulation_result.collisions.clone() {
            let other_mass = if other.radius < 1. { 0.5 } else { 105. };
            let mass_coefficient_v1 = (self_mass - other_mass) / (self_mass + other_mass);
            let mass_coefficient_v2 = (2f64 * other_mass) / (self_mass + other_mass);
            vx = (mass_coefficient_v1 * vx) + (mass_coefficient_v2 * other.vx);
            vy = (mass_coefficient_v1 * vy) + (mass_coefficient_v2 * other.vy);
            vz = (mass_coefficient_v1 * vz) + (mass_coefficient_v2 * other.vz);
        }
        vx += ax * time_step;
        vy += ay * time_step;
        Entity {
            vx,
            vy,
            vz,
            x: x + (vx * time_step),
            y: y + (vy * time_step),
            z: z + (vz * time_step),
            radius: self.radius,
            mass: self.mass,
        }
    }
}

fn main() {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Entity> = Vec::new();
    for i in 0..10_000 {
        vec_that_wants_to_be_a_kdtree.push(Entity {
            x: i as f64,
            y: i as f64,
            z: i as f64,
            vx: i as f64,
            vy: i as f64,
            vz: i as f64,
            mass: 5.,
            radius: 5.,
        });
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2, 3, 0.2);

    for i in 0..20 {
        println!("time step: {}", i);
        test_tree = test_tree.time_step();
    }
}

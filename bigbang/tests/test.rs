extern crate bigbang;
use bigbang::{AsEntity, Entity, GravTree, Responsive, SimulationResult};

#[derive(Clone, PartialEq)]
struct MyEntity {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    radius: f64,
}

impl AsEntity for MyEntity {
    fn as_entity(&self) -> Entity {
        return Entity {
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            radius: self.radius,
            mass: if self.radius < 1. { 0.5 } else { 105. },
        };
    }
}

impl Responsive for MyEntity {
    fn respond(&self, simulation_result: SimulationResult<MyEntity>, time_step: f64) -> Self {
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
        MyEntity {
            vx,
            vy,
            vz,
            x: x + (vx * time_step),
            y: y + (vy * time_step),
            z: z + (vz * time_step),
            radius: self.radius,
        }
    }
}

impl MyEntity {
    pub fn random_entity() -> MyEntity {
        MyEntity {
            vx: 0f64,
            vy: 0f64,
            vz: 0f64,
            x: rand::random::<f64>() * 50f64,
            y: rand::random::<f64>() * 50f64,
            z: rand::random::<f64>() * 50f64,
            radius: rand::random::<f64>() / 10f64,
        }
    }
}

#[test]
fn test_traversal() {
    let mut vec: Vec<MyEntity> = Vec::new();
    for _ in 0..100 {
        let entity = MyEntity::random_entity();
        vec.push(entity);
    }
    let vec_clone = vec.clone();
    let tree = GravTree::new(&vec, 0.2, 3, 0.2);
    let traversed_vec = tree.as_vec();
    let mut all_found = true;
    for i in vec_clone {
        if !traversed_vec.contains(&i) {
            all_found = false;
        }
    }

    assert!(all_found);
}

#[test]
fn test_time_step() {
    let mut vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = Vec::new();
    for _ in 0..1000 {
        let entity = MyEntity::random_entity();
        vec_that_wants_to_be_a_kdtree.push(entity);
    }

    let test_tree = GravTree::new(&vec_that_wants_to_be_a_kdtree, 0.2, 3, 0.2);
    let after_time_step = test_tree.time_step();
    assert_eq!(after_time_step.as_vec().len(), 1000);
}

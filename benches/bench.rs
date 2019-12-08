#![feature(test)]
extern crate test;
use bigbang::{AsEntity, Entity, GravTree, SimulationResult};

#[derive(Clone)]
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

    fn respond(&self, simulation_result: SimulationResult<MyEntity>, time_step: f64) -> Self {
        let (ax, ay, az) = simulation_result.gravitational_acceleration;
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

#[bench]
fn bench_time_step_0125(b: &mut test::Bencher) {
    let mut vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = Vec::new();
    for _ in 0..125 {
        let entity = MyEntity::random_entity();
        vec_that_wants_to_be_a_kdtree.push(entity);
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    b.iter(|| test_tree = test_tree.time_step())
}

#[bench]
fn bench_time_step_1000(b: &mut test::Bencher) {
    let mut vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = Vec::new();
    for _ in 0..1000 {
        let entity = MyEntity::random_entity();
        vec_that_wants_to_be_a_kdtree.push(entity);
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    b.iter(|| test_tree = test_tree.time_step())
}

#[bench]
fn bench_time_step_2000(b: &mut test::Bencher) {
    let mut vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = Vec::new();
    for _ in 0..2000 {
        let entity = MyEntity::random_entity();
        vec_that_wants_to_be_a_kdtree.push(entity);
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    b.iter(|| test_tree = test_tree.time_step())
}
#[bench]
fn bench_time_step_3500(b: &mut test::Bencher) {
    let mut vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = Vec::new();
    for _ in 0..3500 {
        let entity = MyEntity::random_entity();
        vec_that_wants_to_be_a_kdtree.push(entity);
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    b.iter(|| test_tree = test_tree.time_step())
}

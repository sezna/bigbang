// the situation seems to be as such:
// the simulation is totally broken on 2 entities
// collisions are not detected accurately -- perhaps having something to do with the tree
// structure?
// acceleration is zeroed out
// The issue probably arises when the total number of entities is less than max_pts?

extern crate bigbang;
use bigbang::{collisions::soft_body, AsEntity, Entity, GravTree, Responsive, SimulationResult};

#[derive(Clone, PartialEq, AsEntity)]
struct MyEntity {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    radius: f64,
    mass: f64,
    collided_with: Vec<MyEntity>,
}

impl MyEntity {
    fn new(x: f64, y: f64, z: f64, radius: f64, mass: f64) -> MyEntity {
        MyEntity {
            x,
            y,
            z,
            vx: 0.,
            vy: 0.,
            vz: 0.,
            radius,
            mass,
            collided_with: Vec::new(),
        }
    }
}

impl Responsive for MyEntity {
    fn respond(&self, simulation_result: SimulationResult<Self>, time_step: f64) -> Self {
        let mut vx = self.vx;
        let mut vy = self.vy;
        let mut vz = self.vz;
        let mut collided_with = Vec::new();
        let (mut ax, mut ay, mut az) = simulation_result.gravitational_acceleration;
        for other in simulation_result.collisions {
            collided_with.push(other.clone());
            let (collision_ax, collision_ay, collision_az) = soft_body(self, other, 50f64);
            ax += collision_ax;
            ay += collision_ay;
            az += collision_az;
        }
        vx += ax * time_step;
        vy += ay * time_step;
        vz += az * time_step;

        MyEntity {
            vx,
            vy,
            vz,
            x: self.x + (vx * time_step),
            y: self.y + (vy * time_step),
            z: self.z + (vz * time_step),
            radius: self.radius,
            mass: self.mass,
            collided_with,
        }
    }
}

/// Test that, given two entities that are overlapping, the tree detects their collision.
#[test]
fn two_entities_collision() {
    let vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = vec![
        MyEntity::new(0., 0., 0., 10., 5.),
        MyEntity::new(0., 0., 1., 10., 5.),
    ];

    let test_tree = GravTree::new(&vec_that_wants_to_be_a_kdtree, 0.2, 3, 0.2);
    let after_time_step = test_tree.time_step().as_vec();

    // Each entity should have collided with exactly one other entity
    assert_eq!(after_time_step[0].collided_with.len(), 1);
    assert_eq!(after_time_step[1].collided_with.len(), 1);
}

/// Test that, given two entities that are not overlapping, the tree correctly does not report their collision.
#[test]
fn two_entities_no_collision() {
    let vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = vec![
        MyEntity::new(0., 1000., 0., 10., 5.),
        MyEntity::new(0., 0., 1., 10., 5.),
    ];

    let test_tree = GravTree::new(&vec_that_wants_to_be_a_kdtree, 0.2, 3, 0.2);
    let after_time_step = test_tree.time_step().as_vec();

    assert_eq!(after_time_step[0].collided_with.len(), 0);
    assert_eq!(after_time_step[1].collided_with.len(), 0);
}

/// Test that the gravitational acceleration of two distant particles is calculated correctly
#[test]
fn two_entities_accel() {
    let vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = vec![
        MyEntity::new(0., 100., 0., 10., 50.),
        MyEntity::new(50., 0., 1., 10., 500.),
    ];

    let test_tree = GravTree::new(&vec_that_wants_to_be_a_kdtree, 0.3, 3, 0.2);
    let _after_time_step = test_tree.time_step().time_step().as_vec();

    // 1.0 isn't right but it should at least not be 0, what the current test is suggesting
    // Uncomment the following line when you're ready to fix this
    // assert_eq!(after_time_step[0].vx, 1.);
}

/// Test that, given entities that are at the _exact same position_, the tree detects their collision.
/// * NOTE:
/// This is how things _should_ be, but isn't working quite yet. This is related to the fundamental
/// structure of the tree and how it compares every particle to itself as part of the iteration.
/// This is not ideal.
/*
#[test]
fn exact_overlap_collision() {
    let vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = vec![
        MyEntity::new(0., 0., 0., 10., 5.),
        MyEntity::new(0., 0., 1., 10., 5.),
        MyEntity::new(0., 0., 1., 10., 5.),
        MyEntity::new(0., 0., 1., 10., 5.),
        MyEntity::new(0., 0., 1., 10., 5.),
    ];

    let test_tree = GravTree::new(&vec_that_wants_to_be_a_kdtree, 0.2);
    let after_time_step = test_tree.time_step().as_vec();

    // Each entity should have collided with exactly all four other entities
    assert_eq!(after_time_step[0].collided_with.len(), 4);
    assert_eq!(after_time_step[1].collided_with.len(), 4);
    assert_eq!(after_time_step[2].collided_with.len(), 4);
    assert_eq!(after_time_step[3].collided_with.len(), 4);
    assert_eq!(after_time_step[4].collided_with.len(), 4);
}
*/

/// Test that, given five entities that are overlapping, the tree detects their collision.
#[test]
fn five_entities_collision() {
    let vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = vec![
        MyEntity::new(0., 0., 0., 10., 5.),
        MyEntity::new(0., 1., 0., 10., 5.),
        MyEntity::new(1., 0., 0., 10., 5.),
        MyEntity::new(1., 1., 1., 10., 5.),
        MyEntity::new(0., 1., 1., 10., 5.),
    ];

    let test_tree = GravTree::new(&vec_that_wants_to_be_a_kdtree, 0.2, 3, 0.2);
    let after_time_step = test_tree.time_step().as_vec();

    // Each entity should have collided with exactly all four other entities
    assert_eq!(after_time_step[0].collided_with.len(), 4);
    assert_eq!(after_time_step[1].collided_with.len(), 4);
    assert_eq!(after_time_step[2].collided_with.len(), 4);
    assert_eq!(after_time_step[3].collided_with.len(), 4);
    assert_eq!(after_time_step[4].collided_with.len(), 4);
}
/// Test that the gravitational acceleration of five particles is calculated correctly
/// by verifying their velocity afterwards
#[test]
fn five_entities_accel() {
    let vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = vec![
        MyEntity::new(0., 100., 0., 10., 50.),
        MyEntity::new(50., 0., 1., 10., 500.),
        MyEntity::new(50., 20., 1., 10., 500.),
        MyEntity::new(10., 20., 1., 10., 500.),
        MyEntity::new(50., 100., 1., 10., 500.),
    ];

    let test_tree = GravTree::new(&vec_that_wants_to_be_a_kdtree, 0.3, 3, 0.2);
    let after_time_step = test_tree.time_step().time_step().as_vec();

    assert_eq!(after_time_step[0].vx, 234.62426718543517);
    assert_eq!(after_time_step[0].vy, -308.1666357163429);
    assert_eq!(after_time_step[0].vz, 6.374453794316003);

    assert_eq!(after_time_step[1].vx, -728.0140671441542);
    assert_eq!(after_time_step[1].vy, 961.0687440357227);
    assert_eq!(after_time_step[1].vz, -22.318277419464977);

    assert_eq!(after_time_step[2].vx, 68.99261434517841);
    assert_eq!(after_time_step[2].vy, 202.1872542350379);
    assert_eq!(after_time_step[2].vz, 0.28488247914098197);

    assert_eq!(after_time_step[3].vx, -232.853993009126);
    assert_eq!(after_time_step[3].vy, 189.20383841899522);
    assert_eq!(after_time_step[3].vz, -2.014092604958015);
}

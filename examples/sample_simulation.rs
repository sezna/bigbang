extern crate bigbang;
use bigbang::Entity;
use bigbang::GravTree;

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

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);

    for i in 0..20 {
        println!("time step: {}", i);
        test_tree = test_tree.time_step();
    }
}

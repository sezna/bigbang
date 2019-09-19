extern crate bigbang;
use bigbang::Entity;
use bigbang::GravTree;

fn main() {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Entity> = Vec::new();
    for _ in 0..10_000 {
        let entity = Entity::random_entity();
        vec_that_wants_to_be_a_kdtree.push(entity);
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);

    for i in 0..20 {
        println!("time step: {}", i);
        test_tree = test_tree.time_step();
    }
}

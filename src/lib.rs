#![feature(test)]
extern crate either;
extern crate rand;
extern crate rayon;
extern crate test;
mod dimension;
mod entity;
mod gravtree;
mod node;
mod utilities;
use dimension::Dimension;
use node::Node;
use std::ffi::CStr;
use std::mem::transmute_copy;
mod collision_result;

/*  public-facing entry points */
pub use collision_result::CollisionResult;
pub use entity::{AsEntity, Entity};
pub use gravtree::GravTree;

/* FFI interface functions are all plopped right here. */

use std::os::raw::{c_char, c_double, c_int, c_uchar, c_void};
use std::slice;

/// The exported FFI version of [[GravTree]]'s `new()` method. Returns a void pointer to the location
/// in memory where the [[GravTree]] is located. Use this void pointer to tell Rust where to look for
/// the tree in the other FFI functions.
#[no_mangle]
pub unsafe extern "C" fn new(
    array: *const Entity,
    length: c_int,
    time_step: c_double,
) -> *mut c_void {
    assert!(!array.is_null(), "Null pointer in new()");
    let array: &[Entity] = slice::from_raw_parts(array, length as usize);
    let mut rust_vec_of_entities = Vec::from(array);
    let gravtree = GravTree::new(&mut rust_vec_of_entities, time_step as f64);
    Box::into_raw(Box::new(gravtree)) as *mut c_void
}

/// The exported FFI version of [[GravTree]]'s `time_step()` method. Instead of being a method, it is a
/// function which takes in a [[GravTree]] (rather, a void pointer to the space where the [[GravTree]] is).
#[no_mangle]
pub unsafe extern "C" fn time_step(gravtree_buf: *mut c_void) -> *mut c_void {
    let gravtree: Box<GravTree<Entity>> = Box::from_raw(gravtree_buf as *mut GravTree<Entity>);
    // A seg fault happens in the below line.
    Box::into_raw(Box::new(gravtree.time_step())) as *mut c_void
}

/// Loads a [[GravTree]] from a data file. The data file does not encode the `time_step` value, so that
/// must be provided.
#[no_mangle]
pub unsafe extern "C" fn from_data_file(
    file_path_buff: *const c_char,
    time_step: c_double,
) -> *mut c_void {
    let file_path = CStr::from_ptr(file_path_buff);

    let gravtree = GravTree::<Entity>::from_data_file(
        String::from(file_path.to_str().unwrap()),
        time_step as f64,
    )
    .unwrap();
    Box::into_raw(Box::new(gravtree)) as *mut c_void
}

/// Writes a [[GravTree]] out to a data file.
#[no_mangle]
pub unsafe extern "C" fn write_data_file(
    file_path_buff: *const c_char,
    gravtree_buf: *mut c_uchar,
) {
    let gravtree: GravTree<Entity> = transmute_copy(&gravtree_buf);
    let file_path = CStr::from_ptr(file_path_buff);
    gravtree.write_data_file(String::from(file_path.to_str().unwrap()));
}

#[test]
fn test_traversal() {
    let mut vec: Vec<Entity> = Vec::new();
    for _ in 0..100 {
        let entity = Entity::random_entity();
        vec.push(entity);
    }
    let vec_clone = vec.clone();
    let tree = GravTree::new(&mut vec, 0.2);
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
    let mut vec_that_wants_to_be_a_kdtree: Vec<Entity> = Vec::new();
    for _ in 0..10 {
        for _ in 0..10 {
            for _ in 0..10 {
                let entity = Entity::random_entity();
                vec_that_wants_to_be_a_kdtree.push(entity);
            }
        }
    }

    let test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    let _ = test_tree.time_step();
}

#[test]
#[allow(dead_code)]
fn test_tree() {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Entity> = Vec::new();
    for _ in 0..100 {
        for _ in 0..100 {
            for _ in 0..10 {
                let entity = Entity::random_entity();
                vec_that_wants_to_be_a_kdtree.push(entity);
            }
        }
    }
    let kdtree_test = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    assert!(kdtree_test.get_number_of_entities() == 100_000);
    // kdtree_test.display_tree();
    go_to_edges(kdtree_test, 14usize, 15usize);
    let mut smaller_vec: Vec<Entity> = Vec::new();
    for _ in 0..50 {
        let entity = Entity::random_entity();
        smaller_vec.push(entity);
    }
    // Testing center of mass assignment
    let mut vector = vec![
        Entity {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            x: 1.0,
            y: 2.0,
            z: 3.0,
            mass: 2.0,
            radius: 1.0,
        },
        Entity {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            x: 2.0,
            y: 1.0,
            z: 3.0,
            mass: 2.0,
            radius: 1.0,
        },
    ];
    let center_of_mass_test = GravTree::new(&mut vector, 0.2);
    assert!(center_of_mass_test.root.center_of_mass == (1.5, 1.5, 3.0));
}
#[allow(dead_code)]
/// This function is used for testing. It checks the number of nodes on each side, along the "edge" of the tree.
/// left_nodes is how many nodes you expect to see along the left size, and right_nodes is how many you expect to
///  see along the right.
fn go_to_edges(grav_tree: GravTree<Entity>, left_nodes: usize, right_nodes: usize) {
    let mut count_of_nodes = 0;
    let mut node = grav_tree.root.left.expect("null root node\n");
    let mut node2 = node.clone();
    while node.left.is_some() {
        count_of_nodes += 1;
        node = node.left.expect("unexpected null node #1\n");
    }
    assert!(count_of_nodes == left_nodes);
    count_of_nodes = 0;
    while node2.right.is_some() {
        count_of_nodes += 1;
        node2 = node2.right.expect("unexpected null node #2\n");
    }
    assert!(count_of_nodes == right_nodes);
}

#[bench]
fn bench_time_step_05(b: &mut test::Bencher) {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Entity> = Vec::new();
    for _ in 0..5 {
        for _ in 0..5 {
            for _ in 0..5 {
                let entity = Entity::random_entity();
                vec_that_wants_to_be_a_kdtree.push(entity);
            }
        }
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    b.iter(|| test_tree = test_tree.time_step())
}

#[bench]
fn bench_time_step_10(b: &mut test::Bencher) {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Entity> = Vec::new();
    for _ in 0..10 {
        for _ in 0..10 {
            for _ in 0..10 {
                let entity = Entity::random_entity();
                vec_that_wants_to_be_a_kdtree.push(entity);
            }
        }
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    b.iter(|| test_tree = test_tree.time_step())
}

#[bench]
fn bench_time_step_12(b: &mut test::Bencher) {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Entity> = Vec::new();
    for _ in 0..12 {
        for _ in 0..12 {
            for _ in 0..12 {
                let entity = Entity::random_entity();
                vec_that_wants_to_be_a_kdtree.push(entity);
            }
        }
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    b.iter(|| test_tree = test_tree.time_step())
}
#[bench]
fn bench_time_step_15(b: &mut test::Bencher) {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Entity> = Vec::new();
    for _ in 0..15 {
        for _ in 0..15 {
            for _ in 0..15 {
                let entity = Entity::random_entity();
                vec_that_wants_to_be_a_kdtree.push(entity);
            }
        }
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);
    b.iter(|| test_tree = test_tree.time_step())
}

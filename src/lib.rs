//! For more details on usage, see [the README](https://github.com/sezna/blob/master/README.md).

extern crate either;
extern crate rayon;
mod as_entity;
pub mod collisions;
mod dimension;
mod entity;
mod grav_tree;
mod node;
mod simulation_result;
mod utilities;

use dimension::Dimension;
use node::Node;
/*  public-facing entry points */
pub use as_entity::AsEntity;
pub use entity::Entity;
pub use grav_tree::GravTree;
pub use simulation_result::SimulationResult;

/* FFI interface functions are all plopped right here. */

use std::os::raw::{c_double, c_int, c_void};
use std::slice;

/// The exported FFI version of [[GravTree]]'s `new()` method. Returns a void pointer to the location
/// in memory where the [[GravTree]] is located. Use this void pointer to tell Rust where to look for
/// the tree in the other FFI functions.
#[no_mangle]
pub unsafe extern "C" fn new(
    array: *const Entity,
    length: c_int,
    time_step: c_double,
    max_entities: c_int,
    theta: c_double,
) -> *mut c_void {
    assert!(!array.is_null(), "Null pointer in new()");
    let array: &[Entity] = slice::from_raw_parts(array, length as usize);
    let mut rust_vec_of_entities = Vec::from(array);
    let grav_tree = GravTree::new(
        &mut rust_vec_of_entities,
        time_step as f64,
        max_entities as i32,
        theta as f64,
    );
    Box::into_raw(Box::new(grav_tree)) as *mut c_void
}

/// The exported FFI version of [[GravTree]]'s `time_step()` method. Instead of being a method, it is a
/// function which takes in a [[GravTree]] (rather, a void pointer to the space where the [[GravTree]] is).
#[no_mangle]
pub unsafe extern "C" fn time_step(grav_tree_buf: *mut c_void) -> *mut c_void {
    let grav_tree: Box<GravTree<Entity>> = Box::from_raw(grav_tree_buf as *mut GravTree<Entity>);
    // A seg fault happens in the below line.
    Box::into_raw(Box::new(grav_tree.time_step())) as *mut c_void
}

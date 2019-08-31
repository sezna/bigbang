#![feature(test)]
#![cfg(test)]

extern crate test;
use super::io::{open_data_file, write_data_file};
use super::particle::Particle;
use super::{new_kdtree, traverse_tree, KDTree};
// TODO list
// test a tree after gravity has been applied to make sure it is done correctly

#[test]
fn test_traversal() {
    let mut vec: Vec<Particle> = Vec::new();
    for x in 0..100 {
        let particle = Particle::random_particle();
        vec.push(particle);
    }
    let vec_clone = vec.clone();
    let tree = new_kdtree(&mut vec);
    let traversed_vec = traverse_tree(&tree);
    let mut all_found = true;
    for i in vec_clone {
        if !traversed_vec.contains(&i) {
            println!("particle not found");
            all_found = false;
        }
    }

    assert!(all_found);
}

#[test]
/// Really lame test for Particle::new()
fn test() {
    let test_particle = Particle::new();
    assert!(
        Particle {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            radius: 0.0,
            mass: 0.0,
        } == test_particle
    );
}
#[test]
#[allow(dead_code)]
fn test_tree() {
    let mut vec_that_wants_to_be_a_kdtree: Vec<Particle> = Vec::new();
    for x in 0..100 {
        for y in 0..100 {
            for z in 0..10 {
                let particle = Particle::random_particle();
                vec_that_wants_to_be_a_kdtree.push(particle);
            }
        }
    }
    let kdtree_test = new_kdtree(&mut vec_that_wants_to_be_a_kdtree);
    assert!(kdtree_test.number_of_particles == 100000);
    // kdtree_test.display_tree();
    println!("testing integrity of the big tree\n");
    go_to_edges(kdtree_test);
    let mut smaller_vec: Vec<Particle> = Vec::new();
    println!("displaying a smaller tree\n");
    for z in 0..50 {
        let particle = Particle::random_particle();
        smaller_vec.push(particle);
    }
    let smaller_kdtree = new_kdtree(&mut smaller_vec);
    smaller_kdtree.display_tree();
    // Testing center of mass assignment
    let mut vector = vec![
        Particle {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            x: 1.0,
            y: 2.0,
            z: 3.0,
            mass: 2.0,
            radius: 1.0,
        },
        Particle {
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
    let center_of_mass_test = new_kdtree(&mut vector);
    assert!(center_of_mass_test.root.center_of_mass == (1.5, 1.5, 3.0));
}
#[allow(dead_code)]
fn go_to_edges(kdtree: KDTree) {
    let mut count_of_nodes = 0;
    let mut node = kdtree.root.left.expect("null root node\n");
    let mut node2 = node.clone();
    while node.left.is_some() {
        count_of_nodes = count_of_nodes + 1;
        node = node.left.expect("unexpected null node #1\n");
    }
    println!("number of nodes on left: {}\n", count_of_nodes);
    assert!(count_of_nodes == 14);
    count_of_nodes = 0;
    while node2.right.is_some() {
        count_of_nodes = count_of_nodes + 1;
        node2 = node2.right.expect("unexpected null node #2\n");
    }
    println!("number of nodes on right: {}\n", count_of_nodes);
    assert!(count_of_nodes == 15);
}
#[test]
fn test_input() {
    let test_vec = open_data_file("test_files/test_input.txt".to_string());
    println!("test_vec len: {}", test_vec.len());
    assert!(test_vec.len() == 3601);
}
#[test]
fn test_output() {
    let mut test_vec: Vec<Particle> = Vec::new();
    for i in 0..1000 {
        test_vec.push(Particle::random_particle());
    }
    let kd = new_kdtree(&mut test_vec);
    write_data_file(kd, "test_files/test_output.txt".to_string());
    let read_file = open_data_file("test_files/test_output.txt".to_string());
    println!(
        "test_vec.len() = {} read_file.len() = {}",
        test_vec.len(),
        read_file.len()
    );
    assert!(test_vec.len() == read_file.len());
}

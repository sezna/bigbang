#![feature(test)]

extern crate bigbang;
extern crate test;

use bigbang::{Entity, AsEntity, GravTree, max_min_xyz};

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

#[bench]
fn bench_max_min(b: &mut test::Bencher) {
	let mut test_vec: Vec<Entity> = Vec::new();
	for _ in 0..1000 {
		test_vec.push(Entity::random_entity());
	}

	let ref_vec = test_vec
		.iter()
		.map(|x| x.as_entity())
		.collect::<Vec<Entity>>();
	// TODO make it do this with different vecs
	b.iter(|| max_min_xyz(ref_vec.as_slice()));
}

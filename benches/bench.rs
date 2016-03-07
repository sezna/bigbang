#![feature(test)]
extern crate test;
extern crate kdtree;
use kdtree::kdtree::{new_kdtree, KDTree};
use kdtree::kdtree::particle::Particle;
use self::test::Bencher;
#[bench]
fn bench_tree(b: &mut Bencher) {
    b.iter(|| new_kdtree(&mut vec![Particle::random_particle(); 1000]));
}

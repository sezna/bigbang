extern crate test;
extern crate kdtree;
use kdtree::{new_kdtree, KDTree};
use kdtree::particle::Particle;
use self::test::Bencher;
#![feature(test)]
#[bench]
fn bench_tree(b: &mut Bencher) {
    b.iter(|| new_kdtree(&mut vec![Particle::random_particle(); 10000]));
}

extern crate test;
use self::test::Bencher;
use crate::particle::Particle;
use crate::{new_kdtree, KDTree};
#[bench]
fn bench_tree(b: &mut Bencher) {
    b.iter(|| new_kdtree(&mut vec![Particle::random_particle(); 1000]));
}

#[bench]
/// Bench the function which gets the minimum and maximum values for z out of an array slice of particles.
/// This is iterated heavily so the performance of this function impacts the performance of the structure as a whole.
fn bench_min_max(b: &mut Bencher) {
    let mut test_vec: Vec<Particle> = Vec::new();
    for _ in 0..1000 {
        test_vec.push(Particle::random_particle());
    }
    let kd = new_kdtree(&mut test_vec);
}

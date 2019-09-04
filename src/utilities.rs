use super::Dimension;
use crate::entity::Entity;
use std::cmp::Ordering;
extern crate test;
/// Returns the absolute distance in every dimension (the range in every dimension)
/// of an array slice of entities.
pub fn xyz_distances(entities: &[Entity]) -> (f64, f64, f64) {
    let (x_max, x_min, y_max, y_min, z_max, z_min) = max_min_xyz(entities);
    let x_distance = x_max - x_min;
    let y_distance = y_max - y_min;
    let z_distance = z_max - z_min;
    return (x_distance.abs(), y_distance.abs(), z_distance.abs());
}

/// Given an array slice of entities, returns the maximum and minimum x, y, and z values as
/// a septuple.
pub fn max_min_xyz(entities: &[Entity]) -> (&f64, &f64, &f64, &f64, &f64, &f64) {
    let (x_max, x_min) = max_min(Dimension::X, entities);
    let (y_max, y_min) = max_min(Dimension::Y, entities);
    let (z_max, z_min) = max_min(Dimension::Z, entities);
    return (x_max, x_min, y_max, y_min, z_max, z_min);
}

#[bench]
fn bench_max_min(b: &mut test::Bencher) {
    let mut test_vec: Vec<Entity> = Vec::new();
    for _ in 0..1000 {
        test_vec.push(Entity::random_entity());
    }
    // TODO make it do this with different vecs
    b.iter(|| max_min_xyz(&test_vec));
}

/// Returns the maximum and minimum z values in a slice of entities.
pub fn max_min(dim: Dimension, entities: &[Entity]) -> (&f64, &f64) {
    (
        &entities
            .iter()
            .max_by(|a, b| {
                a.get_dim(&dim)
                    .partial_cmp(b.get_dim(&dim))
                    .unwrap_or_else(|| Ordering::Equal)
            })
            .expect(&format!("no max {} found", dim.as_string()))
            .z,
        &entities
            .iter()
            .min_by(|a, b| {
                a.get_dim(&dim)
                    .partial_cmp(b.get_dim(&dim))
                    .unwrap_or_else(|| Ordering::Equal)
            })
            .expect(&format!("no min {} found", dim.as_string()))
            .z,
    )
}

/// Finds the median value for a given dimension in a slice of entities.
/// Making one that clones/uses immutability could be an interesting performance benchmark.
pub fn find_median(dim: Dimension, pts: &mut [Entity]) -> (&f64, usize) {
    find_median_helper(dim, pts, 0, pts.len(), pts.len() / 2usize)
}

fn find_median_helper(
    dim: Dimension,
    pts: &mut [Entity],
    start: usize,
    end: usize,
    mid: usize,
) -> (&f64, usize) {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].get_dim(&dim) < pts[start].get_dim(&dim) {
            low = low + 1;
        } else {
            pts.swap(low, high);
            high -= 1;
        }
    }
    pts.swap(start, high);
    if start == mid {
        return (pts[start].get_dim(&dim), start);
    } else if high < mid {
        return find_median_helper(dim, pts, high + 1, end, mid);
    } else {
        return find_median_helper(dim, pts, start, high, mid);
    }
}

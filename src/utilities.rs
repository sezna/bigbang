use super::Dimension;
#[allow(unused_imports)]
use crate::entity::{AsEntity, Entity};
use std::cmp::Ordering;
/// Returns the absolute distance in every dimension (the range in every dimension)
/// of an array slice of entities.
pub fn xyz_distances(entities: &[Entity]) -> (f64, f64, f64) {
    let (x_max, x_min, y_max, y_min, z_max, z_min) = max_min_xyz(entities);
    let x_distance = x_max - x_min;
    let y_distance = y_max - y_min;
    let z_distance = z_max - z_min;
    (x_distance.abs(), y_distance.abs(), z_distance.abs())
}

/// Given an array slice of entities, returns the maximum and minimum x, y, and z values as
/// a septuple.
pub fn max_min_xyz(entities: &[Entity]) -> (&f64, &f64, &f64, &f64, &f64, &f64) {
    let (x_max, x_min) = max_min(Dimension::X, entities);
    let (y_max, y_min) = max_min(Dimension::Y, entities);
    let (z_max, z_min) = max_min(Dimension::Z, entities);
    (x_max, x_min, y_max, y_min, z_max, z_min)
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

#[test]
fn test_max_min() {
    let mut test_vec: Vec<Entity> = Vec::new();
    for _ in 0..1000 {
        test_vec.push(Entity::random_entity());
    }
    test_vec[100].x = std::f64::MAX;
    test_vec[101].y = std::f64::MAX;
    test_vec[2].z = std::f64::MAX;
    test_vec[888].x = std::f64::MIN;
    test_vec[541].y = std::f64::MIN;
    test_vec[111].z = std::f64::MIN;

    let (x_max, x_min, y_max, y_min, z_max, z_min) = max_min_xyz(&test_vec);
    assert_eq!(std::f64::MAX, *x_max);
    assert_eq!(std::f64::MAX, *y_max);
    assert_eq!(std::f64::MAX, *z_max);
    assert_eq!(std::f64::MIN, *x_min);
    assert_eq!(std::f64::MIN, *y_min);
    assert_eq!(std::f64::MIN, *z_min);
}

/// Returns the maximum and minimum values in a slice of entities, given a dimension.
pub fn max_min(dim: Dimension, entities: &[Entity]) -> (&f64, &f64) {
    (
        entities
            .iter()
            .max_by(|a, b| {
                a.get_dim(&dim)
                    .partial_cmp(b.get_dim(&dim))
                    .unwrap_or_else(|| Ordering::Equal)
            })
            .unwrap_or_else(|| panic!("no max {} found", dim.as_string()))
            .get_dim(&dim),
        entities
            .iter()
            .min_by(|a, b| {
                a.get_dim(&dim)
                    .partial_cmp(b.get_dim(&dim))
                    .unwrap_or_else(|| Ordering::Equal)
            })
            .unwrap_or_else(|| panic!("no min {} found", dim.as_string()))
            .get_dim(&dim),
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
            low += 1;
        } else {
            pts.swap(low, high);
            high -= 1;
        }
    }
    pts.swap(start, high);
    if start == mid {
        (pts[start].get_dim(&dim), start)
    } else if high < mid {
        find_median_helper(dim, pts, high + 1, end, mid)
    } else {
        find_median_helper(dim, pts, start, high, mid)
    }
}

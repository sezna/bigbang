//! This module contains functions to be used for collision calculations.
//! You do not need to use these, they are provided merely for convenience.
//! All of the functions follow the format of:
//!  > Given two `T: AsEntity` `p1` and `p2` and some parameters, return the acceleration vector being
//! > exerted from the collision

use crate::as_entity::AsEntity;

/// Uses [Hooke's law](https://en.wikipedia.org/wiki/Hooke%27s_law) exerting an outwards force
/// proportional to the amount of overlap when two entities are overlapping.
/// The argument `stiffness` refers to the stiffness coefficient applied to the overlapping value.
pub fn soft_body<T>(p1: &T, p2: &T, stiffness: f64) -> (f64, f64, f64)
where
    T: AsEntity,
{
    let p1 = p1.as_entity();
    let p2 = p2.as_entity();
    // calculate the overlap of the two particles
    let distance = p1.distance(&p2);
    let radii_sum = p1.radius + p2.radius;
    // if the distance is greater than the radii combined, then there actually was no collision and
    // we can return early.
    if distance >= radii_sum {
        return (p1.vx, p1.vy, p1.vz);
    }
    let overlap = radii_sum - distance;
    let force = stiffness * overlap;
    let acceleration_scalar = force / p1.mass;
    let (unit_x, unit_y, unit_z) = unit_vector(&p2.distance_vector(&p1));
    (
        unit_x * acceleration_scalar,
        unit_y * acceleration_scalar,
        unit_z * acceleration_scalar,
    )
}

/// Utility function to turn a vector into a unit vector.
fn unit_vector(vec: &(f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z) = vec;
    let length = f64::sqrt((x * x) + (y * y) + (z * z));
    (x / length, y / length, z / length)
}

use super::Dimension;
use kdtree::particle::Particle;
use std::cmp::Ordering;

// The following three functions just return a tuple of the maximum
// and minimum values in the dimensions. Perhaps it could use a
// refactor, as there is a lot of copied code.
/// Returns the maximum and minimum x values in a slice of particles.
//
// pub fn max_min(particles: &[Particle]) -> (Point3, Point3) { -- for use when
// switching to the point3 struct.
// use std::f64;
// assert!(particles.len() > 0);
// let mut max = Point3::new(f64::MIN);
// let mut min = Point3::new(f64::MAX);
// for p in particles {
// if p.x > max.x {
// max.x = p.x;
// }
// if p.y > max.y {
// max.y = p.y;
// }
// if p.z > max.z {
// max.z = p.z;
// }
//
// if p.x < min.x {
// min.x = p.x;
// }
// if p.y < min.y {
// min.y = p.y;
// }
// if p.z < min.z {
// min.z = p.z;
// }
// }
//
// (max, min)
// }
//

/// Returns the absolute distance in every dimension (the range in every dimension)
/// of an array slice of particles.
pub fn xyz_distances(particles: &[Particle]) -> (f64, f64, f64) {
    let (x_max, x_min, y_max, y_min, z_max, z_min) = max_min_xyz(particles);
    let x_distance = x_max - x_min;
    let y_distance = y_max - y_min;
    let z_distance = z_max - z_min;
    return (x_distance.abs(), y_distance.abs(), z_distance.abs());
}

pub fn max_min_xyz(particles: &[Particle]) -> (&f64, &f64, &f64, &f64, &f64, &f64) {
    let (x_max, x_min) = max_min(Dimension::X, particles);
    let (y_max, y_min) = max_min(Dimension::Y, particles);
    let (z_max, z_min) = max_min(Dimension::Z, particles);
    return (x_max, x_min, y_max, y_min, z_max, z_min);
}

/// Returns the maximum and minimum z values in a slice of particles.
pub fn max_min(dim: Dimension, particles: &[Particle]) -> (&f64, &f64) {
    let cmp_func: Box<dyn Fn(&Particle, &Particle) -> Ordering> = match dim {
        Dimension::X => Box::new(|a, b| a.x.partial_cmp(&b.x).unwrap_or_else(|| Ordering::Equal)),
        Dimension::Y => Box::new(|a, b| a.y.partial_cmp(&b.y).unwrap_or_else(|| Ordering::Equal)),
        Dimension::Z => Box::new(|a, b| a.z.partial_cmp(&b.z).unwrap_or_else(|| Ordering::Equal)),
    };
    (
        &particles
            .iter()
            .max_by(|a, b| cmp_func(a, b))
            .expect(&format!("no max {} found", dim.as_string()))
            .z,
        &particles
            .iter()
            .min_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or_else(|| Ordering::Equal))
            .expect(&format!("no min {} found", dim.as_string()))
            .z,
    )
}

// The following three functions just find median points  for the x, y, or z
// dimension. Perhaps it could use a refactor, because there is a lot of copied
// code. They return a tuple of the value being split at and the index being
// split at.
/// Returns the median "z" value in a slice of particles.
pub fn find_median_z(pts: &mut [Particle], start: usize, end: usize, mid: usize) -> (f64, usize) {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].z < pts[start].z {
            low = low + 1;
        } else {
            pts.swap(low, high);
            high -= 1;
        }
    }
    pts.swap(start, high);
    if start == mid {
        return (pts[start].z, start);
    } else if high < mid {
        return find_median_z(pts, high + 1, end, mid);
    } else {
        return find_median_z(pts, start, high, mid);
    }
}
/// Returns the median "y" value in a slice of particles.
pub fn find_median_y(pts: &mut [Particle], start: usize, end: usize, mid: usize) -> (f64, usize) {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].y < pts[start].y {
            low = low + 1;
        } else {
            pts.swap(low, high);
            high -= 1;
        }
    }
    pts.swap(start, high);
    if start == mid {
        return (pts[start].y, start);
    } else if high < mid {
        return find_median_y(pts, high + 1, end, mid);
    } else {
        return find_median_y(pts, start, high, mid);
    }
}
/// Returns the median "x" value in a slice of particles.
pub fn find_median_x(pts: &mut [Particle], start: usize, end: usize, mid: usize) -> (f64, usize) {
    let mut low = (start + 1) as usize;
    let mut high = (end - 1) as usize; //exclusive end
    while low <= high {
        if pts[low].x < pts[start].x {
            low = low + 1;
        } else {
            pts.swap(low, high);
            high -= 1;
        }
    }
    pts.swap(start, high);
    if start == mid {
        return (pts[start].x, start);
    } else if high < mid {
        return find_median_x(pts, high + 1, end, mid);
    } else {
        return find_median_x(pts, start, high, mid);
    }
}

use kdtree::particle::Particle;

// The following three functions just return a tuple of the maximum
// and minimum values in the dimensions. Perhaps it could use a
// refactor, as there is a lot of copied code.
/// Returns the maximum and minimum x values in a slice of particles.
pub fn max_min_x(particles: &[Particle]) -> (f64, f64) {
    let mut to_return_max = 0.0;
    let mut to_return_min = particles[0].x;
    for i in particles {
        if i.x > to_return_max {
            to_return_max = i.x;
        }
        if i.x < to_return_min {
            to_return_min = i.x;
        }
    }
    return (to_return_max, to_return_min);
}

/// Returns the maximum and minimum y values in a slice of particles.
pub fn max_min_y(particles: &[Particle]) -> (f64, f64) {
    let mut to_return_max = 0.0;
    let mut to_return_min = particles[0].y;
    for i in particles {
        if i.y > to_return_max {
            to_return_max = i.y;
        }
        if i.y < to_return_min {
            to_return_min = i.y;
        }
    }
    return (to_return_max, to_return_min);
}

/// Returns the maximum and minimum z values in a slice of particles.
pub fn max_min_z(particles: &[Particle]) -> (f64, f64) {
    let mut to_return_max = 0.0;
    let mut to_return_min = particles[0].z;
    for i in particles {
        if i.z > to_return_max {
            to_return_max = i.z;
        }
        if i.z < to_return_min {
            to_return_min = i.z;
        }
    }
    return (to_return_max, to_return_min);
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
            let tmp = pts[low].clone();
            pts[low] = pts[high].clone();
            pts[high] = tmp;
            high -= 1;
        }
    }
    let tmp = pts[high].clone();
    pts[high] = pts[start].clone();
    pts[start] = tmp;
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
            let tmp = pts[low].clone();
            pts[low] = pts[high].clone();
            pts[high] = tmp;
            high -= 1;
        }
    }
    let tmp = pts[high].clone();
    pts[high] = pts[start].clone();
    pts[start] = tmp;
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
            let tmp = pts[low].clone();
            pts[low] = pts[high].clone();
            pts[high] = tmp;
            high -= 1;
        }
    }
    let tmp = pts[high].clone();
    pts[high] = pts[start].clone();
    pts[start] = tmp;
    if start == mid {
        return (pts[start].x, start);
    } else if high < mid {
        return find_median_x(pts, high + 1, end, mid);
    } else {
        return find_median_x(pts, start, high, mid);
    }
}


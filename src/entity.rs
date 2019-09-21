extern crate rand;
use super::Dimension;
use crate::Node;
use either::{Either, Left, Right};

/// The tolerance for the distance from an entity to the center of mass of an entity
/// If the distance is beyond this threshold, we treat the entire node as one giant
/// entity instead of recursing into it.

const THETA: f64 = 0.2;
#[derive(Clone, Default)]
/// An Entity is an object (generalized to be spherical, having only a radius dimension) which has
/// velocity, position, radius, and mass. This gravitational tree contains many entities and it moves
/// them around according to the gravity they exert on each other.
#[repr(C)]
pub struct Entity {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
    pub mass: f64,
    pub is_colliding: bool,
}

/// [[GravTree]] works with any type which implements [[AsEntity]]. In order to implement [[AsEntity]],
/// a type must be able to represent itself as a gravitational spatial entity. This, simply, entails
/// constructing an [[Entity]] from the type, and defining how to acceleration to the velocity of your type.
///
/// More generally, this entails that a type must contain, or be able to derive, its velocity, position,
/// radius and mass, and it must be able to respond to acceleration impulses in the form of triples of `f64`s.
///
/// See `impl AsEntity for Entity' for an example of what this could look like.
pub trait AsEntity {
    fn as_entity(&self) -> Entity;
    fn apply_velocity(&self, simulation_results: ((f64, f64, f64), bool), time_step: f64) -> Self;
}

impl AsEntity for Entity {
    fn as_entity(&self) -> Entity {
        return self.clone();
    }
    fn apply_velocity(&self, simulation_results: ((f64, f64, f64), bool), time_step: f64) -> Self {
        let ((vx, vy, vz), colliding) = simulation_results;
        Entity {
            vx,
            vy,
            vz,
            x: self.x + (vx * time_step),
            y: self.y + (vy * time_step),
            z: self.z + (vz * time_step),
            radius: self.radius,
            mass: self.mass,
            is_colliding: colliding,
        }
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.z == other.z
            && self.radius == other.radius
            && self.mass == other.mass
    }
}

impl Entity {
    /// Convenience function for testing.
    /// Generates an entity with random properties.
    pub fn random_entity() -> Entity {
        Entity {
            vx: rand::random::<f64>(),
            vy: rand::random::<f64>(),
            vz: rand::random::<f64>(),
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
            radius: rand::random::<f64>(),
            mass: rand::random::<f64>(),
            is_colliding: false,
        }
    }

    /// Returns a velocity vector which represents the velocity of the particle after it has interacted
    /// with the rest of the tree. Also returns a boolean representing whether or not a collision happened.
    /// TODO make this a struct???
    pub fn interact_with<T: AsEntity + Clone>(
        &self,
        node: &Node<T>,
        time_step: f64,
    ) -> ((f64, f64, f64), bool) {
        let (v, collided) = self.collide(node, None);
        // If there was a collision and we were not already colliding, use that velocity.
        let (mut vx, mut vy, mut vz) = if collided & !self.is_colliding {
            v
        // Otherwise, just use its own velocity.
        } else {
            (self.vx, self.vy, self.vz)
        };

        // Get the gravitational acceleration from the tree...
        let acceleration = self.get_entity_acceleration_from(node);
        // Apply the gravitational acceleration to the calculated velocity.
        (
            (
                vx + acceleration.0 * time_step,
                vy + acceleration.1 * time_step,
                vz + acceleration.2 * time_step,
            ),
            collided,
        )
    }

    /// Needs to be reworked to use min/max position values, but it naively checks
    /// if two things collide right now.
    fn did_collide_into(&self, other: &Entity) -> bool {
        if other.radius > 1f64 {
            return true;
        }
        self != other && self.distance(other) <= (self.radius + other.radius)
    }

    fn collide<T: AsEntity + Clone>(
        &self,
        node: &Node<T>,
        starter_velocities: Option<(f64, f64, f64)>,
    ) -> ((f64, f64, f64), bool) {
        let mut collided = false;
        let (mut vx, mut vy, mut vz) = if let Some(v) = starter_velocities {
            v
        } else {
            (self.vx, self.vy, self.vz)
        };
        // If the two entities are touching...
        if self.did_collide_into(&node.as_entity()) {
            // ...then there is the potential for a collision.
            // If this is a leaf node...
            if let Some(points) = &node.points {
                // Check every particle in the leaf to see if it collided.
                for other_T in points.iter() {
                    let other = other_T.as_entity();
                    // if they collided...
                    if self.did_collide_into(&other) {
                        // do some math.
                        let mass_coefficient_v1 =
                            (self.mass - other.mass) / (self.mass + other.mass);
                        let mass_coefficient_v2 = (2f64 * other.mass) / (self.mass + other.mass);
                        vx = (mass_coefficient_v1 * vx) + (mass_coefficient_v2 * other.vx);
                        vy = (mass_coefficient_v1 * vy) + (mass_coefficient_v2 * other.vy);
                        vz = (mass_coefficient_v1 * vz) + (mass_coefficient_v2 * other.vz);
                        collided = true;
                    }
                }
            }
            // Otherwise, this isn't a leaf, and we must...
            else {
                // Recurse!
                // on both the left...
                if let Some(left) = &node.left {
                    // If there was a collision...
                    let ((l_vx, l_vy, l_vz), l_collided) = self.collide(&left, Some((vx, vy, vz)));
                    if (l_collided) {
                        // return the new velocity.
                        vx = l_vx;
                        vy = l_vy;
                        vz = l_vz;
                        collided = true;
                    }
                }
            }
            // and the right...
            if let Some(right) = &node.right {
                let ((r_vx, r_vy, r_vz), r_collided) = self.collide(&right, Some((vx, vy, vz)));
                if (r_collided) {
                    vx = r_vx;
                    vy = r_vy;
                    vz = r_vz;
                    collided = true;
                }
            }
        }
        return ((vx, vy, vz), collided);
    }

    /// Returns the entity as a string with space separated values.
    pub fn as_string(&self) -> String {
        return format!(
            "{} {} {} {} {} {} {} {}",
            self.x, self.y, self.z, self.vx, self.vy, self.vz, self.mass, self.radius
        );
    }
    /// The returns the distance squared between two particles.
    /// Take the sqrt of this to get the distance.
    fn distance_squared(&self, other: &Entity) -> f64 {
        // (x2 - x1) + (y2 - y1) + (z2 - z1)
        // all dist variables  are squared
        // This is being called from somewhere where `other` has NaN values
        let (x_dist, y_dist, z_dist) = self.distance_vector(other);
        x_dist.abs() + y_dist.abs() + z_dist.abs()
    }
    /// Returns the distance between the two entities
    fn distance(&self, other: &Entity) -> f64 {
        // sqrt((x2 - x1) + (y2 - y1) + (z2 - z1))
        f64::sqrt(self.distance_squared(other).abs())
    }
    /// Returns the distance between two entities as an (x:f64,y:f64,z:f64) tuple.
    fn distance_vector(&self, other: &Entity) -> (f64, f64, f64) {
        let x_dist = (other.x - self.x) * (other.x - self.x) * (other.x - self.x).signum();
        let y_dist = (other.y - self.y) * (other.y - self.y) * (other.y - self.y).signum();
        let z_dist = (other.z - self.z) * (other.z - self.z) * (other.z - self.z).signum();
        (x_dist, y_dist, z_dist)
    }

    pub fn get_dim(&self, dim: &Dimension) -> &f64 {
        match *dim {
            Dimension::X => &self.x,
            Dimension::Y => &self.y,
            Dimension::Z => &self.z,
        }
    }

    /// Returns a boolean representing whether or node the node is within the theta range
    /// of the entity.
    fn theta_exceeded<T: AsEntity + Clone>(&self, node: &Node<T>) -> bool {
        // 1) distance from entity to COM of that node
        // 2) if 1) * theta > size (max diff) then
        // This frequently makes a node with NaN positions
        let node_as_entity = node.as_entity();
        let dist = self.distance_squared(&node_as_entity);
        let max_dist = node.max_distance();
        (dist) * (THETA * THETA) > (max_dist * max_dist)
    }

    /// Given two entities, self and other, returns the acceleration that other is exerting on
    /// self. Other can be either an entity or a node.
    fn get_gravitational_acceleration<T: AsEntity + Clone>(
        &self,
        oth: Either<&Entity, &Node<T>>,
    ) -> (f64, f64, f64) {
        // TODO get rid of this clone
        let other = match oth {
            Left(entity) => entity.clone(),
            Right(node) => node.as_entity(),
        };
        let d_magnitude = self.distance(&other);
        if d_magnitude == 0.0 {
            // sort of other use of THETA here
            return (0f64, 0f64, 0f64);
        }
        let d_vector = self.distance_vector(&other);
        let d_over_d_cubed = (
            d_vector.0 / d_magnitude * d_magnitude,
            d_vector.1 / d_magnitude * d_magnitude,
            d_vector.2 / d_magnitude * d_magnitude,
        );
        (
            d_over_d_cubed.0 * other.mass,
            d_over_d_cubed.1 * other.mass,
            d_over_d_cubed.2 * other.mass,
        )
    }

    /// Returns the acceleration of an entity after it has had gravity from the specified node applied to it.
    /// In this function, we approximate some entities if they exceed a certain critera specified in
    /// "exceeds_theta()". If we reach a node and it is a leaf, then we automatically get the
    /// acceleration from every entity in that node, but if we reach a node that is not a leaf and
    /// exceeds_theta() is true, then we treat the node as one giant entity and get the
    /// acceleration from it.
    pub fn get_entity_acceleration_from<T: AsEntity + Clone>(
        &self,
        node: &Node<T>,
    ) -> (f64, f64, f64) {
        let mut acceleration = (0f64, 0f64, 0f64);
        if let Some(node) = &node.left {
            if node.points.is_some() {
                // if this node has some points, calculate their gravitational acceleration
                // same logic as above
                for i in node.points.as_ref().expect("unexpected null node 2") {
                    let tmp_accel = self.get_gravitational_acceleration::<T>(Left(&i.as_entity()));
                    acceleration.0 += tmp_accel.0;
                    acceleration.1 += tmp_accel.1;
                    acceleration.2 += tmp_accel.2;
                }
            } else if self.theta_exceeded(&node) {
                // otherwise, if theta is exceeded, calculate the entire node as a big boi particle
                let tmp_accel = self.get_gravitational_acceleration(Right(&node));
                acceleration.0 += tmp_accel.0;
                acceleration.1 += tmp_accel.1;
                acceleration.2 += tmp_accel.2;
            } else {
                // otherwise, theta has not been exceeded and this is not a leaf. recurse
                let tmp_accel = self.get_entity_acceleration_from(&node);
                acceleration.0 += tmp_accel.0;
                acceleration.1 += tmp_accel.1;
                acceleration.2 += tmp_accel.2;
            }
        };
        if let Some(node) = &node.right {
            if node.points.is_some() {
                // same logic as above
                for i in node.points.as_ref().expect("unexpected null node 2") {
                    let tmp_accel = self.get_gravitational_acceleration::<T>(Left(&i.as_entity()));
                    acceleration.0 += tmp_accel.0;
                    acceleration.1 += tmp_accel.1;
                    acceleration.2 += tmp_accel.2;
                }
            } else if self.theta_exceeded(&node) {
                // TODO
                let tmp_accel = self.get_gravitational_acceleration(Right(&node));
                acceleration.0 += tmp_accel.0;
                acceleration.1 += tmp_accel.1;
                acceleration.2 += tmp_accel.2;
            } else {
                let tmp_accel = self.get_entity_acceleration_from(&node);
                acceleration.0 += tmp_accel.0;
                acceleration.1 += tmp_accel.1;
                acceleration.2 += tmp_accel.2;
            }
        };
        (
            acceleration.0 + acceleration.0,
            acceleration.1 + acceleration.1,
            acceleration.2 + acceleration.2,
        )
    }
}

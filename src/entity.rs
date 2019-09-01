extern crate rand;
use super::Dimension;
use crate::Node;
use either::{Either, Left, Right};
/// The length of time that passes each step. This coefficient is multiplied by the velocity
/// before the velocity is added to the position of the entities each step.
const TIME_STEP: f64 = 0.2;
/// The tolerance for the distance from an entity to the center of mass of an entity
/// If the distance is beyond this threshold, we treat the entire node as one giant
/// entity instead of recursing into it.
const THETA: f64 = 0.2;
#[derive(Clone, PartialEq, Default)]

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
}
impl Entity {
    /// Returns an entity with all 0.0 values.
    pub fn new() -> Entity {
        Entity::default()
    }
    /// Convenience function for testing.
    /// Generates an entity with random properties.
    pub fn random_entity() -> Entity {
        return Entity {
            vx: rand::random::<f64>(),
            vy: rand::random::<f64>(),
            vz: rand::random::<f64>(),
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
            radius: rand::random::<f64>(),
            mass: rand::random::<f64>(),
        };
    }

    /// Returns a new entity after gravity from a node has been applied to it.
    /// Should be read as "apply gravity from node"
    pub fn apply_gravity_from(&self, node: &Node) -> Entity {
        let acceleration = self.get_entity_acceleration_from(node);
        let (vx, vy, vz) = (
            self.vx + acceleration.0 * TIME_STEP,
            self.vy + acceleration.1 * TIME_STEP,
            self.vz + acceleration.2 * TIME_STEP,
        );
        return Entity {
            vx: vx,
            vy: vy,
            vz: vz,
            x: self.x + (vx * TIME_STEP),
            y: self.y + (vy * TIME_STEP),
            z: self.z + (vz * TIME_STEP),
            radius: self.radius,
            mass: self.mass,
        };
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
        let x_dist = (other.x - self.x) * (other.x - self.x);
        let y_dist = (other.y - self.y) * (other.y - self.y);
        let z_dist = (other.z - self.z) * (other.z - self.z);
        let distance = x_dist + y_dist + z_dist;
        return distance;
    }
    /// Returns the distance between the two entities
    fn distance(&self, other: &Entity) -> f64 {
        // sqrt((x2 - x1) + (y2 - y1) + (z2 - z1))
        f64::sqrt(self.distance_squared(other))
    }
    /// Returns the distance between two entities as an (x:f64,y:f64,z:f64) tuple.
    fn distance_vector(&self, other: &Entity) -> (f64, f64, f64) {
        let x_dist = (other.x - self.x) * (other.x - self.x);
        let y_dist = (other.y - self.y) * (other.y - self.y);
        let z_dist = (other.z - self.z) * (other.z - self.z);
        return (x_dist, y_dist, z_dist);
    }

    pub fn get_dim(&self, dim: &Dimension) -> &f64 {
        match dim {
            &Dimension::X => &self.x,
            &Dimension::Y => &self.y,
            &Dimension::Z => &self.z,
        }
    }

    /// Returns a boolean representing whether or node the node is within the theta range
    /// of the entity.
    fn theta_exceeded(&self, node: &Node) -> bool {
        // 1) distance from entity to COM of that node
        // 2) if 1) * theta > size (max diff) then
        let node_as_entity = node.as_entity();
        let dist = self.distance_squared(&node_as_entity);
        let max_dist = node.max_distance();
        return (dist) * (THETA * THETA) > (max_dist * max_dist);
    }

    /// Given two entities, self and other, returns the acceleration that other is exerting on
    /// self. Other can be either an entity or a node.
    fn get_gravitational_acceleration(&self, oth: Either<&Entity, &Node>) -> (f64, f64, f64) {
        // TODO get rid of this clone
        let other = match oth {
            Left(entity) => entity.clone(),
            Right(node) => node.as_entity(),
        };
        let d_magnitude = self.distance(&other);
        let d_vector = self.distance_vector(&other);
        let d_over_d_cubed = (
            d_vector.0 / d_magnitude * d_magnitude,
            d_vector.1 / d_magnitude * d_magnitude,
            d_vector.2 / d_magnitude * d_magnitude,
        );
        let acceleration = (
            d_over_d_cubed.0 * other.mass,
            d_over_d_cubed.1 * other.mass,
            d_over_d_cubed.2 * other.mass,
        );
        return acceleration;
    }

    /// Returns the acceleration of an entity after it has had gravity from the specified node applied to it.
    /// In this function, we approximate some entities if they exceed a certain critera specified in
    /// "exceeds_theta()". If we reach a node and it is a leaf, then we automatically get the
    /// acceleration from every entity in that node, but if we reach a node that is not a leaf and
    /// exceeds_theta() is true, then we treat the node as one giant entity and get the
    /// acceleration from it.
    pub fn get_entity_acceleration_from(&self, node: &Node) -> (f64, f64, f64) {
        let mut acceleration = (0f64, 0f64, 0f64);
        if node.left.is_some() {
            if node.points.is_some() {
                // If the node is a leaf
                for i in node.points.as_ref().expect("unexpected null node 1") {
                    // iterate through entities, accumulating the accelerations
                    // they are exerting on self.
                    let tmp_accel = self.get_gravitational_acceleration(Left(i));
                    acceleration.0 = acceleration.0 + tmp_accel.0;
                    acceleration.1 = acceleration.1 + tmp_accel.1;
                    acceleration.2 = acceleration.2 + tmp_accel.2;
                }
            } else if self.theta_exceeded(&node) {
                // otherwise, check if the theta threshold is exceeded. If so, we want to
                // calculate the forces from the node as a whole
                let tmp_accel = self.get_gravitational_acceleration(Right(&node));
                acceleration.0 = acceleration.0 + tmp_accel.0; // if theta was exceeded, then
                acceleration.1 = acceleration.1 + tmp_accel.1; // get the force from the node's
                acceleration.2 = acceleration.2 + tmp_accel.2; // COM and mass
            } else {
                let tmp_accel = self.get_entity_acceleration_from(&node);
                acceleration.0 = acceleration.0 + tmp_accel.0; // otherwise recurse
                acceleration.1 = acceleration.1 + tmp_accel.1;
                acceleration.2 = acceleration.2 + tmp_accel.2;
            }
        }

        match node.right {
            Some(ref node) => {
                if node.points.is_some() {
                    // same logic as above
                    for i in node.points.as_ref().expect("unexpected null node 2") {
                        let tmp_accel = self.get_gravitational_acceleration(Left(i));
                        acceleration.0 = acceleration.0 + tmp_accel.0;
                        acceleration.1 = acceleration.1 + tmp_accel.1;
                        acceleration.2 = acceleration.2 + tmp_accel.2;
                    }
                } else if self.theta_exceeded(&node) {
                    // TODO
                    let tmp_accel = self.get_gravitational_acceleration(Right(&node));
                    acceleration.0 = acceleration.0 + tmp_accel.0;
                    acceleration.1 = acceleration.1 + tmp_accel.1;
                    acceleration.2 = acceleration.2 + tmp_accel.2;
                } else {
                    let tmp_accel = self.get_entity_acceleration_from(&node);
                    acceleration.0 = acceleration.0 + tmp_accel.0;
                    acceleration.1 = acceleration.1 + tmp_accel.1;
                    acceleration.2 = acceleration.2 + tmp_accel.2;
                }
            }
            None => (),
        }
        return (
            acceleration.0 + acceleration.0,
            acceleration.1 + acceleration.1,
            acceleration.2 + acceleration.2,
        );
    }
}

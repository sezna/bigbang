use super::Dimension;
use crate::simulation_result::SimulationResult;
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
    /// Return an [[Entity]] representation of your struct.
    fn as_entity(&self) -> Entity;
    /// Respond to the forces that bigbang has calculated are acting upon the entity.
    /// It is recommended to at least set the position to where the simulation says
    /// it should be and add the velocity to the position. See the docs for an example.
    fn respond(&self, collision_result: SimulationResult, time_step: f64) -> Self;
}

impl AsEntity for Entity {
    fn as_entity(&self) -> Entity {
        return self.clone();
    }
    fn respond(&self, simulation_result: SimulationResult, time_step: f64) -> Self {
        let (vx, vy, vz) = simulation_result.velocity;
        let (x, y, z) = simulation_result.position;
        Entity {
            vx,
            vy,
            vz,
            x: x + (vx * time_step),
            y: y + (vy * time_step),
            z: z + (vz * time_step),
            radius: self.radius,
            mass: self.mass,
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
    /// Returns a velocity vector which represents the velocity of the particle after it has interacted
    /// with the rest of the tree. Also returns a boolean representing whether or not a collision happened.
    /// TODO make this a struct???
    pub fn interact_with<T: AsEntity + Clone>(
        &self,
        node: &Node<T>,
        time_step: f64,
    ) -> SimulationResult {
        let result = self.collide(node, None);
        let collided = result.collided;
        // If there was a collision and we were not already colliding, use that velocity.
        let (vx, vy, vz) = result.velocity;
        // Set the position of all the entities so that nothing is overlapping

        // Get the gravitational acceleration from the tree...
        let acceleration = self.get_entity_acceleration_from(node);
        // Apply the gravitational acceleration to the calculated velocity.
        SimulationResult {
            velocity: (
                vx + acceleration.0 * time_step,
                vy + acceleration.1 * time_step,
                vz + acceleration.2 * time_step,
            ),
            collided,
            position: result.position,
        }
    }

    /// Needs to be reworked to use min/max position values, but it naively checks
    /// if two things collide right now.
    fn did_collide_into(&self, other: &Entity) -> bool {
        self != other && self.distance(other) <= (self.radius + other.radius)
    }

    fn collide<T: AsEntity + Clone>(
        &self,
        node: &Node<T>,
        starter_velocities: Option<(f64, f64, f64)>,
    ) -> SimulationResult {
        let mut collided = false;
        let (mut vx, mut vy, mut vz) = if let Some(v) = starter_velocities {
            v
        } else {
            (self.vx, self.vy, self.vz)
        };
        let (mut x, mut y, mut z) = (self.x, self.y, self.z);
        // If the two entities are touching...
        if self.did_collide_into(&node.as_entity()) {
            // ...then there is the potential for a collision.
            // If this is a leaf node...
            if let Some(points) = &node.points {
                // Check every particle in the leaf to see if it collided.
                for other_t in points.iter() {
                    let other = other_t.as_entity();
                    // if they collided...
                    if self.did_collide_into(&other) {
                        // do some math.
                        let mass_coefficient_v1 =
                            (self.mass - other.mass) / (self.mass + other.mass);
                        let mass_coefficient_v2 = (2f64 * other.mass) / (self.mass + other.mass);
                        if self.mass < other.mass {
                            // If this entity is smaller than the other one, nudge it.
                            let mut dist_vec = other.distance_vector(self);
                            let dist_scalar = other.distance(self);
                            let buffer_space = 1.01 * (self.radius + other.radius) / dist_scalar;
                            dist_vec = (
                                dist_vec.0 * buffer_space,
                                dist_vec.1 * buffer_space,
                                dist_vec.2 * buffer_space,
                            );
                            x = other.x + dist_vec.0;
                            y = other.y + dist_vec.1;
                            z = other.z + dist_vec.2;
                        }
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
                    let result = self.collide(&left, Some((vx, vy, vz)));
                    if result.collided {
                        collided = result.collided;
                        vx = result.velocity.0;
                        vy = result.velocity.1;
                        vz = result.velocity.2;
                        x = result.position.0;
                        y = result.position.1;
                        z = result.position.2;
                    }
                }
                // and the right...
                if let Some(right) = &node.right {
                    // If there was a collision...
                    let result = self.collide(&right, Some((vx, vy, vz)));
                    if result.collided {
                        collided = result.collided;
                        vx = result.velocity.0;
                        vy = result.velocity.1;
                        vz = result.velocity.2;
                        x = result.position.0;
                        y = result.position.1;
                        z = result.position.2;
                    }
                }
            }
        }
        return SimulationResult {
            collided,
            velocity: (vx, vy, vz),
            position: (x, y, z),
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
        // This is being called from somewhere where `other` has NaN values
        let (x_dist, y_dist, z_dist) = self.distance_vector(other);
        x_dist * x_dist + y_dist * y_dist + z_dist * z_dist
    }
    /// Returns the distance between the two entities
    fn distance(&self, other: &Entity) -> f64 {
        // sqrt((x2 - x1) + (y2 - y1) + (z2 - z1))
        f64::sqrt(self.distance_squared(other))
    }
    /// Returns the distance between two entities as an (x:f64,y:f64,z:f64) tuple.
    fn distance_vector(&self, other: &Entity) -> (f64, f64, f64) {
        let x_dist = other.x - self.x;
        let y_dist = other.y - self.y;
        let z_dist = other.z - self.z;
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
        if d_magnitude == 0. {
            // sort of other use of THETA here
            return (0., 0., 0.);
        }
        let d_vector = self.distance_vector(&other);
        let d_mag_cubed = d_magnitude * d_magnitude; // TODO cube this
        let d_over_d_cubed = (
            d_vector.0 / d_mag_cubed,
            d_vector.1 / d_mag_cubed,
            d_vector.2 / d_mag_cubed,
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

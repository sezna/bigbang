use either::{Either, Left, Right};

use super::Dimension;
use crate::{as_entity::AsEntity, simulation_result::SimulationResult, Entity, Node};
use emu_core::prelude::*;
use emu_glsl::*;
use zerocopy::*;

// TODO:
// - collisions
// - Rewrite with new version of Emu
#[repr(C)]
#[derive(AsBytes, FromBytes, Copy, Clone, Default, Debug)]
struct GpuEntity {
    x: f64,
    y: f64,
    z: f64,
    mass: f64,
}

impl GpuEntity {
    fn new(x: f64, y: f64, z: f64, mass: f64) -> GpuEntity {
        GpuEntity { x, y, z, mass }
    }
}

trait GpuEntity: AsEntity {
    /// Needs to be reworked to use min/max position values, but it naively checks
    /// if two things collide right now.
    fn did_collide_into(&self, other: &Entity) -> bool {
        self != other && self.distance(other) <= (self.radius + other.radius)
    }

    fn get_dim(&self, dim: &Dimension) -> &f64 {
        match *dim {
            Dimension::X => &self.x,
            Dimension::Y => &self.y,
            Dimension::Z => &self.z,
        }
    }

    /// Returns a boolean representing whether or node the node is within the theta range
    /// of the entity.
    fn theta_exceeded<T: AsEntity + Clone>(&self, node: &Node<T>, theta: f64) -> bool {
        // 1) distance from entity to COM of that node
        // 2) if 1) * theta > size (max diff) then
        // This frequently makes a node with NaN positions
        let node_as_entity = node.as_entity();
        let dist = self.distance_squared(&node_as_entity);
        let max_dist = node.max_distance();
        (dist) * (theta * theta) > (max_dist * max_dist)
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
    fn get_acceleration_points<'a, T: AsEntity + Clone>(
        &'a self,
        node: &'a Node<T>,
        theta: f64,
    ) -> Vec<GpuEntity> {
        // First,  build a vector of all the positions and their masses that are going to get gravitational
        // acceleration calculated.
        // (x, y, z, mass)
        let mut accel_points: Vec<GpuEntity> = Vec::new();
        for opt_node in [&node.left, &node.right].iter() {
            if let Some(node) = opt_node {
                if node.points.is_some() {
                    for point in node
                        .points
                        .as_ref()
                        .expect("Broken tree structure: unexpected null node")
                    {
                        let point = point.as_entity();
                        accel_points.push(GpuEntity {
                            x: point.x,
                            y: point.y, 
                            z: point.z, 
                            mass: point.mass
                        });                    }
                } else if self.theta_exceeded(&node, theta) {
                    let point = node.as_entity();
                    accel_points.push(GpuEntity {
                        x: point.x,
                        y: point.y, 
                        z: point.z, 
                        mass: point.mass
                    });
                } else {
                    let mut recursed_accel_points = self.get_acceleration_points(&node, theta);
                    accel_points.append(&mut recursed_accel_points);
                }
            }
        }
        return accel_points;
    }

    fn calculate_acceleration(&self, &mut points: Vec<GpuEntity>) -> (f64, f64, f64) {
        // ensure the device pool is initialized
        futures::executor::block_on(assert_device_pool_initialized());
        

        // offload data to the gpu
        let offloaded_data: DeviceBox<[GpuEntity]> = points.as_device_boxed().expect("Failed to offload data to GPU");

        // compile GslKernel to SPIR-V
        let c = compile::<GlslKernel, GlslKernelCompile, Vec<u32>, GlobalCache>(
            GlslKernel::new()
                .spawn(64)
                .share("float stuff[64]")
                .param_mut::<[GpuEntity], _>("GpuEntity[] entities")
                .with_struct::<GpuEntity>()
                .with_helper_code(
                    r#"
    GpuEntity flip(Shape s) {
        s.x = s.x + s.w;
        s.y = s.y + s.h;
        s.w *= -1;
        s.h *= -1;
        s.r = ivec2(5, 3);
        return s;
    }
    "#,
        ) // TODO reconcile the above with this example: https://github.com/calebwin/emu/blob/master/emu_core/examples/basic.rs
        /*
        let mut self_coords = vec![self.x, self.y, self.z];
        let mut final_accel = vec![0f64, 0f64, 0f64];
        gpu_do!(load(final_accel));
        gpu_do!(load(points));
        gpu_do!(load(self_coords));
        gpu_do!(launch());
        for point in points {
            let self_x = self_coords[0];
            let self_y = self_coords[1];
            let self_z = self_coords[2];
            // Get the distance from self to this tuple
            // sqrt((x2 - x1) + (y2 - y1) + (z2 - z1))
            let x_dist = point.0 - self_x;
            let y_dist = point.1 - self_y;
            let z_dist = point.2 - self_z;
            let d_magnitude = f64::sqrt(x_dist * x_dist + y_dist * y_dist + z_dist * z_dist);

            // if d_magnitude == 0. {
            //     // sort of other use of THETA here
            //     return (0., 0., 0.);
            // }
            let d_mag_cubed = d_magnitude * d_magnitude; // TODO cube this // TODO why did i say this?
            let d_over_d_cubed = (
                x_dist / d_mag_cubed,
                y_dist / d_mag_cubed,
                z_dist / d_mag_cubed,
            );
            final_accel[0] += d_over_d_cubed.0 * point.3;
            final_accel[1] += d_over_d_cubed.1 * point.3;
            final_accel[2] += d_over_d_cubed.2 * point.3;
        }
        gpu_do!(read(final_accel));
        (final_accel[0], final_accel[1], final_accel[2])
        */
    }
}
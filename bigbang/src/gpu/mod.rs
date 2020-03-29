use either::{Either, Left, Right};

use super::Dimension;
use crate::{as_entity::AsEntity, simulation_result::SimulationResult, Entity, Node};

// TODO:
// - collisions
// - Rewrite with new version of Emu
#[repr(C)]
struct AccelEntity {
    x: f64,
    y: f64,
    z: f64,
    mass: f64,
}

impl AccelEntity {
    fn new(x: f64, y: f64, z: f64, mass: f64) -> AccelEntity {
        AccelEntity { x, y, z, mass }
    }
}

trait GpuEntity: AsEntity {
    /// Needs to be reworked to use min/max position values, but it naively checks
    /// if two things collide right now.
    fn did_collide_into(&self, other: &Entity) -> bool {
        let self_entity = self.as_entity();
        let other_entity = other.as_entity();
        
        self_entity != other_entity && self_entity.distance(&other_entity) <= (self_entity.radius + other_entity.radius)
    }

    /// Returns a boolean representing whether or node the node is within the theta range
    /// of the entity.
    fn theta_exceeded<T: AsEntity + Clone>(&self, node: &Node<T>, theta: f64) -> bool {
        // 1) distance from entity to COM of that node
        // 2) if 1) * theta > size (max diff) then
        // This frequently makes a node with NaN positions
        let node_as_entity = node.as_entity();
        let dist = self.as_entity().distance_squared(&node_as_entity);
        let max_dist = node.max_distance();
        (dist) * (theta * theta) > (max_dist * max_dist)
    }

    /// Given two entities, self and other, returns the acceleration that other is exerting on
    /// self. Other can be either an entity or a node.
    fn get_gravitational_acceleration<T: AsEntity + Clone>(
        &self,
        oth: Either<&Entity, &Node<T>>,
    ) -> (f64, f64, f64) {
        let self_entity = self.as_entity();
        // TODO get rid of this clone
        let other = match oth {
            Left(entity) => entity.clone(),
            Right(node) => node.as_entity(),
        };
        let d_magnitude = self_entity.distance(&other);
        if d_magnitude == 0. {
            // sort of other use of THETA here
            return (0., 0., 0.);
        }
        let d_vector = self_entity.distance_vector(&other);
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
    ) -> Vec<AccelEntity> {
        // First,  build a vector of all the positions and their masses that are going to get gravitational
        // acceleration calculated.
        // (x, y, z, mass)
        let mut accel_points: Vec<AccelEntity> = Vec::new();
        for opt_node in [&node.left, &node.right].iter() {
            if let Some(node) = opt_node {
                if node.points.is_some() {
                    for point in node
                        .points
                        .as_ref()
                        .expect("Broken tree structure: unexpected null node")
                    {
                        let point = point.as_entity();
                        accel_points.push(AccelEntity::new(point.x, point.y, point.z, point.mass));
                    }
                } else if self.theta_exceeded(&node, theta) {
                    let point = node.as_entity();
                    accel_points.push(AccelEntity::new(point.x, point.y, point.z, point.mass));
                } else {
                    let mut recursed_accel_points = self.get_acceleration_points(&node, theta);
                    accel_points.append(&mut recursed_accel_points);
                }
            }
        }
        return accel_points;
    }

    fn calculate_acceleration<'a, T: AsEntity + Clone> (&self, node: &'a Node<T>, theta: f64) {
        let mut entities = self.get_acceleration_points(node, theta);
        gpu::calculate_acceleration(entities);
    }
    /*
    fn calculate_acceleration(&self, &mut points: Vec<(f64, f64, f64, f64)>) -> (f64, f64, f64) {
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
    }
    */
}

mod gpu {
    // Copyright (c) 2017 The vulkano developers
    // Licensed under the Apache License, Version 2.0
    // <LICENSE-APACHE or
    // http://www.apache.org/licenses/LICENSE-2.0> or the MIT
    // license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
    // at your option. All files in the project carrying such
    // notice may not be copied, modified, or distributed except
    // according to those terms.

    // This example demonstrates how to use the compute capabilities of Vulkan.
    //
    // While graphics cards have traditionally been used for graphical operations, over time they have
    // been more or more used for general-purpose operations as well. This is called "General-Purpose
    // GPU", or *GPGPU*. This is what this example demonstrates.

    use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
    use vulkano::command_buffer::AutoCommandBufferBuilder;
    use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
    use vulkano::descriptor::PipelineLayoutAbstract;
    use vulkano::device::{Device, DeviceExtensions};
    use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
    use vulkano::pipeline::ComputePipeline;
    use vulkano::sync;
    use vulkano::sync::GpuFuture;

    use std::sync::Arc;

    pub(super) fn calculate_acceleration(accel_entities: Vec<super::AccelEntity>) {
        // As with other examples, the first step is to create an instance.
        let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();

        // Choose which physical device to use.
        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();

        // Choose the queue of the physical device which is going to run our compute operation.
        //
        // The Vulkan specs guarantee that a compliant implementation must provide at least one queue
        // that supports compute operations.
        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_compute())
            .unwrap();

        // Now initializing the device.
        let (device, mut queues) = Device::new(
            physical,
            physical.supported_features(),
            &DeviceExtensions {
                khr_storage_buffer_storage_class: true,
                ..DeviceExtensions::none()
            },
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();

        // Since we can request multiple queues, the `queues` variable is in fact an iterator. In this
        // example we use only one queue, so we just retrieve the first and only element of the
        // iterator and throw it away.
        let queue = queues.next().unwrap();

        println!("Device initialized");

        // Now let's get to the actual example.
        //
        // What we are going to do is very basic: we are going to fill a buffer with 64k integers
        // and ask the GPU to multiply each of them by 12.
        //
        // GPUs are very good at parallel computations (SIMD-like operations), and thus will do this
        // much more quickly than a CPU would do. While a CPU would typically multiply them one by one
        // or four by four, a GPU will do it by groups of 32 or 64.
        //
        // Note however that in a real-life situation for such a simple operation the cost of
        // accessing memory usually outweighs the benefits of a faster calculation. Since both the CPU
        // and the GPU will need to access data, there is no other choice but to transfer the data
        // through the slow PCI express bus.

        // We need to create the compute pipeline that describes our operation.
        //
        // If you are familiar with graphics pipeline, the principle is the same except that compute
        // pipelines are much simpler to create.
        let pipeline = Arc::new({
            mod cs {
                vulkano_shaders::shader! {
                    ty: "compute",
                    src: "
                    #version 450
                    layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
                    struct AccelEntity {
                        double x;
                        double y;
                        double z;
                        double mass;
                    };

                    layout(set = 0, binding = 0) buffer Entities {
                        AccelEntity[] entities;
                    };
                    void main() {
                        uint idx = gl_GlobalInvocationID.x;
                        calculate_accel(entities[idx]);
                    }

                    
                "
                }
            }
            let shader = cs::Shader::load(device.clone()).unwrap();
            ComputePipeline::new(device.clone(), &shader.main_entry_point(), &()).unwrap()
        });

        // We start by creating the buffer that will store the data.
        let data_buffer = {
            // Iterator that produces the data.
            // Builds the buffer and fills it with this iterator.
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, accel_entities.into_iter())
                .unwrap()
        };

        // In order to let the shader access the buffer, we need to build a *descriptor set* that
        // contains the buffer.
        //
        // The resources that we bind to the descriptor set must match the resources expected by the
        // pipeline which we pass as the first parameter.
        //
        // If you want to run the pipeline on multiple different buffers, you need to create multiple
        // descriptor sets that each contain the buffer you want to run the shader on.
        let layout = pipeline.layout().descriptor_set_layout(0).unwrap();
        let set = Arc::new(
            PersistentDescriptorSet::start(layout.clone())
                .add_buffer(data_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        // In order to execute our operation, we have to build a command buffer.
        let command_buffer =
            AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                .unwrap()
                // The command buffer only does one thing: execute the compute pipeline.
                // This is called a *dispatch* operation.
                //
                // Note that we clone the pipeline and the set. Since they are both wrapped around an
                // `Arc`, this only clones the `Arc` and not the whole pipeline or set (which aren't
                // cloneable anyway). In this example we would avoid cloning them since this is the last
                // time we use them, but in a real code you would probably need to clone them.
                .dispatch([1024, 1, 1], pipeline.clone(), set.clone(), ())
                .unwrap()
                // Finish building the command buffer by calling `build`.
                .build()
                .unwrap();

        // Let's execute this command buffer now.
        // To do so, we TODO: this is a bit clumsy, probably needs a shortcut
        let future = sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            // This line instructs the GPU to signal a *fence* once the command buffer has finished
            // execution. A fence is a Vulkan object that allows the CPU to know when the GPU has
            // reached a certain point.
            // We need to signal a fence here because below we want to block the CPU until the GPU has
            // reached that point in the execution.
            .then_signal_fence_and_flush()
            .unwrap();

        // Blocks execution until the GPU has finished the operation. This method only exists on the
        // future that corresponds to a signalled fence. In other words, this method wouldn't be
        // available if we didn't call `.then_signal_fence_and_flush()` earlier.
        // The `None` parameter is an optional timeout.
        //
        // Note however that dropping the `future` variable (with `drop(future)` for example) would
        // block execution as well, and this would be the case even if we didn't call
        // `.then_signal_fence_and_flush()`.
        // Therefore the actual point of calling `.then_signal_fence_and_flush()` and `.wait()` is to
        // make things more explicit. In the future, if the Rust language gets linear types vulkano may
        // get modified so that only fence-signalled futures can get destroyed like this.
        future.wait(None).unwrap();

        // Now that the GPU is done, the content of the buffer should have been modified. Let's
        // check it out.
        // The call to `read()` would return an error if the buffer was still in use by the GPU.
        let data_buffer_content = data_buffer.read().unwrap();
    }
}

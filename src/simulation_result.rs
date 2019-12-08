use crate::collision_result::CollisionResult;

pub struct SimulationResult<'a, T> {
    /// The result of the simulation's collision check
    /// just a vector of references to potential collisions
    pub collision_result: CollisionResult<'a, T>,
    pub gravitational_acceleration: (f64, f64, f64),
}

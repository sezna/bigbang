use crate::collision_result::CollisionResult;

pub struct SimulationResult<'a, T> {
    pub collision: CollisionResult<'a, T>,
    pub acceleration: (f64, f64, f64),
}

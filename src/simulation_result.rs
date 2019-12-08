
pub struct SimulationResult<'a, T> {
    /// The result of the simulation's collision check
    /// just a vector of references to potential collisions
    pub collisions: Vec<&'a T>,
    pub gravitational_acceleration: (f64, f64, f64),
}

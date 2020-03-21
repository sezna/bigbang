use crate::SimulationResult;

/// Define how to respond to the results of the simulation at every time step.
pub trait Responsive {
    /// Respond to the forces that bigbang has calculated are acting upon the entity.
    /// It is recommended to at least set the position to where the simulation says
    /// it should be and add the velocity to the position. See the examples directory for examples.
    /// Basic collision functions are available in [collisions](crate::collisions].
    fn respond(&self, simulation_result: SimulationResult<Self>, time_step: f64) -> Self
    where
        Self: std::marker::Sized;
}

use crate::entity::Entity;
use crate::simulation_result::SimulationResult;

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
    /// it should be and add the velocity to the position. See the examples directory for examples.
    /// Basic collision functions are available in [collisions](crate::collisions].
    fn respond(&self, simulation_result: SimulationResult<Self>, time_step: f64) -> Self
    where
        Self: std::marker::Sized;
}

use crate::entity::Entity;

/// [[GravTree]] works with any type which implements [[AsEntity]] and [[Responsive]]. In order to implement [[AsEntity]],
/// a type must be able to represent itself as a gravitational spatial entity. This, simply, entails
/// constructing an [[Entity]] from the type, and defining how to add acceleration to the velocity of your type.
///
/// More generally, this entails that a type must contain, or be able to derive, its velocity, position,
/// radius and mass, and it must be able to respond to acceleration impulses in the form of triples of `f64`s.
pub trait AsEntity {
    /// Return an [[Entity]] representation of your struct.
    fn as_entity(&self) -> Entity;
}

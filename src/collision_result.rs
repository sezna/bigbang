/// Contains the forces that bigbang has calculated are being exerted on an entity.
pub struct CollisionResult<'a, T> {
    /// The particles that are potentially being collided with.
    pub collisions: Vec<&'a T>,
}

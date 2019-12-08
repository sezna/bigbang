use crate::Entity;
pub struct CollisionResult {
    pub collided: bool,
    pub velocity: (f64, f64, f64),
    pub position: (f64, f64, f64),
    pub collided_entities: Vec<Entity>,
}

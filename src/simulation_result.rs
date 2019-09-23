/// Contains the forces that bigbang has calculated are being exerted on an entity.
pub struct SimulationResult {
    pub velocity: (f64, f64, f64),
    pub position: (f64, f64, f64),
    pub collided: bool,
}

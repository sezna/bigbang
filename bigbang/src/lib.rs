//! For more details on usage, see [the README](https://github.com/sezna/blob/master/README.md).
extern crate either;
extern crate rayon;
mod as_entity;
mod dimension;
mod entity;
#[cfg(feature = "gpu")]
mod gpu;
mod grav_tree;
mod node;
mod responsive;
mod simulation_result;
mod utilities;

use dimension::Dimension;
use node::Node;
/*  public-facing entry points */
pub use as_entity::AsEntity;
pub use bigbang_derive::AsEntity;
pub use entity::Entity;
pub use grav_tree::GravTree;
pub use responsive::Responsive;
pub use simulation_result::SimulationResult;
pub mod collisions;

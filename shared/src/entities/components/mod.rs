use std::any::Any;

/// The trait that any component must implement to be usable by a [`World`](../struct.World.html).
pub trait Component: Any + Send + Sync {}

pub mod position;

pub use self::position::{PositionComponent, Positional};

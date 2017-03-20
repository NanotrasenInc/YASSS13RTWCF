use std::any::Any;

pub trait Component: Any + Send + Sync {}

pub mod position;

pub use self::position::PositionComponent;

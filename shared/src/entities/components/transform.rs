use nalgebra::core::Vector2;
use super::Component;

pub struct Transform {
    position: Position,
    scale: Vector2<f32>

    // TODO: needs locking children and parent.
}

impl Transform {
    pub fn empty() -> Transform {
        Transform {
            position: Position::empty(),
            scale: Vector2::new(0.0, 0.0)
        }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_scale(&self) -> &Vector2<f32> {
        &self.scale
    }
}

impl Component for Transform {}

// TODO: Move this!
/// Represents an immutable, absolute location somewhere in the game world.
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    location: Vector2<f64>,
    dimension: u16
}

impl Position {
    pub fn new(location: &Vector2<f64>, dimension: u16) -> Position {
        Position {
            location: location.clone(),
            dimension: dimension
        }
    }

    pub fn empty() -> Position {
        Position::new(&Vector2::new(0.0, 0.0), 0)
    }

    pub fn get_location(&self) -> &Vector2<f64> {
        &self.location
    }

    pub fn get_dimension(&self) -> u16 {
        self.dimension
    }
}

use super::Component;
use nalgebra::Vector2;


/// A component for giving an entity a position.
#[derive(Debug)]
pub struct PositionComponent {
    position: Positional
}

impl PositionComponent {
    pub fn new(position: Positional) -> PositionComponent {
        PositionComponent {
            position: position
        }
    }

    pub fn empty() -> PositionComponent {
        PositionComponent::new(Positional::empty())
    }

    pub fn get_position(&self) -> Positional {
        self.position
    }

    pub fn set_position(&mut self, position: Positional) {
        self.position = position
    }
}

impl Component for PositionComponent {}

/// Represents an absolute position somewhere in the world.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Positional {
    pub coordinates: Vector2<f64>,
    pub dimension: u16
}

impl Positional {
    pub fn new(coordinates: Vector2<f64>, dimension: u16) -> Positional {
        Positional {
            coordinates: coordinates,
            dimension: dimension
        }
    }

    pub fn empty() -> Positional {
        Positional::new(Vector2::new(0.0, 0.0), 0)
    }
}

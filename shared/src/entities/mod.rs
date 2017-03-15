pub mod components;
pub mod entity;
pub mod world;

type ID = u64;

pub use self::entity::Entity;
pub use self::world::World;
pub use self::entitybuilder::EntityBuilder;
use std::any::TypeId;
use std::collections::HashMap;
use super::components::Component;
use super::world::WORLD;

pub struct EntityBuilder {
    components: HashMap<TypeId, Box<Component>>
}

impl EntityBuilder {
    fn new() -> EntityBuilder {
        EntityBuilder {components: HashMap::new()}
    }

    /// Add a new component instance `T` to be added to the final entity.
    pub fn with_component<T: Component>(mut self, component: T) -> EntityBuilder {
        self.components.insert(TypeId::of::<T>(), component)
        self
    }

    /// Finish this entity and return the reference to it.
    ///
    /// This uses the global `WORLD`.
    pub fn finish(self) -> Entity {
        let world = WORLD.write().unwrap();

        world.new_entity(self)
    }
}
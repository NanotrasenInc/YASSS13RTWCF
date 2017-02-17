use std::sync::RwLock;
use std::any::{TypeId, Any};
use std::collections::HashMap;
use super::{ID, Entity};
use super::components::Component;
use std::sync::Arc;

lazy_static! {
    pub static ref WORLD: World = {
        World { components: HashMap::new(), entities: RwLock::new(HashMap::new()) }
    };
}


pub struct World {
    // The `Box<Any + Send + Sync>` is supposed to represent a ComponentStorage.
    components: HashMap<TypeId, Box<Any + Send + Sync>>,
    entities: RwLock<HashMap<ID, Arc<RwLock<Entity>>>>
}

impl World {
    /// Register a new component type for being used by entities.
    pub fn register_component<T: Component>(&mut self) {
        self.components.insert(TypeId::of::<T>(), Box::new(ComponentStorage::<T>::new()));
    }

    pub fn get_entity<T: Component>(&self, id: ID) -> Option<Arc<RwLock<Entity>>> {
        let lock = self.entities.read().expect("Alright who the FUCK poisoned the world?");
        lock.get(&id).map(|x| x.clone())
    }
}

struct ComponentStorage<T: Component>(RwLock<HashMap<ID, RwLock<T>>>);

impl<T: Component> ComponentStorage<T> {
    fn new() -> ComponentStorage<T> {
        ComponentStorage(RwLock::new(HashMap::new()))
    }
}

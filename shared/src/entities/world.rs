// TODO: Rewrite all of this to not perform terribly.

use std::sync::{RwLock, RwLockReadGuard};
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::hash_map;
use super::{ID, Entity, EntityBuilder};
use super::components::Component;
use std::sync::Arc;
use mopa;

lazy_static! {
    pub static ref WORLD: RwLock<World> = {
        RwLock::new(World { components: HashMap::new(), entities: HashMap::new() })
    };
}

/// Represents a "storage" for entities and their components.
pub struct World {
    // The `Box<Any + Send + Sync>` is supposed to represent a ComponentStorage.
    components: HashMap<TypeId, Box<ComponentStorageTrait>>,
    entities: HashMap<ID, Arc<RwLock<Entity>>>,
    /// The last ID allocated.
    id: ID,
}

impl World {
    /// Register a new component type for being used by entities.
    pub fn register_component<T: Component>(&mut self) {
        self.components.insert(TypeId::of::<T>(), Box::new(ComponentStorage::<T>::new()));
    }

    /// Get an entity by ID.
    pub fn get_entity(&self, id: ID) -> Option<Arc<RwLock<Entity>>> {
        self.entities.get(&id).map(|x| x.clone())
    }

    /// Gets a component of an entity, by the entity ID.
    ///
    /// # Panics.
    /// Panics if the component type is not registered.
    pub fn get_component<T: Component>(&self, id: ID) -> Option<Arc<RwLock<T>>> {
        let lock = &self.get_storage::<T>().0;
        let storage = lock.read().unwrap();
        let ret = storage.get(&id).map(|x| x.clone());
        ret
    }
}


// Non-public stuff here!
impl World {
    /// Gets a storage.
    ///
    /// # Panics.
    /// Panics if the storage doesn't exist.
    fn get_storage<T: Component>(&self) -> &ComponentStorage<T> {
        &self.components
            .get(&TypeId::of::<T>())
            .expect("Tried to retrieve unregistered component storage.")
            .downcast_ref()
            .unwrap()
    }

    fn new_entity(&mut self, builder: EntityBuilder) -> Arc<RwLock<Entity>> {
        self.id += 1;

    }
}

pub fn iter_components<'a, T: Component>() -> ComponentIter<'a, T> {
    let lock = WORLD.read().unwrap();
    let storage = lock.get_storage::<T>();
    ComponentIter {
        iter: storage.0.iter(),
        lock: lock,
    }
}

trait ComponentStorageTrait: mopa::Any + Send + Sync {}
mopafy!(ComponentStorageTrait);

struct ComponentStorage<T: Component>(HashMap<ID, Arc<RwLock<T>>>);
impl<T: Component> ComponentStorageTrait for ComponentStorage<T> {}
impl<T: Component> ComponentStorage<T> {
    fn new() -> ComponentStorage<T> {
        ComponentStorage(HashMap::new())
    }
}

pub struct ComponentIter<'a, T: Component> {
    iter: hash_map::Iter<'a, ID, Arc<RwLock<T>>>,
    /// Lock on the world read guard.
    lock: RwLockReadGuard<'static, World>,
}

impl<'a, T: Component> Iterator for ComponentIter<'a, T> {
    type Item = (ID, Arc<RwLock<T>>);

    fn next(&mut self) -> Option<(ID, Arc<RwLock<T>>)> {
        self.iter.next().map(|(id, arc)| (*id, arc.clone()))
    }
}
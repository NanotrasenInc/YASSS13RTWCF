//! This module provides all

pub mod components;

use std::sync::{RwLock, RwLockWriteGuard};
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::hash_map;
use entities::components::{Component, PositionComponent};
use std::sync::Arc;
use mopa;

type ID = u64;

// Once upon a time this was in different files.
// Then I realized that was a pain without making everything public.
// TODO: Rewrite all the locking and stuff. This is horrible for performance and lock contention.

lazy_static! {
    /// A global [`World`](struct.world.html) for common use.
    /// This is a lazy static, but it can be used as a global variable.
    pub static ref THEWORLD: RwLock<World> = {
        let mut world = World::new();

        world.register_component::<PositionComponent>();

        RwLock::new(world)
    };
}

// Get around a case sensitivity collision with WORLD and World,
// Which breaks cargo doc.
pub use self::THEWORLD as WORLD;

/// Represents a "storage" for entities and their components.
pub struct World {
    // The `Box<Any + Send + Sync>` is supposed to represent a ComponentStorage.
    components: HashMap<TypeId, Box<ComponentStorageTrait>>,
    entities: HashMap<ID, Arc<RwLock<Entity>>>,
    /// The last ID allocated.
    id: ID,
}

impl World {
    /// Create a new world.
    ///
    /// Do not use this outside tests, use the static `WORLD` instead.
    pub fn new() -> World {
        World {
            components: HashMap::new(),
            entities: HashMap::new(),
            id: 0,
        }
    }

    /// Register a new component type for being used by entities.
    /// This must always be called before attempting to use a component type!
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
        let storage = &self.get_storage().0;
        let ret = storage.get(&id).map(|x| x.clone());
        ret
    }

    /// Returns an iterator over all components of a certain type.
    ///
    /// # Panics.
    /// Panics if the component type is not registered.
    pub fn iter_components<'a, T: Component>(&'a self) -> ComponentIter<'a, T> {
        let storage = self.get_storage();
        let iter = storage.0.iter();
        ComponentIter { iter: iter }
    }

    /// Returns an iterator over all the entities.
    pub fn iter_entities<'a>(&'a self) -> EntityIter<'a> {
        EntityIter { iter: self.entities.iter() }
    }
}

impl World {
    fn get_storage<'a, T: Component>(&'a self) -> &'a ComponentStorage<T> {
        self.components
            .get(&TypeId::of::<T>())
            .expect("Tried to retrieve unregistered component storage.")
            .downcast_ref()
            .unwrap()
    }

    fn get_storage_mut<T: Component>(&mut self) -> &mut ComponentStorage<T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .expect("Tried to retrieve unregistered component storage.")
            .downcast_mut()
            .unwrap()
    }
}

/// Returns an `EntityBuilder` to make a new entity.
/// See the documentation on `EntityBuilder` for more info.
///
/// This function takes in a reference to an `RwLock<World>`.
/// As such you do not want to lock the `World` beforehand.
/// Pass the global `WORLD` for common use instead.
///
/// **NOTE**: While the `EntityBuilder` is held, the world is mutably locked.
/// If already have a lock on the world (or are planning to make one), this *will* deadlock!
pub fn make_builder<'a>(world: &'a RwLock<World>) -> EntityBuilder<'a> {
    let mut world = world.write().unwrap();
    let id = world.id;
    world.id += 1;
    world.entities.insert(id, Arc::new(RwLock::new(Entity { id: id })));

    EntityBuilder::new(world, id)
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

/// An iterator over all components of a certain type in a `World`.
pub struct ComponentIter<'a, T: Component> {
    iter: hash_map::Iter<'a, ID, Arc<RwLock<T>>>,
}

impl<'a, T: Component> Iterator for ComponentIter<'a, T> {
    type Item = (ID, Arc<RwLock<T>>);

    fn next(&mut self) -> Option<(ID, Arc<RwLock<T>>)> {
        self.iter.next().map(|(id, arc)| (*id, arc.clone()))
    }
}

/// An iterator over all entities in a `World`.
pub struct EntityIter<'a> {
    iter: hash_map::Iter<'a, ID, Arc<RwLock<Entity>>>,
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = (ID, Arc<RwLock<Entity>>);

    fn next(&mut self) -> Option<(ID, Arc<RwLock<Entity>>)> {
        self.iter.next().map(|(id, arc)| (*id, arc.clone()))
    }
}

/// An interface into building an entity.
///
/// An `EntityBuilder` is returned by [`make_builder`](fn.make_builder.html),
/// it is a "window" into the creation of the entity,
/// and allows you to give it components on creation.
///
/// # Example
///
/// ```
/// use shared::entities::{WORLD, make_builder};
/// use shared::entities::components::PositionComponent;
/// let new_entity = make_builder(&WORLD)
///                  .with_component(PositionComponent::empty())
///                  .finish();
/// assert_eq!(new_entity.read().unwrap().get_id(), 0);
/// ```
pub struct EntityBuilder<'a> {
    id: ID,
    world: RwLockWriteGuard<'a, World>,
}

impl<'a> EntityBuilder<'a> {
    fn new(world: RwLockWriteGuard<World>, id: ID) -> EntityBuilder {
        EntityBuilder {
            id: id,
            world: world,
        }
    }

    /// Add a new component instance `T` to be added to the final entity.
    pub fn with_component<T: Component>(mut self, component: T) -> EntityBuilder<'a> {
        {
            let ref mut storage = self.world.get_storage_mut().0;
            storage.insert(self.id, Arc::new(RwLock::new(component)));
        }
        self
    }

    /// Consumes the `EntityBuilder`, returning the entity that was created.
    ///
    /// It should be noted that, as of the time of writing,
    /// This function doesn't need to be called to have the entity be created.
    pub fn finish(self) -> Arc<RwLock<Entity>> {
        self.world.get_entity(self.id).unwrap()
    }
}

#[derive(Hash, Debug)]
pub struct Entity {
    id: ID,
}

impl Entity {
    pub fn get_id(&self) -> ID {
        self.id
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

impl PartialEq<ID> for Entity {
    fn eq(&self, other: &ID) -> bool {
        self.id == *other
    }
}

impl<'a> PartialEq<RwLockWriteGuard<'a, Entity>> for Entity {
    fn eq(&self, other: &RwLockWriteGuard<'a, Entity>) -> bool {
        self.id == other.id
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent {
        a: i32,
    }

    impl Component for TestComponent {}

    #[test]
    fn test_basic() {
        let world = RwLock::new(World::new());
        if let Some(_) = world.read().unwrap().get_entity(0) {
            panic!("World returned a Some!")
        }
        let entity = make_builder(&world).finish();
        let other = world.read()
            .unwrap()
            .get_entity(0)
            .unwrap();

        assert_eq!(entity.read().unwrap().get_id(),
                   other.read().unwrap().get_id());
    }

    #[test]
    fn test_component() {
        let world = RwLock::new(World::new());
        world.write().unwrap().register_component::<TestComponent>();
        make_builder(&world).with_component(TestComponent { a: 123 }).finish();

        let comp = world.read()
            .unwrap()
            .get_component::<TestComponent>(0)
            .unwrap();

        assert_eq!(comp.read().unwrap().a, 123);
    }

    #[test]
    fn test_iter_entities() {
        let world = RwLock::new(World::new());
        make_builder(&world).finish();
        make_builder(&world).finish();
        make_builder(&world).finish();

        let lock = world.read().unwrap();
        let mut iter = lock.iter_entities();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();
        assert_eq!(a.0,
                   a.1
                       .read()
                       .unwrap()
                       .get_id());
        assert_eq!(b.0,
                   b.1
                       .read()
                       .unwrap()
                       .get_id());
        assert_eq!(c.0,
                   c.1
                       .read()
                       .unwrap()
                       .get_id());
    }
}

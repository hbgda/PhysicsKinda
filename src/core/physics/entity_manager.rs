use std::{collections::{HashMap, hash_map::{Iter, IterMut}}, sync::atomic::{AtomicUsize, Ordering}};
use log;

use sdl2::rect::Rect;

use crate::core::rendering::ext::entity::PhysicsEntityExt;

use super::{entity::PhysicsEntity, vector::Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityID(pub usize);

pub struct EntityManager {
    _registry: HashMap<EntityID, PhysicsEntity>,
    _id_generator: AtomicUsize
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            _registry: HashMap::new(),
            _id_generator: AtomicUsize::new(0)
        }
    }
}

impl EntityManager {
    fn generate_id(&self) -> EntityID {
        EntityID(self._id_generator.fetch_add(1, Ordering::SeqCst))
    }

    pub fn create_entity(&mut self) -> (EntityID, &mut PhysicsEntity) {
        let id = self.generate_id();
        let entity = PhysicsEntity::default();
        self._registry.insert(
            id, 
            entity
        );
        (id, self.get_entity_mut(id).unwrap())
    }
    
    pub fn destroy(&mut self, entity_id: EntityID) {
        self._registry.remove(&entity_id).unwrap();
        println!("Destroyed entity: {entity_id:?}");
    }

    pub fn get_entity(&self, id: EntityID) -> Option<&PhysicsEntity> {
        if let Some(entity) = self._registry.get(&id) {
            return Some(entity)
        }
        None
    }

    pub fn get_entity_mut(&mut self, id: EntityID) -> Option<&mut PhysicsEntity> {
        if let Some(entity) = self._registry.get_mut(&id) {
            return Some(entity)
        }
        None
    }
}

impl EntityManager {
    pub fn iter_mut(&mut self) -> IterMut<EntityID, PhysicsEntity> {
        self._registry.iter_mut()
    }

    pub fn iter(&self) -> Iter<EntityID, PhysicsEntity> {
        self._registry.iter()
    }

    pub fn ids(&self) -> Vec<EntityID> {
        self._registry.keys().map(|id| *id).collect()
    }

    pub fn all(&self) -> Vec<&PhysicsEntity> {
        self._registry.values().collect()
    }

    pub fn rects(&self, viewport: Vector::<u32>) -> Vec<Rect> {
        self._registry.iter().map(|(_, e)| e.to_rect(viewport)).collect()
    }
}

use std::{sync::{Mutex, atomic::{AtomicUsize, Ordering}}, collections::HashMap, fmt::Error};

use super::{entity::*, entity_manager::{EntityManager, EntityID}};
use super::vector::Vector;


pub struct Engine {
    pub entities: EntityManager,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            entities: EntityManager::new()
        }
    }
}

impl Engine {
    pub fn update(&mut self) {
        for id in self.entities.ids() {
            let _ = self.update_entity(id);
        }
    }

    fn update_entity(&mut self, id: EntityID) -> Result<(), String>
    {
        // Just basic currently, may eventually be updated to include various physical elements
        // such as air resistance, gravity, acceleration etc
        
        let maybe_entity = self.entities.get_entity_mut(id);
        if let None = maybe_entity {
            return Err(format!("Unknown entity of id {:?}", &id))
        }

        let entity = maybe_entity.unwrap();

        entity.position += entity.velocity;

        Ok(())
    }
}

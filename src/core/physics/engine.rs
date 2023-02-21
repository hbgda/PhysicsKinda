use std::{sync::{Mutex, atomic::{AtomicUsize, Ordering}}, collections::HashMap};

use super::entity::*;

static ENTITY_ID_GEN: AtomicUsize = AtomicUsize::new(0);

pub struct Engine<'e> {
    _entities: HashMap<usize, &'e dyn PhysicsEntity>
}

impl Engine<'_> {
    pub fn new() -> Self {
        Engine {
            _entities: HashMap::new()
        }
    }
}

impl<'e> Engine<'e> {
    pub fn register_entity<T>(&mut self, entity: &'e mut T)
    where 
        T: PhysicsEntity
    {
        self._entities.insert(
            ENTITY_ID_GEN.fetch_add(1, Ordering::SeqCst),
            entity as &dyn PhysicsEntity
        );
    }
}

impl <'e> Engine<'e> {
    pub fn update(&mut self) {
        let mut x = &mut self._entities;
        for (_, mut entity) in x.iter_mut() {
            Engine::update_entity(&mut entity);
        }   
    }

    fn update_entity(entity: &mut &dyn PhysicsEntity)
    {
        // Just basic currently, will eventually be updated to include various physical elements
        // such as air resistance, gravity, acceleration etc
        let curr_pos = entity.get_position();
        let velocity = entity.get_velocity();
        let updated_pos = (
            curr_pos.0 + velocity.0,
            curr_pos.1 + velocity.1
        );
        (&mut entity).set_position(updated_pos)
    }
}

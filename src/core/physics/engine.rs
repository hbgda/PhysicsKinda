use std::{sync::{Mutex, atomic::{AtomicUsize, Ordering}}, collections::HashMap, fmt::Error};

use super::{entity::*, entity_manager::{EntityManager, EntityID}, engine_properties::EngineProperties, units::METER, collision::Collision};
use super::vector::Vector;
use crate::core::rendering::ext::entity::PhysicsEntityExt;


pub struct Engine {
    viewport: Vector<u32>,
    pub entities: EntityManager,
    pub properties: EngineProperties,
    collision: Collision
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            entities: EntityManager::new(),
            viewport: Vector::<u32>::new(800, 600),
            properties: EngineProperties { gravity: (4 * METER) / 60 },
            collision: Collision::new()
        }
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.viewport.set(width, height);
    }
}

impl Engine {
    pub fn update(&mut self) {
        self.collision.clear();
        let ids = self.entities.ids();
        for id in &ids {
            let _ = self.update_entity(*id);
        }
        self.update_collision();
    }

    fn update_collision(&mut self) {
        let ids = self.entities.ids();
        for id in &ids {
            for other in &ids {
                if id != other {
                    let (does_collide, collision_axis, amount) = self.collision.check_collision(
                        self.entities.get_entity(*id).unwrap().to_rect(self.viewport),
                        self.entities.get_entity(*other).unwrap().to_rect(self.viewport) 
                    );
                    if does_collide {
                        self.entities.get_entity_mut(*id).unwrap().position += Vector::<i32>::new(0, -50);
                    }
                }
            }
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
        entity.velocity += Vector::<i32>::new(0, self.properties.gravity as i32);
        entity.bound(self.viewport);

        Ok(())
    }
}

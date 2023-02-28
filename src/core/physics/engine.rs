use super::{entity_manager::{EntityManager, EntityID}, properties::engine_props::EngineProperties, units::{METER, TPS}, collision::Collision};
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
            properties: EngineProperties { gravity: (4 * METER as i32) / TPS as i32 },
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
            // dbg!(id.0);
            for other in &ids {
                if id != other {
                    let intersection = self.collision.check_collision(
                        self.entities.get_entity(*id).unwrap().to_rect(self.viewport),
                        self.entities.get_entity(*other).unwrap().to_rect(self.viewport) 
                    );
                    if !intersection.0 {
                        if let Some(rect) = intersection.1 {
                            // dbg!(id.0, rect);
                            let entity = self.entities.get_entity_mut(*id).unwrap();
                            if entity.velocity.x() > 0 {
                                entity.position -= Vector::<i32>::new(rect.w, 0);
                            }
                            else if entity.velocity.x() < 0 {
                                entity.position += Vector::<i32>::new(rect.w, 0);
                            }

                            if entity.velocity.y() > 0 {
                                entity.position -= Vector::<i32>::new(0, rect.h);
                            }
                            else if entity.velocity.y() < 0 {
                                entity.position += Vector::<i32>::new(0, rect.h);
                            }
                            // entity.position -= Vector::<i32>::new(0, rect.h);
                            println!("ID: {:?}, POS: {:?}, RECT: {:?}", id, entity.position, rect);
                            entity.velocity = Vector::<i32>::new(0, 0);
                        }
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
        if entity.properties.gravity {
            entity.velocity += Vector::<i32>::new(0, self.properties.gravity);
        }
        entity.bound(self.viewport);

        Ok(())
    }
}

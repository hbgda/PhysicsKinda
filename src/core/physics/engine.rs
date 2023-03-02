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
    pub fn new(viewport: Vector<u32>) -> Self {
        let mut engine = Engine {
            entities: EntityManager::new(),
            viewport,
            properties: EngineProperties { gravity: (2 * METER as i32) / TPS as i32 },
            collision: Collision::new()
        };
        let(_, floor) = engine.entities.create_entity();
        // let(_, cieling) = engine.entities.create_entity();
        // let(_, left_wall) = engine.entities.create_entity();
        // let(_, right_wall) = engine.entities.create_entity();

        floor.size.set(viewport.x(), 5);
        floor.position.set(0, (viewport.y() / 2) as i32 - 1);
        floor.properties.gravity = false;
        engine
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
    }

    pub fn update_collision(&mut self, entity_id: EntityID) -> Result<(), ()> {
        let entity_rect = self.entities.get_entity(entity_id).unwrap().to_rect(self.viewport);
        for other in &self.entities.ids() {
            if entity_id == *other {
                continue;
            }
            let other_rect = self.entities.get_entity(*other).unwrap().to_rect(self.viewport);
            if let Some(intersection) = entity_rect.intersection(other_rect) {
                dbg!(intersection);
                let entity_mut = self.entities.get_entity_mut(entity_id).unwrap();
                let mut clip = Vector::<i32>::new(0, 0);
                if intersection.h < intersection.w {
                    if intersection.h <= other_rect.height() as i32 / 2 {
                        clip.set(0, -intersection.h);
                    }
                    else {
                        clip.set(0, intersection.h);
                    }
                    entity_mut.velocity.set(entity_mut.velocity.x(), 0);
                } 
                entity_mut.position += clip;
            }
        }
        Ok(())
    }
    
    fn update_collision_1(&mut self) {
        let ids = self.entities.ids();
        for id in &ids {
            // dbg!(id.0);
            for other in &ids {
                if id != other {
                    let a = self.entities.get_entity(*id).unwrap().to_rect(self.viewport);
                    let b = self.entities.get_entity(*other).unwrap().to_rect(self.viewport);
                    let intersection = self.collision.check_collision(a, b);
                    if let Some(rect) = intersection.1 {
                        // dbg!(id.0, rect);
                        let entity = self.entities.get_entity_mut(*id).unwrap();
                        if entity.velocity.x() != 0 || entity.velocity.y() != 0 {
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
                        }
                        else {
                            let mut clip = Vector::<i32>::new(0, 0);
                            if rect.w < rect.h {
                                if rect.x < b.width() as i32 / 2 {
                                    clip = Vector::<i32>::new(-rect.w, 0); 
                                }
                                else {;
                                    clip = Vector::<i32>::new(rect.w, 0); 
                                }
                            }
                            else if rect.h < rect.w {
                                if rect.y < b.height() as i32 / 2 {
                                    clip = Vector::<i32>::new(0, -rect.y); 
                                }
                                else {;
                                    clip = Vector::<i32>::new(0, rect.y); 
                                }
                            }
                            else {
                                clip = Vector::<i32>::new(rect.w, rect.h);
                            }
                            entity.position += clip;

                        }
                        // entity.position -= Vector::<i32>::new(0, rect.h);
                        println!("ID: {:?}, POS: {:?}, RECT: {:?}", id, entity.position, rect);
                        entity.velocity = Vector::<i32>::new(0, 0);
                    }
                }
            }
        }
    }

    fn update_entity(&mut self, entity_id: EntityID) -> Result<(), String>
    {
        // Just basic currently, may eventually be updated to include various physical elements
        // such as air resistance, gravity, acceleration etc
        
        let maybe_entity = self.entities.get_entity_mut(entity_id);
        if let None = maybe_entity {
            return Err(format!("Unknown entity of id {:?}", &entity_id))
        }

        let entity = maybe_entity.unwrap();
        entity.position += entity.velocity;

        if entity.properties.gravity {
            entity.velocity += Vector::<i32>::new(0, self.properties.gravity);
        }
        // entity.bound(self.viewport);

        self.update_entity_collision(entity_id);
        Ok(())
    }

    fn update_entity_collision(&mut self, entity_id: EntityID) {
        let entity_rect = self.entities.get_entity(entity_id).unwrap().to_rect(self.viewport);
        for other in self.entities.ids() {
            if entity_id == other {
                continue;
            }
            let other_rect = self.entities.get_entity(other).unwrap().to_rect(self.viewport);
            if let Some(intersect) = entity_rect.intersection(other_rect) {
                dbg!(intersect);
                let entity = self.entities.get_entity_mut(entity_id).unwrap();
                let mut adjust = Vector::<i32>::new(0, 0);
                let mut adjust_vel = entity.velocity;
                if entity.velocity.x() > 0 {
                    adjust.set(intersect.w, 0);
                    adjust_vel.set(0, adjust_vel.y());
                }
                else if entity.velocity.x() < 0 {
                    adjust.set(-intersect.w, 0);
                    adjust_vel.set(0, adjust_vel.y());
                }
                if entity.velocity.y() > 0 {
                    adjust.set(adjust.x(), intersect.h);
                    adjust_vel.set(adjust_vel.x(), 0);
                }
                else if entity.velocity.y() < 0 {
                    adjust.set(adjust.x(), -intersect.h);
                    adjust_vel.set(adjust_vel.x(), 0);
                }
                entity.position -= adjust;
                entity.velocity = adjust_vel;
            }
        }
    }
}

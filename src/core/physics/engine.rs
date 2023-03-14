use sdl2::rect::Rect;

use super::{entity_manager::{EntityManager, EntityID, self}, properties::engine_props::EngineProperties, units::{METER, TPS}, entity::PhysicsEntity, collision::{self, line::{Point, Line, line_intersect}, rect::{collide_left, collide_top}}};
use super::vector::Vector;
use crate::core::rendering::ext::entity::PhysicsEntityExt;


pub struct Engine {
    viewport: Vector<u32>,
    pub entities: EntityManager,
    pub properties: EngineProperties,
}

impl Engine {
    pub fn new(viewport: Vector<u32>) -> Self {
        Engine {
            entities: EntityManager::new(),
            viewport,
            properties: EngineProperties::new(
                Vector::<i32>::new(0, 2)
            ),
        }
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.viewport.set(width, height);
    }
}

impl Engine {
    pub fn update(&mut self) {
        let ids = self.entities.ids();
        for id in &ids {
            let _ = self.update_entity(*id);
        }
    }

    fn update_entity(&mut self, entity_id: EntityID) -> Result<(), String>
    {
        // Just basic currently, may eventually be updated to include various physical elements
        // such as air resistance, gravity, acceleration etc
        
        // let maybe_entity = self.entities.get_entity_mut(entity_id);
        // if let None = maybe_entity {
        //     return Err(format!("Unknown entity of id {:?}", &entity_id))
        // }

        // let entity = maybe_entity.unwrap();
        // entity.position += entity.velocity;

        // entity.bound(self.viewport);

        // self.update_entity_collision(entity_id);
        self.apply_physics(entity_id);
        
        // let entity = self.entities.get_entity(entity_id).unwrap();
        // let mut rect = entity.to_rect(self.viewport);
        // let step = entity.velocity / Vector::<i32>::new(COLLISION_TICKS as i32, COLLISION_TICKS as i32);
        // // println!("{:?}", step);
        // for tick in 0..COLLISION_TICKS {
        //     let collisions = self.get_collisions(rect);
        //     if collisions.iter().filter(|id| **id != entity_id).count() > 0 {
        //         // println!("ID: {:?} Tick {tick} {:?}", entity_id, collisions);
        //         self.collide_entities(entity_id, collisions);
        //         break;
        //     }
        //     rect.x += step.x();
        //     rect.y += step.y();
        //     self.entities.get_entity_mut(entity_id).unwrap().position += step;
        // }   

        self.update_position(entity_id);

        // self.entities.get_entity_mut(entity_id).unwrap().bound(self.viewport);
        Ok(())
    }

    fn update_position(&mut self, entity_id: EntityID) {
        self.handle_collisions(entity_id);
    }

    fn handle_collisions(&mut self, entity_id: EntityID) -> (i32, i32) {
        let entity = self.entities.get_entity(entity_id).unwrap();
        if entity.material.gravity == false {
            return (0, 0);
        }
        let entity_rect = entity.to_rect(Vector::new(0, 0));

        let mut potential_collisions = self.entities.all();
        potential_collisions.sort_unstable_by(|a, b| 
            Vector::distance(Vector::new(entity.position.x(), entity.position.y()), Vector::new(a.position.x(), a.position.y())).partial_cmp(
                &Vector::distance(Vector::new(entity.position.x(), entity.position.y()), Vector::new(b.position.x(), b.position.y()))
            ).unwrap()
        );


        let mut entity_pos = entity.position + entity.velocity;
        let mut entity_vel = entity.velocity;
        for other in &potential_collisions[1..] {
            let other_rect = other.to_rect((0,0).into());
            
            // dbg!(other.position.y(), other_rect.y);
            
            if entity.velocity.y() > 0 {
                let path = Line(
                    Point { x: entity_rect.x + entity.size.x() as i32 / 2, 
                            y: entity_rect.y + entity.size.y() as i32 },
                    Point { x: entity_rect.x + entity.size.x() as i32 / 2 + entity.velocity.x(),
                            y: entity_rect.y + entity.size.y() as i32 + entity.velocity.y() }
                );
                if let Some(_) = collide_top(other_rect, path) {
                    entity_pos.set(
                        entity_pos.x(), 
                        other.position.y() - other.size.y() as i32 / 2
                        - entity.size.y() as i32 / 2
                    );
                    entity_vel.set(entity_vel.x(), 0);
                }
            }
            if entity.velocity.x() > 0 {

                let path = Line(
                    Point { x: entity_rect.x + entity.size.x() as i32 / 2, 
                            y: entity_rect.y },
                    Point { x: entity_rect.x + entity.size.x() as i32 / 2 + entity.velocity.x(),
                            y: entity_rect.y + entity.velocity.y() }
                );
                if let Some(_) = collide_left(other_rect, path) {
                    entity_pos.set(
                        other.position.x() - other.size.x() as i32 / 2
                        - entity.size.x() as i32 / 2,
                        entity_pos.y() 
                    );
                    entity_vel.set(0, entity_vel.y());
                }
            }
        }   
        
        let entity = self.entities.get_entity_mut(entity_id).unwrap();
        entity.position = entity_pos;
        entity.velocity = entity_vel;
        (0, 0)
    }

    fn apply_physics(&mut self, entity_id: EntityID) {
        let entity = self.entities.get_entity_mut(entity_id).unwrap();

        if entity.material.gravity {
            entity.velocity += self.properties.gravity;
        }
    }

    fn get_collisions(&self, rect: Rect) -> Vec<EntityID> {
        self.entities.iter().filter_map(|(id, entity)| {
            if rect.has_intersection(entity.to_rect(self.viewport)) {
                return Some(*id);
            }
            None
        }).collect()
    }

    fn collide_entities(&mut self, entity_id: EntityID, others: Vec<EntityID>) {
        let entity_rect = self.entities.get_entity(entity_id).unwrap().to_rect(self.viewport);
        for other in others {
            if entity_id == other {
                continue;
            }
            let other_rect = self.entities.get_entity(other).unwrap().to_rect(self.viewport);
            if let Some(intersect) = entity_rect.intersection(other_rect) {
                // dbg!(intersect);
                let entity = self.entities.get_entity_mut(entity_id).unwrap();
                let mut adjust = Vector::<i32>::new(0, 0);
                let mut adjust_vel = entity.velocity;
                if entity.velocity.y() > 0 && other_rect.y > entity_rect.y {
                    adjust.set(adjust.x(), intersect.h);
                    let inverse_weighted = adjust_vel.y() as f32 * (entity.material.bounce as f32 / 255.0);
                    adjust_vel.set(adjust_vel.x(), -inverse_weighted as i32);
                }
                else if entity.velocity.y() < 0 && other_rect.y < entity_rect.y {
                    adjust.set(adjust.x(), -intersect.h);
                    adjust_vel.set(adjust_vel.x(), 0);
                }
                if entity.velocity.x() > 0 && other_rect.x > entity_rect.x {
                    adjust.set(intersect.w, 0);
                    adjust_vel.set(0, adjust_vel.y());
                }
                else if entity.velocity.x() < 0 && other_rect.x < entity_rect.x {
                    adjust.set(-intersect.w, 0);
                    adjust_vel.set(0, adjust_vel.y());
                }
                entity.position -= adjust;
                entity.velocity = adjust_vel;
            }
        }

    }
}

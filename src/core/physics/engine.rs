use sdl2::rect::Rect;

use super::{entity_manager::{EntityManager, EntityID, self}, properties::engine_props::EngineProperties, units::{METER, TPS}, collision::Collision, entity::PhysicsEntity};
use super::vector::Vector;
use crate::core::rendering::ext::entity::PhysicsEntityExt;

const COLLISION_TICKS: u8 = 4;

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
        floor.size.set(viewport.x(), 2);
        floor.position.set(0, (viewport.y() / 2) as i32 - 1);
        floor.material.gravity = false;

        // let(_, cieling) = engine.entities.create_entity();
        // cieling.size.set(viewport.x(), 2);
        // cieling.position.set(0, -(viewport.y() as i32 / 2) + 1);
        // cieling.material.gravity = false;

        let (_, left_wall) = engine.entities.create_entity();
        left_wall.size.set(2, viewport.y());
        left_wall.position.set(-(viewport.x() as i32 / 2) + 1, 0);
        left_wall.material.gravity = false;

        let (_, right_wall) = engine.entities.create_entity();
        right_wall.size.set(2, viewport.y());
        right_wall.position.set((viewport.x() as i32 / 2) - 1, 0);
        right_wall.material.gravity = false;


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
        
        let entity = self.entities.get_entity(entity_id).unwrap();
        let mut rect = entity.to_rect(self.viewport);
        let step = entity.velocity / Vector::<i32>::new(COLLISION_TICKS as i32, COLLISION_TICKS as i32);
        // println!("{:?}", step);
        for tick in 0..COLLISION_TICKS {
            let collisions = self.get_collisions(rect);
            if collisions.iter().filter(|id| **id != entity_id).count() > 0 {
                // println!("ID: {:?} Tick {tick} {:?}", entity_id, collisions);
                self.collide_entities(entity_id, collisions);
                break;
            }
            rect.x += step.x();
            rect.y += step.y();
            self.entities.get_entity_mut(entity_id).unwrap().position += step;
        }

        // self.entities.get_entity_mut(entity_id).unwrap().bound(self.viewport);
        Ok(())
    }

    fn apply_physics(&mut self, entity_id: EntityID) {
        let entity = self.entities.get_entity_mut(entity_id).unwrap();

        if entity.material.gravity {
            entity.velocity += Vector::<i32>::new(0, self.properties.gravity);
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

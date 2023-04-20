use sdl2::rect::Rect;

use super::{entity_manager::{EntityManager, EntityID, self}, properties::engine_props::EngineProperties, units::{METER, TPS}, entity::PhysicsEntity, collision::{self, line::{Point, Line, line_intersect, self}, rect::{collide_left, collide_top}}};
use super::vector::Vector;
use crate::core::rendering::ext::entity::PhysicsEntityExt;


pub struct Engine {
    viewport: Vector<u32>,
    viewport_simulation_offset: Vector<u32>,
    pub entities: EntityManager,
    pub properties: EngineProperties,
}

impl Engine {
    pub fn new(viewport: Vector<u32>) -> Self {
        Engine {
            viewport,
            viewport_simulation_offset: (0, 0).into(),
            entities: EntityManager::new(),
            properties: EngineProperties::new(
                Vector::<i32>::new(0, 2)
            ),
        }
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.viewport.set(width, height);
    }
    
    pub fn set_simulation_boundary(&mut self, x_offset: u32, y_offset: u32) {
        self.viewport_simulation_offset.set(x_offset, y_offset);
    }

    pub fn simulated_viewport(&self) -> Vector<u32> {
        (
            self.viewport.x() + self.viewport_simulation_offset.x() * 2,
            self.viewport.y() + self.viewport_simulation_offset.y() * 2
        ).into()
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
         self.apply_physics(entity_id);
        
        self.update_position(entity_id);

        Ok(())
    }
    
    fn is_entity_lost(&self, entity_id: EntityID) -> bool {
        let entity = self.entities.get_entity(entity_id).unwrap();
        let simulated_viewport = self.simulated_viewport();

        entity.position.x() >= simulated_viewport.x() as i32 / 2      ||
        entity.position.x() <= -(simulated_viewport.x() as i32) / 2   ||
        entity.position.y() >= simulated_viewport.y() as i32 / 2      ||
        entity.position.y() <= -(simulated_viewport.y() as i32) / 2    
    }

    fn update_position(&mut self, entity_id: EntityID) {
        if self.is_entity_lost(entity_id) {
            self.entities.destroy(entity_id);
            return;
        }
        if let Some((pos, vel)) = self.handle_collisions(entity_id) {
           let mut entity = self.entities.get_entity_mut(entity_id).unwrap();
           entity.position = pos;
           entity.velocity = vel;
        }
    }
                                                                    // Position  // Velocity
    fn handle_collisions(&mut self, entity_id: EntityID) -> Option<(Vector<i32>, Vector<i32>)> {
        let entity = self.entities.get_entity(entity_id).unwrap();
        if entity.velocity == (0, 0).into() {
            return None;
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
            let other_rect = other.to_rect((0, 0).into());
            if let Some((left, right)) = entity.make_path() {
                if line::lines_contain_line(
                    left, right, 
                    Line( Point { x: other_rect.x, y: other_rect.y }, Point { x: other_rect.x + other_rect.width() as i32, y: other_rect.y } )
                ) {
                    entity_pos.set(entity_pos.x(), other_rect.y - entity.size.y() as i32 / 2 );
                    entity_vel.set(entity_vel.x(), 0);
                }
            }
        }   
        Some((entity_pos, entity_vel))
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

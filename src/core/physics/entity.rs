// use struct_extension::extendable;
use super::vector::Vector;
use super::properties::entity_props::MaterialProperties;
use crate::core::physics::collision::line::{Line, Point};

// #[extendable]
pub struct PhysicsEntity {
    pub position: Vector<i32>,
    pub velocity: Vector<i32>,
    pub size: Vector<u32>,
    pub material: MaterialProperties
}

impl PhysicsEntity {
    pub fn new(position: Vector<i32>, velocity: Vector<i32>, size: Vector<u32>) -> Self {
        PhysicsEntity { position, velocity, size, material: MaterialProperties::new() }
    }
}

impl Default for PhysicsEntity {
    fn default() -> Self {
        PhysicsEntity::new (
            Vector::<i32>::new(0, 0),
            Vector::<i32>::new(0, 0), 
            Vector::<u32>::new(0, 0)
        )
    }
}

impl PhysicsEntity {
    pub fn make_path(&self) -> Option<(Line, Line)> {
        let left = self.position.x() - self.size.x() as i32 / 2;
        let right = self.position.x() + self.size.x() as i32 / 2;
        let top = self.position.y() + self.size.y() as i32 / 2;
        let bottom = self.position.y() - self.size.y() as i32 / 2;

        let top_left = Line( Point { x: left, y: top }, Point { x: left + self.velocity.x(), y: top + self.velocity.y() } );
        let top_right = Line( Point { x: right, y: top }, Point { x: right + self.velocity.x(), y: top + self.velocity.y() } ); 
        let bottom_left = Line( Point { x: left, y: bottom }, Point { x: left + self.velocity.x(), y: bottom + self.velocity.y() } );
        let bottom_right = Line( Point { x: right, y: bottom }, Point { x: right + self.velocity.x(), y: bottom + self.velocity.y() } );


        if self.velocity.x() > 0 && self.velocity.y() > 0 {
            return Some((bottom_right, top_left)) 
        }
        if self.velocity.x() > 0 && self.velocity.y() < 0 {
            return Some((bottom_left, top_right))
        }
        if self.velocity.x() < 0 && self.velocity.y() > 0 {
            return Some((bottom_left, top_right))
        }
        if self.velocity.x() < 0 && self.velocity.y() < 0 {
            return Some((bottom_right, top_left))
        }
        if self.velocity.x() > 0 {
            return Some((top_right, bottom_right))
        }
        if self.velocity.x() < 0 {
            return Some((top_left, bottom_left))
        }
        if self.velocity.y() > 0 {
            return Some((top_left, top_right))
        }
        if self.velocity.y() < 0 {
            return Some((bottom_left, bottom_right))
        }
        return None
    }
    pub fn bound(&mut self, viewport: Vector<u32>) {
        let (viewport_x, viewport_y) = (
            viewport.x() as i32 - (viewport.x() / 2) as i32,
            viewport.y() as i32 - (viewport.y() / 2) as i32
        );

        let (width, height) = (
            (self.size.x() / 2) as i32,
            (self.size.y() / 2) as i32
        );
        
        let max_x = viewport_x - width;
        let max_y = viewport_y - height;
        let min_x = - viewport_x + width;
        let min_y = - viewport_y + height;

        if self.position.x() > max_x {
            self.position.set(max_x, self.position.y());
            // self.velocity.set(0, self.velocity.y());
        }
        else if self.position.x() < min_x {
            self.position.set(min_x, self.position.y());
            // self.velocity.set(0, self.velocity.y());
        }
        
        if self.position.y() > max_y {
            self.position.set(self.position.x(), max_y);
            // self.velocity.set(self.velocity.x(), 0);
        }
        else if self.position.y() < min_y {
            self.position.set(self.position.x(), min_y);
            // self.velocity.set(self.velocity.x(), 0);
        }
    }
}

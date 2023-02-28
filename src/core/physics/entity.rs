// use struct_extension::extendable;
use super::vector::Vector;
use super::properties::entity_props::EntityProperties;

// #[extendable]
pub struct PhysicsEntity {
    pub position: Vector<i32>,
    pub velocity: Vector<i32>,
    pub size: Vector<u32>,
    pub properties: EntityProperties
}

impl PhysicsEntity {
    pub fn new(position: Vector<i32>, velocity: Vector<i32>, size: Vector<u32>) -> Self {
        PhysicsEntity { position, velocity, size, properties: EntityProperties::new() }
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
            self.velocity.set(0, self.velocity.y());
        }
        else if self.position.x() < min_x {
            self.position.set(min_x, self.position.y());
            self.velocity.set(0, self.velocity.y());
        }
        
        if self.position.y() > max_y {
            self.position.set(self.position.x(), max_y);
            self.velocity.set(self.velocity.x(), 0);
        }
        else if self.position.y() < min_y {
            self.position.set(self.position.x(), min_y);
            self.velocity.set(self.velocity.x(), 0);
        }
    }
}

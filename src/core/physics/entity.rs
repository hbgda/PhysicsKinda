// use struct_extension::extendable;
use super::vector::Vector;

pub type ScreenPosition = Vector<u32>;
pub type WorldPosition = Vector<i32>;

impl WorldPosition {
    pub fn screen_space(&self, viewport: Vector<u32>) -> WorldPosition {
        WorldPosition::new(0, y)    
    }
}

// #[extendable]
pub struct PhysicsEntity {
    pub position: WorldPosition,
    pub velocity: Vector<i32>,
    pub size: Vector<u32>
}

impl PhysicsEntity {
    pub fn new(position: Vector<i32>, velocity: Vector<i32>, size: Vector<u32>) -> Self {
        PhysicsEntity { position, velocity, size }
    }

    pub fn empty() -> Self {
        PhysicsEntity::new (
            Vector::<i32>::new(0, 0),
            Vector::<i32>::new(0, 0), 
            Vector::<u32>::new(0, 0)
        )
    }
}

impl PhysicsEntity {
    pub fn bound(&mut self, viewport: Vector<u32>) {
        let max_x = viewport.x() as i32 - self.position.x() / 2;
    }
}

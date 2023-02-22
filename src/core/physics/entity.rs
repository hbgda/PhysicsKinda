// use struct_extension::extendable;
use super::vector::Vector;

// #[extendable]
pub struct PhysicsEntity {
    pub position: Vector,
    pub velocity: Vector,
    pub size: Vector
}

impl PhysicsEntity {
    pub fn new(position: Vector, velocity: Vector, size: Vector) -> Self {
        PhysicsEntity { position, velocity, size }
    }

    pub fn empty() -> Self {
        PhysicsEntity::new (
            Vector::new(0, 0),
            Vector::new(0, 0), 
            Vector::new(0, 0)
        )
    }
}

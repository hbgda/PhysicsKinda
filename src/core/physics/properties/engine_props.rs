use crate::core::physics::{vector::Vector, units::{TPS, METER}};

pub struct EngineProperties {
    pub gravity: Vector<i32>
}

impl EngineProperties {
    pub fn new(gravity: Vector::<i32>, ) -> Self {
        EngineProperties { 
            gravity: Vector::<i32>::new(
                (gravity.x() * METER as i32) / TPS as i32,
                (gravity.y() * METER as i32) / TPS as i32
            ) 
        }
    }
}
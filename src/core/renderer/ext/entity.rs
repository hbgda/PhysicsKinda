use sdl2::rect::Rect;

use crate::core::physics::{entity::PhysicsEntity, vector::Vector};

pub trait PhysicsEntityExt {
    fn to_rect(&self, viewport: Vector<u32>) -> Rect;
}

impl PhysicsEntityExt for PhysicsEntity {
    fn to_rect(&self, viewport: Vector<u32>) -> Rect {
        dbg!(Rect::new(
            self.position.x() + viewport.x() as i32 / 2 - self.size.x() as i32 / 2, 
            self.position.y() + viewport.y() as i32 / 2 - self.size.y() as i32 / 2, 
            self.size.x(), self.size.y()
        ))
    }
}

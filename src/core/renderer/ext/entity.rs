use sdl2::rect::Rect;

use crate::core::physics::entity::PhysicsEntity;

impl Into<Rect> for &PhysicsEntity {
    fn into(self) -> Rect {
        Rect::new(
            self.position.x() as i32,
            self.position.y() as i32,
            self.size.x() as u32,
            self.size.y() as u32
        )
    }
}
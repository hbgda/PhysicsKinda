pub mod core;

pub mod entity {
    pub use macros::*;
    pub trait EntityType {
        fn get_position(&self) -> (u32, u32);
        fn set_position(&mut self, pos: (u32, u32));
    }
}
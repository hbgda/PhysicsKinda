use crate::core::physics::vector::Vector;

pub struct DebugInfo {
    mouse_pos: Vector<u32>,
    entities: usize,
    fps: u16,
}

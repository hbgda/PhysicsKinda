use super::entity::*;

pub struct Engine<'e> {
    _entities: Vec<&'e dyn PhysicsEntity>
}

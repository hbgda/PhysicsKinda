pub enum PhysicsProperty {
    Static(bool),
}

pub struct EntityProperties {
    pub gravity: bool,
    pub mass: u32, 
    pub extra: Vec<PhysicsProperty>
}

impl EntityProperties {
    pub fn new() -> Self {
        EntityProperties { gravity: true, mass: 0, extra: Vec::new() }
    }
}
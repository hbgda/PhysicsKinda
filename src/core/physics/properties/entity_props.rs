pub enum PhysicsProperty {
    Static(bool),
}

pub struct MaterialProperties {
    pub gravity: bool,
    pub bounce: u8,
    pub mass: u32, 
    pub extra: Vec<PhysicsProperty>
}

impl MaterialProperties {
    pub fn new() -> Self {
        MaterialProperties { gravity: true, bounce: 255, mass: 0, extra: Vec::new() }
    }
}

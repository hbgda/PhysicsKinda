#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector((u32, u32));

impl Vector {
    pub fn new(x: u32, y: u32) -> Self {
        Vector((x, y))
    }

    pub fn x(&self) -> u32 {
        self.0.0
    }

    pub fn y(&self) -> u32 {
        self.0.1
    }

    pub fn set(&mut self, x: u32, y: u32) {
        self.0 = (x, y)
    }
}

impl From<(u32, u32)> for Vector {
    fn from(value: (u32, u32)) -> Self {
        Vector::new(value.0, value.1)
    }
}

impl std::ops::Add<Self> for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector ((
            self.x() + rhs.x(),
            self.y() + rhs.y()
        ))
    }
}

impl std::ops::AddAssign<Self> for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.set(
            self.x() + rhs.x(),
            self.y() + rhs.y()
        )
    }
}

impl std::ops::Sub<Self> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector ((
            self.x() - rhs.x(),
            self.y() - rhs.y()
        ))
    }
}

impl std::ops::SubAssign<Self> for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.set(
            self.x() - rhs.x(),
            self.y() - rhs.y()
        )
    }
}
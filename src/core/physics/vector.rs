#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector<T>((T, T))
where 
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy;

impl<T> Vector<T> 
where 
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + Copy
{
    pub fn new(x: T, y: T) -> Self {
        Vector((x, y))
    }
pub fn x(&self) -> T { self.0.0
    }

    pub fn y(&self) -> T {
        self.0.1
    }

    pub fn set(&mut self, x: T, y: T) {
        self.0 = (x, y)
    }

}

impl<T> std::ops::Add<Self> for Vector<T>
where 
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector ((
            self.x() + rhs.x(),
            self.y() + rhs.y()
        ))
    }
}

impl<T> std::ops::AddAssign<Self> for Vector<T> 
where 
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy
{
    fn add_assign(&mut self, rhs: Self) {
        self.set(
            self.x() + rhs.x(),
            self.y() + rhs.y()
        )
    }
}

impl<T> std::ops::Sub<Self> for Vector<T> 
where 
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector ((
            self.x() - rhs.x(),
            self.y() - rhs.y()
        ))
    }
}

impl<T> std::ops::SubAssign<Self> for Vector<T> 
where 
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy
{
    fn sub_assign(&mut self, rhs: Self) {
        self.set(
            self.x() - rhs.x(),
            self.y() - rhs.y()
        )
    }
}

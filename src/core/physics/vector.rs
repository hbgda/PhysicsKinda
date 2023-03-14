pub trait VectorType: std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Div<Output = Self> + std::ops::Mul<Output = Self> + Copy { }

impl<T> VectorType for T where T: std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Div<Output = Self> + std::ops::Mul<Output = Self> + Copy {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector<T: VectorType>((T, T));

impl<T: VectorType> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector((x, y))
    }

    pub fn x(&self) -> T {
        self.0.0
    }

    pub fn y(&self) -> T {
        self.0.1
    }

    pub fn set(&mut self, x: T, y: T) {
        self.0 = (x, y)
    }

}

impl Vector<i32> {
    pub fn distance(a: Vector<i32>, b: Vector<i32>) -> f32 {
        f32::sqrt(
           ((a.x() - b.x()) as f32).powf(2.0) +
           ((a.y() - b.y()) as f32).powf(2.0)  
        ).abs()
    }
}

impl <T: VectorType> Into<Vector<T>> for (T, T) {
    fn into(self) -> Vector<T> {
        Vector((self.0, self.1))
    }
}

impl<T: VectorType> std::ops::Add<Self> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector((
            self.x() + rhs.x(), 
            self.y() + rhs.y())
        )
    }
}

impl<T: VectorType> std::ops::AddAssign<Self> for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        let (x, y) = (*self + rhs).0;
        self.set(x, y);
    }
}

impl<T: VectorType> std::ops::Sub<Self> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector((
            self.x() - rhs.x(), 
            self.y() - rhs.y())
        )
    }
}

impl<T: VectorType> std::ops::SubAssign<Self> for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        let (x, y) = (*self - rhs).0;
        self.set(x, y);
    }
}

impl<T: VectorType> std::ops::Div<Self> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Vector ((
            self.x() / rhs.x(),
            self.y() / rhs.y()
        ))
    }
}

impl<T: VectorType> std::ops::DivAssign<Self> for Vector<T> {
    fn div_assign(&mut self, rhs: Self) {
        let (x, y) = (*self / rhs).0;
        self.set(x, y);
    }
}

impl<T: VectorType> std::ops::Mul<Self> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector((
            self.x() * rhs.x(),
            self.y() * rhs.y()
        ))
    }
}

impl<T: VectorType> std::ops::MulAssign<Self> for Vector<T> {
    fn mul_assign(&mut self, rhs: Self) {
        let (x, y) = (*self * rhs).0;
        self.set(x, y);
    }
}

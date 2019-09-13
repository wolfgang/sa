use std::ops::{AddAssign, Mul};

pub type Vector2f = Vector2<f32>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector2<T> where T: Copy {
    pub(crate) x: T,
    pub(crate) y: T,
}


impl<T> Vector2<T> where T: Copy {
    pub fn with(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Mul<T> for Vector2<T> where T: Copy + Mul<Output=T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::with(self.x * rhs, self.y * rhs)
    }
}

impl<T> Mul<Vector2<T>> for Vector2<T> where T: Copy + Mul<Output=T> {
    type Output = Self;

    fn mul(self, rhs: Vector2<T>) -> Self::Output {
        Vector2::with(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> AddAssign<Vector2<T>> for Vector2<T> where T: Copy + AddAssign {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}



impl<T> From<(T, T)> for Vector2<T> where T: Copy {
    fn from(t: (T, T)) -> Self {
        Self::with(t.0, t.1)
    }
}


impl<T> Into<(T, T)> for Vector2<T> where T: Copy {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

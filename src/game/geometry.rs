use std::ops::{Mul, MulAssign};

pub type Vector2f = Vector2<f32>;
pub type Vector2F = Vector2<f64>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector2<T> where T: Copy {
    pub(crate) x: T,
    pub(crate) y: T,
}


impl<T> Vector2<T> where T: Copy {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn scale<U>(&mut self, other: &Vector2<U>) where T: MulAssign + From<U>, U: MulAssign + Copy {
        self.x *= other.x.into();
        self.y *= other.y.into();
    }
}

impl<T> Mul<T> for Vector2<T> where T: Copy + Mul<Output=T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> From<(T, T)> for Vector2<T> where T: Copy {
    fn from(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl<T> Into<(T, T)> for Vector2<T> where T: Copy {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

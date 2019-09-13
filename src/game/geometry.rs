use std::ops::MulAssign;

#[derive(Copy, Clone)]
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

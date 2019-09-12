#[derive(Copy, Clone)]
pub struct Vector2<T> where T: Copy {
    x: T,
    y: T,
}

impl<T> Vector2<T> where T: Copy {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
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

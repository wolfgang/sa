use std::ops::{AddAssign, Mul};

pub type Vector2f = Vector2<f32>;

#[derive(Copy, Clone, Default)]
pub struct Rectanglef {
    pub position: Vector2f,
    pub width: f32,
    pub height: f32,
}

impl Rectanglef {
    pub fn with(position: Vector2f, width: f32, height: f32) -> Self {
        Self { position, width, height }
    }

    pub fn with_tuple(position: Vector2f, dimensions: (u32, u32)) -> Self {
        Self::with(position, dimensions.0 as f32, dimensions.1 as f32)
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        if self.right() < other.position.x { return false; }
        if self.position.x > other.right() { return false; }
        if self.bottom() < other.position.y { return false; }
        if self.position.y > other.bottom() { return false; }
        true
    }

    fn right(&self) -> f32 {
        self.position.x + self.width - 1.0
    }

    fn bottom(&self) -> f32 {
        self.position.y + self.height - 1.0
    }

    pub fn is_inside_of(&self, other: &Self) -> bool {
        self.position.x >= other.position.x
            && self.right() <= other.right()
            && self.position.y >= other.position.y
            && self.bottom() <= other.bottom()
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vector2<T> where T: Copy {
    pub(crate) x: T,
    pub(crate) y: T,
}


impl<T> Vector2<T> where T: Copy {
    pub fn with(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self where T: Default {
        Self::default()
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

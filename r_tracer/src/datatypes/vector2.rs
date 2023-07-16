use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f64) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

impl Div<f64> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: f64) -> Vector2 {
        Vector2::new(self.x / scalar, self.y / scalar)
    }
}

impl Mul<Vector2> for f64 {
    type Output = Vector2;

    fn mul(self, vector: Vector2) -> Vector2 {
        Vector2::new(self * vector.x, self * vector.y)
    }
}

impl Div<Vector2> for f64 {
    type Output = Vector2;

    fn div(self, vector: Vector2) -> Vector2 {
        Vector2::new(self / vector.x, self / vector.y)
    }
}

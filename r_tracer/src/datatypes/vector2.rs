
#[derive(Copy, Clone)]
pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x: x, y: y}
    }
}

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x: x, y: y, z: z}
    }

    pub fn self_dot(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn component_add(self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn square(self) -> Vector3 {
        Vector3::new(self.x * self.x, self.y * self.y, self.z * self.z)
    }

    pub fn distance(self, other: Vector3) -> f64 {
        (self - other).square().component_add().sqrt()
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = f64;

    fn mul(self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self * other.x, self * other.y, self * other.z)
    }
}
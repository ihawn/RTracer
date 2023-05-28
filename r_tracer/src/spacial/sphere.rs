use crate::datatypes::vector3::Vector3;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, r: f64) -> Sphere {
        Sphere {
            center: Vector3::new(x, y, z),
            radius: r
        }
    }
}
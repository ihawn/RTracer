use crate::datatypes::vector3::Vector3;
use crate::datatypes::color::Color;
use crate::datatypes::material::{Material, self};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Material,
    pub id: i32
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, r: f64, material: Material, id: i32) -> Sphere {
        Sphere {
            center: Vector3::new(x, y, z),
            radius: r,
            material: material,
            id: id
        }
    }

    pub fn empty() -> Sphere {
        Sphere {
            center: Vector3::zero(),
            radius: 0.0,
            material: Material::empty(),
            id: -1
        }
    }
}
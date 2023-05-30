use crate::datatypes::vector3::Vector3;
use crate::spacial::sphere::Sphere;
use crate::spacial::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitPoint {
    pub point: Vector3,
    pub hitting_ray: Ray,
    pub normal: Vector3,
    pub object: Sphere,
    pub is_empty: bool
}

impl HitPoint {
    pub fn new(point: Vector3, ray: Ray, object: Sphere) -> HitPoint {
        HitPoint {
            point: point,
            hitting_ray: ray,
            normal: (point - object.center),
            object: object,
            is_empty: false
        }
    }

    pub fn empty() -> HitPoint {
        HitPoint {
            point: Vector3::zero(),
            hitting_ray: Ray::empty(),
            normal: Vector3::zero(),
            object: Sphere::empty(),
            is_empty: true
        }
    }
}
use crate::datatypes::vector3::Vector3;
use crate::spacial::tri::Tri;
use crate::spacial::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitPoint {
    pub point: Vector3,
    pub hitting_ray: Ray,
    pub normal: Vector3,
    pub barycentric_coords: Vector3,
    pub object: Tri,
    pub is_empty: bool,
    pub is_front_face: bool
}

impl HitPoint {
    pub fn new_from_tri(point: Vector3, ray: &Ray, object: &Tri, outward_normal: Vector3) -> HitPoint {
        let is_front = ray.direction * outward_normal < 0.0;
        HitPoint {
            point: point,
            hitting_ray: *ray,
            normal: if is_front { outward_normal } else { -1.0 * outward_normal },
            barycentric_coords: object.compute_barycentric_coords(point),
            object: *object,
            is_empty: false,
            is_front_face: is_front
        }
    }

    pub fn empty() -> HitPoint {
        HitPoint {
            point: Vector3::zero(),
            hitting_ray: Ray::empty(),
            normal: Vector3::zero(),
            barycentric_coords: Vector3::zero(),
            object: Tri::empty(),
            is_empty: true,
            is_front_face: false
        }
    }
}
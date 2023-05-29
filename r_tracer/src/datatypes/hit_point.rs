use crate::datatypes::vector3::Vector3;
use crate::spacial::sphere::Sphere;

#[derive(Copy, Clone)]
pub struct HitPoint {
    pub point: Vector3,
    pub camera_origion: Vector3,
    pub projection_point: Vector3,
    pub object: Sphere
}

impl HitPoint {
    pub fn new(point: Vector3, cam_origin: Vector3, proj_point: Vector3, object: Sphere) -> HitPoint {
        HitPoint {
            point: point,
            camera_origion: cam_origin,
            projection_point: proj_point,
            object: object
        }
    }
}
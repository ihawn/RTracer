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
            normal: (point - object.center).normalize(),
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

    pub fn closest_front_hit_point(hit_points: Vec<HitPoint>) -> HitPoint {
        let mut min_dist: f64 = hit_points[0].point.distance(hit_points[0].hitting_ray.origin);
        let mut min_i: usize = 0;
        for i in 1..hit_points.len() {
            let dist = hit_points[i].point.distance(hit_points[1].hitting_ray.origin);
            if dist < min_dist
            && (hit_points[i].point - hit_points[i].hitting_ray.origin) * hit_points[i].hitting_ray.direction > 0.0 {
                min_i = i;
                min_dist = dist;
            }
        }

        if min_i > 0 {
            return hit_points[min_i];
        } else if (hit_points[0].point - hit_points[0].hitting_ray.origin) * hit_points[0].hitting_ray.direction > 0.0 {
            return hit_points[min_i];
        } else {
            return HitPoint::empty();
        }  
    }

}
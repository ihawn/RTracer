use crate::datatypes::vector3::Vector3;
use crate::datatypes::hit_point::HitPoint;
use crate::spacial::ray::Ray;
use crate::datatypes::material::Material;

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

    pub fn ray_collision(ray: Ray, objects: &Vec<Sphere>, skip_id: i32) -> HitPoint {
        let mut hit_points: Vec<HitPoint> = Vec::new();
        for sphere in objects {
            if sphere.id == skip_id {
                continue;
            }

            let r: f64 = sphere.radius;
            let object_direction: Vector3 = ray.origin - sphere.center;

            let a: f64 = ray.direction.self_dot();
            let b: f64 = 2.0*object_direction*ray.direction;
            let c: f64 = object_direction.square().component_add() - r*r ;
            
            let desc: f64 = b*b - 4.0*a*c;

            if desc >= 0.0 {
                let desc_sqrt: f64 = desc.sqrt();
                let ax2: f64 = 2.0 * a;
                let t1: f64 = (-b + desc_sqrt) / ax2;
                let t2: f64 = (-b - desc_sqrt) / ax2;
                let pt1: Vector3 = ray.origin + t1*ray.direction;
                let pt2: Vector3 = ray.origin + t2*ray.direction;
    
                hit_points.push(
                    HitPoint::new(pt1, ray, sphere.clone())
                );
                hit_points.push(
                    HitPoint::new(pt2, ray, sphere.clone())
                );
            }
        }

        if hit_points.len() > 0 {
            HitPoint::closest_front_hit_point(hit_points)
        } else {
            HitPoint::empty()
        }
    }
}
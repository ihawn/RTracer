use crate::datatypes::vector3::Vector3;
use crate::datatypes::hit_point::HitPoint;
use crate::spacial::ray::Ray;
use crate::datatypes::material::Material;
use uuid::Uuid;

#[derive(Copy, Clone)]
pub struct Mesh {
    pub mesh_type: PrimitiveMeshType,

    pub center: Vector3,
    pub radius: f64,

    pub p1: Vector3,
    pub p2: Vector3,
    pub p3: Vector3,

    pub material: Material,
    pub id: Uuid
}

impl Mesh {
    pub fn new_sphere(x: f64, y: f64, z: f64, r: f64, material: Material) -> Mesh {
        Mesh {
            mesh_type: PrimitiveMeshType::Sphere,
            center: Vector3::new(x, y, z),
            radius: r,
            material: material,
            id: Uuid::new_v4(),

            p1: Vector3::zero(),
            p2: Vector3::zero(),
            p3: Vector3::zero(),
        }
    }

    pub fn new_triangle(p1: Vector3, p2: Vector3, p3: Vector3, material: Material) -> Mesh {
        Mesh {
            mesh_type: PrimitiveMeshType::Triangle,
            p1: p1,
            p2: p2,
            p3: p3,
            material: material,
            id: Uuid::new_v4(),

            center: Vector3::zero(),
            radius: 0.0
        }
    }

    pub fn empty() -> Mesh {
        Mesh {
            mesh_type: PrimitiveMeshType::Empty,
            center: Vector3::zero(),
            radius: 0.0,
            material: Material::empty(),
            id: Uuid::new_v4(),
            p1: Vector3::zero(),
            p2: Vector3::zero(),
            p3: Vector3::zero(),
        }
    }

    pub fn ray_collision(ray: Ray, objects: &Vec<Mesh>, skip_id: Uuid) -> HitPoint {
        let mut hit_points: Vec<HitPoint> = Vec::new();
        for mesh in objects {
            if mesh.id == skip_id {
                continue;
            }

            if mesh.mesh_type == PrimitiveMeshType::Triangle {
                hit_points = Self::intersect_triangle(&ray, mesh, hit_points);
            } else if mesh.mesh_type == PrimitiveMeshType::Sphere {
                hit_points = Self::intersect_sphere(&ray, mesh, hit_points);
            }
        }

        if hit_points.len() > 0 {
            HitPoint::closest_front_hit_point(hit_points)
        } else {
            HitPoint::empty()
        }
    }

    fn intersect_sphere(ray: &Ray, sphere: &Mesh, mut existing_hitpoints: Vec<HitPoint>) -> Vec<HitPoint> {

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

            existing_hitpoints.push(
                HitPoint::new_from_sphere(pt1, *ray, *sphere)
            );
            existing_hitpoints.push(
                HitPoint::new_from_sphere(pt2, *ray, *sphere)
            );
        }

        existing_hitpoints
    }

    fn intersect_triangle(ray: &Ray, triangle: &Mesh, mut existing_hitpoints: Vec<HitPoint>) -> Vec<HitPoint> {
        let epsilon = 1e-6;
    
        let edge1 = triangle.p2 - triangle.p1;
        let edge2 = triangle.p3 - triangle.p1;
    
        let h = ray.direction.cross(&edge2);
        let a = edge1*h;
    
        if a.abs() < epsilon {
            return existing_hitpoints;
        }
    
        let f = 1.0 / a;
        let s = ray.origin - triangle.p1;
        let u = f*s*h;
    
        if u < 0.0 || u > 1.0 {
            return existing_hitpoints;
        }
    
        let q = s.cross(&edge1);
        let v = f * ray.direction*q;
    
        if v < 0.0 || u + v > 1.0 {
            return existing_hitpoints;
        }
    
        let t = f * edge2*q;
    
        if t > epsilon {
            let point = ray.origin + t*ray.direction;
            let normal = edge1.cross(&edge2).normalize();
            let hitpoint = HitPoint {
                point,
                hitting_ray: ray.clone(),
                normal,
                object: triangle.clone(),
                is_empty: false,
            };
            existing_hitpoints.push(hitpoint);
        }
    
        existing_hitpoints
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum PrimitiveMeshType {
    Empty = 0,
    Sphere = 1,
    Triangle = 2
}
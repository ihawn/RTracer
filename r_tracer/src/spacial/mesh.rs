use crate::datatypes::vector3::Vector3;
use crate::datatypes::hit_point::HitPoint;
use crate::spacial::ray::Ray;
use crate::spacial::bvh::BVH;
use crate::datatypes::material::Material;


#[derive(Clone)]
pub struct MeshObject {
    pub tris: Vec<Mesh>,
    pub smooth_shading: bool
}

impl MeshObject {
    pub fn new(mut tris: Vec<Mesh>, smooth_shading: bool) -> MeshObject {
        for i in 0..tris.len() {
            tris[i].smooth_shading = smooth_shading;
        }
        MeshObject { tris: tris, smooth_shading }
    }
}


#[derive(Copy, Clone)]
pub struct Mesh {
    pub mesh_type: PrimitiveMeshType,

    pub center: Vector3,
    pub radius: f64,

    pub p1: Vector3,
    pub p2: Vector3,
    pub p3: Vector3,
    pub p1_normal: Vector3,
    pub p2_normal: Vector3,
    pub p3_normal: Vector3,
    pub normal: Vector3,
    pub smooth_shading: bool,

    pub bounding_box: (Vector3, Vector3),
    pub bounding_box_center: Vector3,

    pub material: Material,
    pub is_empty: bool
}

impl Mesh {
    pub fn new_sphere(x: f64, y: f64, z: f64, r: f64, material: Material) -> Mesh {
        let c = Vector3::new(x, y, z);
        let bb = Self::get_bounding_box(
            PrimitiveMeshType::Sphere, Vector3::zero(), 
            Vector3::zero(), Vector3::zero(), c, r
        );
        Mesh {
            mesh_type: PrimitiveMeshType::Sphere,
            center: c,
            radius: r,
            material: material,      
            is_empty: false,

            p1: Vector3::zero(),
            p2: Vector3::zero(),
            p3: Vector3::zero(),
            p1_normal: Vector3::zero(),
            p2_normal: Vector3::zero(),
            p3_normal: Vector3::zero(),
            normal: Vector3::zero(),
            smooth_shading: false,

            bounding_box: bb,
            bounding_box_center: Self::get_bounding_box_center(bb)
        }
    }

    pub fn new_triangle(p1: Vector3, p2: Vector3, p3: Vector3, 
        p1_normal: Vector3, p2_normal: Vector3, p3_normal: Vector3,
        normal: Vector3, material: Material) -> Mesh {
        let bb = Self::get_bounding_box(
            PrimitiveMeshType::Triangle, p1, 
            p2, p3, Vector3::zero(), 0.0
        );
        Mesh {
            mesh_type: PrimitiveMeshType::Triangle,
            p1: p1,
            p2: p2,
            p3: p3,
            p1_normal: p1_normal,
            p2_normal: p2_normal,
            p3_normal: p3_normal,
            normal: normal,
            material: material,
            is_empty: false,
            smooth_shading: false,

            center: Vector3::zero(),
            radius: 0.0,

            bounding_box: bb,
            bounding_box_center: Self::get_bounding_box_center(bb)
        }
    }

    pub fn empty() -> Mesh {
        Mesh {
            mesh_type: PrimitiveMeshType::Empty,
            center: Vector3::zero(),
            radius: 0.0,
            material: Material::empty(),
            p1: Vector3::zero(),
            p2: Vector3::zero(),
            p3: Vector3::zero(),
            p1_normal: Vector3::zero(),
            p2_normal: Vector3::zero(),
            p3_normal: Vector3::zero(),
            normal: Vector3::zero(),
            smooth_shading: false,
            is_empty: true,
            bounding_box: (Vector3::zero(), Vector3::zero()),
            bounding_box_center: Vector3::zero()
        }
    }

    pub fn get_bounding_box(mesh_type: PrimitiveMeshType, p1: Vector3, 
        p2: Vector3, p3: Vector3, center: Vector3, r: f64) -> (Vector3, Vector3) {
        if mesh_type == PrimitiveMeshType::Triangle {
            return (
                p1.min(p2.min(p3)),
                p1.max(p2.max(p3))
            )
        } else if mesh_type == PrimitiveMeshType::Sphere {
            return (
                center - Vector3::new(r, r, r),
                center + Vector3::new(r, r, r)
            )
        } else {
            return (Vector3::zero(), Vector3::zero())
        }
    }

    pub fn get_bounding_box_center(bb: (Vector3, Vector3)) -> Vector3 {
        0.5*(bb.0 + bb.1)
    }

    pub fn ray_collision(ray: Ray, bvh: &BVH, sphere_objects: &Vec<Mesh>) -> HitPoint {
        let meshes_to_check = Self::traverse_bvh_for_meshes(ray, bvh, Vec::new());
        let mut hit_points: Vec<HitPoint> = Vec::new();

        for mesh in meshes_to_check {
            hit_points = Self::intersect_triangle(&ray, &mesh, hit_points);
        }
        for mesh in sphere_objects {
            hit_points = Self::intersect_sphere(&ray, &mesh, hit_points);
        }

        if hit_points.len() > 0 {
            HitPoint::closest_front_hit_point(hit_points)
        } else {
            HitPoint::empty()
        }
    }
    
    fn traverse_bvh_for_meshes(ray: Ray, node: &BVH, mut meshes_to_check: Vec<Mesh>) -> Vec<Mesh> {
        if !node.is_leaf {
            if let Some(left_child) = &node.bvh_obj_1 {
                if ray.bb_intersects(left_child.bb_corner_1, left_child.bb_corner_2) {
                    meshes_to_check = Self::traverse_bvh_for_meshes(ray, left_child, meshes_to_check);
                }
            }
            if let Some(right_child) = &node.bvh_obj_2 {
                if ray.bb_intersects(right_child.bb_corner_1, right_child.bb_corner_2) {
                    meshes_to_check = Self::traverse_bvh_for_meshes(ray, right_child, meshes_to_check);
                }
            }
        } else {
            meshes_to_check.push(node.mesh);
        }

        meshes_to_check
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

            let hp1 = HitPoint::new_from_sphere(pt1, *ray, *sphere);
            let hp2 = HitPoint::new_from_sphere(pt2, *ray, *sphere);

            if hp1.normal*ray.direction > 0.0 {
                existing_hitpoints.push(hp1);
            } else {
                existing_hitpoints.push(hp2);
            }
        }

        existing_hitpoints
    }

    fn intersect_triangle(ray: &Ray, triangle: &Mesh, mut existing_hitpoints: Vec<HitPoint>) -> Vec<HitPoint> {

        if triangle.normal*ray.direction > 0.0 {
            return existing_hitpoints
        }

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
            let mut normal: Vector3 = triangle.normal;
            if triangle.smooth_shading { normal = triangle.compute_hitpoint_normal(point); }
            let hitpoint = HitPoint::new_from_tri(
                point,
                ray,
                &triangle,
                normal
            );
            existing_hitpoints.push(hitpoint);
        }
    
        existing_hitpoints
    }

    pub fn compute_hitpoint_normal(&self, hit_location: Vector3) -> Vector3 {
        let barycentric_coords = self.compute_barycentric_coords(hit_location);

        let p1_normal_weight = barycentric_coords.x;
        let p2_normal_weight = barycentric_coords.y;
        let p3_normal_weight = barycentric_coords.z;

        let interpolated_normal =
            self.p1_normal * p1_normal_weight + self.p2_normal * p2_normal_weight + self.p3_normal * p3_normal_weight;

        interpolated_normal.normalize()
    }
    
    fn compute_barycentric_coords(&self, point: Vector3) -> Vector3 {
        let v0 = self.p2 - self.p1;
        let v1 = self.p3 - self.p1;
        let v2 = point - self.p1;

        let dot00 = v0*v0;
        let dot01 = v0*v1;
        let dot11 = v1*v1;
        let dot20 = v2*v0;
        let dot21 = v2*v1;

        let denom: f64 = dot00 * dot11 - dot01 * dot01;
        let v: f64 = (dot11 * dot20 - dot01 * dot21) / denom;
        let w: f64 = (dot00 * dot21 - dot01 * dot20) / denom;
        let u = 1.0 - v - w;

        Vector3::new(u, v, w)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum PrimitiveMeshType {
    Empty = 0,
    Sphere = 1,
    Triangle = 2
}

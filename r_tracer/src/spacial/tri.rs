use crate::datatypes::vector3::Vector3;
use crate::datatypes::vector2::Vector2;
use crate::datatypes::hit_point::HitPoint;
use crate::spacial::ray::Ray;
use crate::spacial::bvh::BVH;
use crate::datatypes::material::Material;
use rand::Rng;


#[derive(Copy, Clone)]
pub struct Tri {
    pub p1: Vector3,
    pub p2: Vector3,
    pub p3: Vector3,
    pub p1_normal: Vector3,
    pub p2_normal: Vector3,
    pub p3_normal: Vector3,
    pub p1_texture: Vector2,
    pub p2_texture: Vector2,
    pub p3_texture: Vector2,

    pub normal: Vector3,
    pub smooth_shading: bool,

    pub bounding_box: (Vector3, Vector3),
    pub bounding_box_center: Vector3,

    pub material: Material,
    pub is_empty: bool
}

impl Tri {
    pub fn new(p1: Vector3, p2: Vector3, p3: Vector3, 
        p1_normal: Vector3, p2_normal: Vector3, p3_normal: Vector3,
        normal: Vector3, p1_texture: Vector2, p2_texture: Vector2,
        p3_texture: Vector2, material: Material) -> Tri {
        let bb = Self::get_bounding_box(p1, p2, p3);
        Tri {
            p1: p1,
            p2: p2,
            p3: p3,
            p1_normal: p1_normal,
            p2_normal: p2_normal,
            p3_normal: p3_normal,
            p1_texture: p1_texture,
            p2_texture: p2_texture,
            p3_texture: p3_texture,
            normal: normal,
            material: material,
            is_empty: false,
            smooth_shading: false,
            bounding_box: bb,
            bounding_box_center: Self::get_bounding_box_center(bb)
        }
    }

    pub fn empty() -> Tri {
        Tri {
            material: Material::empty(),
            p1: Vector3::zero(),
            p2: Vector3::zero(),
            p3: Vector3::zero(),
            p1_normal: Vector3::zero(),
            p2_normal: Vector3::zero(),
            p3_normal: Vector3::zero(),
            p1_texture: Vector2::zero(),
            p2_texture: Vector2::zero(),
            p3_texture: Vector2::zero(),
            normal: Vector3::zero(),
            smooth_shading: false,
            is_empty: true,
            bounding_box: (Vector3::zero(), Vector3::zero()),
            bounding_box_center: Vector3::zero()
        }
    }

    pub fn get_bounding_box(p1: Vector3, p2: Vector3, p3: Vector3) -> (Vector3, Vector3) {
        let rand_vec_1: Vector3 = Vector3::new(
            rand::thread_rng().gen_range(0.0..1.0), 
            rand::thread_rng().gen_range(0.0..1.0),
                rand::thread_rng().gen_range(0.0..1.0)
        );
        let rand_vec_2: Vector3 = Vector3::new(
            rand::thread_rng().gen_range(0.0..1.0), 
            rand::thread_rng().gen_range(0.0..1.0),
                rand::thread_rng().gen_range(0.0..1.0)
        );
        let (bb1, bb2) = (p1.min(p2.min(p3)), p1.max(p2.max(p3)));
        let size: f32 = 0.1*(bb1 - bb2).magnitude();
        (bb1 - rand_vec_1*size, bb2 + rand_vec_2*size)
    }

    pub fn get_bounding_box_center(bb: (Vector3, Vector3)) -> Vector3 {
        0.5*(bb.0 + bb.1)
    }

    pub fn ray_collision(ray: Ray, bvh: &BVH) -> HitPoint {
        let meshes_to_check = Self::traverse_bvh_for_meshes(ray, bvh, Vec::new());
        let mut closest_hit_point: HitPoint = HitPoint::empty();
        let mut closest_hit_distance: f32 = f32::MAX;

        for mesh in meshes_to_check {
            let hit_point: HitPoint = Self::intersect_tri(&ray, &mesh);
            if !hit_point.is_empty {
                let dist: f32 = hit_point.point.distance(hit_point.hitting_ray.origin);
                if closest_hit_point.is_empty || dist < closest_hit_distance {
                    closest_hit_distance = dist;
                    closest_hit_point = hit_point;
                }
            }
        }

        closest_hit_point
    }
    
    fn traverse_bvh_for_meshes(ray: Ray, node: &BVH, mut meshes_to_check: Vec<Tri>) -> Vec<Tri> {
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
            meshes_to_check.push(node.tri);
        }

        meshes_to_check
    }

    pub fn intersect_tri(ray: &Ray, triangle: &Tri) -> HitPoint {

        if triangle.normal*ray.direction > 0.0 {
            return HitPoint::empty()
        }

        let epsilon = 1e-6;
    
        let edge1 = triangle.p2 - triangle.p1;
        let edge2 = triangle.p3 - triangle.p1;
    
        let h = ray.direction.cross(&edge2);
        let a = edge1*h;
    
        if a.abs() < epsilon {
            return HitPoint::empty()
        }
    
        let f = 1.0 / a;
        let s = ray.origin - triangle.p1;
        let u = f*s*h;
    
        if u < 0.0 || u > 1.0 {
            return HitPoint::empty()
        }
    
        let q = s.cross(&edge1);
        let v = f * ray.direction*q;
    
        if v < 0.0 || u + v > 1.0 {
            return HitPoint::empty()
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
            return hitpoint
        }
    
        HitPoint::empty()
    }

    pub fn compute_hitpoint_normal(&self, hit_location: Vector3) -> Vector3 {
        let barycentric_coords = self.compute_barycentric_coords(hit_location); //todo: remove redundant calculation

        let p1_normal_weight = barycentric_coords.x;
        let p2_normal_weight = barycentric_coords.y;
        let p3_normal_weight = barycentric_coords.z;

        let interpolated_normal =
            self.p1_normal * p1_normal_weight + self.p2_normal * p2_normal_weight + self.p3_normal * p3_normal_weight;

        interpolated_normal.normalize()
    }
    
    pub fn compute_barycentric_coords(&self, point: Vector3) -> Vector3 {
        let v0 = self.p2 - self.p1;
        let v1 = self.p3 - self.p1;
        let v2 = point - self.p1;

        let dot00 = v0*v0;
        let dot01 = v0*v1;
        let dot11 = v1*v1;
        let dot20 = v2*v0;
        let dot21 = v2*v1;

        let denom: f32 = dot00 * dot11 - dot01 * dot01;
        let v: f32 = (dot11 * dot20 - dot01 * dot21) / denom;
        let w: f32 = (dot00 * dot21 - dot01 * dot20) / denom;
        let u = 1.0 - v - w;

        Vector3::new(u, v, w)
    }

    pub fn compute_face_normal(p1: Vector3, p2: Vector3, p3: Vector3) -> Vector3 {
        let u: Vector3 = p2 - p1;
        let v: Vector3 = p3 - p1;
    
        let unnormalized_normal: Vector3 = Vector3 {
            x: u.z * v.y - u.y * v.z,
            y: u.x * v.z - u.z * v.x,
            z: u.y * v.x - u.x * v.y,
        };
    
        unnormalized_normal.normalize()
    }
}

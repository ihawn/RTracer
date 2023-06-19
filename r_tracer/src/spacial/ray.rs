use crate::datatypes::vector3::Vector3;
use crate::datatypes::vector2::Vector2;
use crate::spacial::camera::Camera;
use crate::datatypes::color::Color;
use crate::datatypes::hit_point::HitPoint;
use crate::spacial::mesh::Mesh;
use crate::spacial::bvh::BVH;
use crate::datatypes::material::Material;
use rand::Rng;


#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin: origin,
            direction: direction
        }
    }
    
    pub fn empty() -> Ray {
        Ray {
            origin: Vector3::zero(),
            direction: Vector3::zero()
        }
    }

    pub fn reflect(self, normal: Vector3) -> Vector3 {
        (self.direction - 2.0*normal*self.direction*normal).normalize()
    }

    pub fn refract(self, normal: Vector3, eta_ratio: f64) -> Vector3 {
        let cos_theta = f64::min(-1.0*self.direction*normal, 1.0);
        let perpendicular = eta_ratio*(self.direction.normalize() + cos_theta*normal);
        perpendicular - (1.0 - perpendicular.magnitude_squared()).abs().sqrt() * normal
    }

    pub fn refract_precomputed_cos_theta(self, normal: Vector3, eta_ratio: f64, cos_theta: f64) -> Vector3 {
        let perpendicular = eta_ratio*(self.direction.normalize() + cos_theta*normal);
        perpendicular - (1.0 - perpendicular.magnitude_squared()).abs().sqrt() * normal
    }

    pub fn bb_intersects(&self, bb_corner_1: Vector3, bb_corner_2: Vector3) -> bool {
        let mut t_min: f64 = 0.0;
        let mut t_max: f64 = f64::MAX;
        for a in 0..3 {
            let inverse_dir: f64 = 1.0 / self.direction[a];
            let mut t0: f64 = (bb_corner_1[a] - self.origin[a]) * inverse_dir;
            let mut t1: f64 = (bb_corner_2[a] - self.origin[a]) * inverse_dir;
            if inverse_dir < 0.0 { (t0, t1) = (t1, t0) }
            if t0 > t_min { t_min = t0 }
            if t1 < t_max { t_max = t1 }
            if t_max <= t_min { return false }
        }
        true
    }

    pub fn cast_ray_from_camera(camera: &Camera, bvh: &BVH, sphere_objects: &Vec<Mesh>, x: usize, y: usize) -> Color {
        let projection_point = camera.blur_strength *
        Vector3::random_perturb(Vector2::new(camera.width as f64, camera.height as f64)) + 
        Vector3::new(
            (camera.width as f64) / camera.fov,
            y as f64 - (camera.width as f64)/2.0, 
            (camera.height as f64)/2.0 - x as f64
        ).normalize().rot(camera.rotation);

        let focal_point = camera.position + camera.focal_distance * projection_point.normalize();
        let ray_origin = camera.position + camera.dof_strength * Vector3::random_perturb(Vector2::new(1.0, 1.0));
        let ray_direction = (focal_point - ray_origin).normalize();

        Ray::new(ray_origin, ray_direction)
            .cast_ray(bvh, sphere_objects, camera.max_bounces, camera.exposure, camera.scene.env_color)
    }

    pub fn cast_ray(mut self, bvh: &BVH, sphere_objects: &Vec<Mesh>, 
        max_bounces: u32, exposure: f64, env_color: Color) -> Color {

        let mut hit_point: HitPoint = HitPoint::empty();

        let mut incoming_light: Color = Color::black();
        let mut ray_color: Color = Color::white();

        for i in 0..max_bounces + 1 {
            hit_point = Mesh::ray_collision(self, bvh, sphere_objects);

            if !hit_point.is_empty {

                let material: Material = hit_point.object.material;
                let random_val: f64 = rand::thread_rng().gen_range(0.0..1.0);

                self.origin = hit_point.point;
                self.direction = self.ray_redirect(hit_point, random_val);

                if material.is_dielectric {
                    ray_color = ray_color * material.color;
                } else {
                    let emitted_light: Color = material.emission_color;
                    let light_strength: f64 = hit_point.normal * self.direction;
                    incoming_light = emitted_light * ray_color + incoming_light;

                    if i > 3 && incoming_light.to_vector3().magnitude() < 0.015 {
                        break;
                    }
                    
                    ray_color = ray_color * Color::lerp(
                        material.color * light_strength * exposure, 
                        material.specular_color * light_strength * exposure, 
                        ((material.specular >= random_val) as u8) as f64
                    );
                }

            } else {
                incoming_light = env_color * ray_color + incoming_light;
                return incoming_light;
            }
        }

        incoming_light
    }

    fn ray_redirect(self: Ray, hit: HitPoint, random_val: f64) -> Vector3 {
        let mat: Material = hit.object.material;
        if mat.is_dielectric {
            let mut ior = mat.index_of_refraction;
            if hit.is_front_face { ior = 1.0/ior } 

            let cos_theta: f64 = f64::min(-1.0*self.direction*hit.normal, 1.0);
            let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

            if ior*sin_theta > 1.0 || Self::get_reflectance(cos_theta, ior) > random_val {
                return self.reflect(hit.normal)
            } else {
                return self.refract_precomputed_cos_theta(hit.normal, ior, cos_theta)
            }
        } else {
            let diffuse_direction = Vector3::random_hemisphere_normal(hit.normal);
            let specular_direction = self.reflect(hit.normal);
            let is_specular_bounce = ((mat.specular >= random_val) as u8) as f64;
            return Vector3::lerp(diffuse_direction, specular_direction, mat.smoothness * is_specular_bounce);
        }
    }

    fn get_reflectance(cosine: f64, ior: f64) -> f64 {
        let mut r0: f64 = (1.0 - ior) / (1.0 + ior);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
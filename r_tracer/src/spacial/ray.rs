use crate::datatypes::vector3::Vector3;
use crate::datatypes::vector2::Vector2;
use crate::spacial::camera::Camera;
use crate::datatypes::color::Color;
use crate::datatypes::hit_point::{HitPoint, self};
use crate::spacial::tri::Tri;
use crate::spacial::bvh::BVH;
use crate::datatypes::material::Material;
use crate::spacial::scene::Scene;
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

    pub fn cast_ray_from_camera(camera: &Camera, bvh: &BVH, x: usize, y: usize) -> Color {
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
            .cast_ray(bvh, camera.max_bounces, camera.exposure, &camera.scene)
    }

    pub fn cast_ray(mut self, bvh: &BVH, max_bounces: u32, exposure: f64, scene: &Scene) -> Color {

        let mut hit_point: HitPoint;
        let mut incoming_light: Color = Color::black();
        let mut ray_color: Color = Color::white();

        for _i in 0..max_bounces + 1 {
            hit_point = Tri::ray_collision(self, bvh);

            if !hit_point.is_empty {

                let (
                    diffuse_color, emission_color,
                    specular_color, dielectric_color,
                    normal_map_vector
                ) = Self::get_maps(&hit_point, scene);

                let material: Material = hit_point.object.material;
                let random_val: f64 = rand::thread_rng().gen_range(0.0..1.0);

                self.origin = hit_point.point;
                self.direction = self.ray_redirect(hit_point, random_val, normal_map_vector);

                if material.visible {
                    incoming_light = emission_color * ray_color + incoming_light;
                }
                
                if material.dielectric > 0.0 {
                    ray_color = ray_color * dielectric_color;
                } else {
                    let light_strength: f64 = hit_point.normal * self.direction;
                    ray_color = ray_color * Color::lerp(
                        diffuse_color * light_strength * exposure, 
                        specular_color * light_strength * exposure, 
                        ((material.specular >= random_val) as u8) as f64
                    );
                }

            } else {
                incoming_light = scene.env_color * ray_color + incoming_light;
                return incoming_light;
            }
        }

        incoming_light
    }

    fn ray_redirect(self: Ray, hit: HitPoint, random_val: f64, normal_map_vector: Vector3) -> Vector3 {        
        let mat: Material = hit.object.material;
        let is_specular_bounce = (mat.specular >= random_val) as u8 as f64;
        let mut normal: Vector3 = hit.normal;
        if normal_map_vector != Vector3::zero() {
            normal = (normal + normal_map_vector*hit.object.material.normal_strength).normalize();
        }

        let diffuse_direction: Vector3 = Vector3::random_hemisphere_normal(normal);
        let specular_direction: Vector3 = self.reflect(normal);
        let glossy_direction: Vector3 = Vector3::lerp(diffuse_direction, specular_direction, mat.smoothness * is_specular_bounce);

        if mat.dielectric > 0.0 {
            let mut ior: f64 = mat.index_of_refraction;
            if hit.is_front_face { ior = 1.0 / ior }
            let random_val_2: f64 = rand::thread_rng().gen_range(0.0..1.0);
    
            let cos_theta: f64 = f64::min(-1.0 * self.direction * normal, 1.0);
            let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
    
            if ior * sin_theta > 1.0 || Self::get_reflectance(cos_theta, ior) > random_val_2 {
                return glossy_direction
            } else {
                let random_val_3 = rand::thread_rng().gen_range(0.0..1.0);
                let is_dielectric_bounce = (mat.dielectric >= random_val_3) as u8 as f64;
                let refracted_direction = Vector3::lerp(
                    -1.0*diffuse_direction, self.refract_precomputed_cos_theta(normal, ior, cos_theta), mat.smoothness
                );  
                return Vector3::lerp(glossy_direction, refracted_direction, is_dielectric_bounce);
            }
        } else {
            return glossy_direction
        }
    }

    fn get_reflectance(cosine: f64, ior: f64) -> f64 {
        let mut r0: f64 = (1.0 - ior) / (1.0 + ior);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }

    fn get_maps(hit: &HitPoint, scene: &Scene) -> (Color, Color, Color, Color, Vector3) {
        let mut diffuse_col: Color = hit.object.material.diffuse_color;
        let mut emission_col: Color = hit.object.material.emission_color;
        let mut specular_col: Color = hit.object.material.specular_color;
        let mut dielectric_col: Color = hit.object.material.dielectric_color;
        let mut normal_map_vec: Vector3 = Vector3::zero();

        let uv: Vector2 = hit.barycentric_coords.x*hit.object.p1_texture
            + hit.barycentric_coords.y*hit.object.p2_texture
            + hit.barycentric_coords.z*hit.object.p3_texture;

        if hit.object.material.diffuse_color_map_index != None {
            diffuse_col = Self::get_map_color(scene, uv, hit.object.material.diffuse_color_map_index.unwrap());
        }
        if hit.object.material.emission_color_map_index != None {
            emission_col = Self::get_map_color(scene, uv, hit.object.material.emission_color_map_index.unwrap());
        }
        if hit.object.material.specular_color_map_index != None {
            specular_col = Self::get_map_color(scene, uv, hit.object.material.specular_color_map_index.unwrap());
        }
        if hit.object.material.dielectric_color_map_index != None {
            dielectric_col = Self::get_map_color(scene, uv, hit.object.material.dielectric_color_map_index.unwrap());
        }
        if hit.object.material.normal_map_index != None {
            normal_map_vec = (Self::get_map_color(
                scene, uv, hit.object.material.normal_map_index.unwrap()
            ).to_vector3() * 2.0 - Vector3::one()).normalize();
        }

        (diffuse_col, emission_col, specular_col, dielectric_col, normal_map_vec)
    }

    fn get_map_color(scene: &Scene, uv: Vector2, map_index: usize) -> Color {
        let width: usize = scene.texture_maps[map_index].width;
        let height: usize = scene.texture_maps[map_index].height;
        *scene.texture_maps[map_index].get(
            (f64::round((width as f64 - 1.0) * uv.x) as usize) % width, 
            (f64::round((height as f64 - 1.0) * uv.y) as usize) % height
        ).unwrap()
    }
}
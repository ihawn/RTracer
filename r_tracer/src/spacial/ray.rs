use crate::datatypes::vector3::Vector3;
use crate::datatypes::vector2::Vector2;
use crate::spacial::camera::Camera;
use crate::datatypes::color::Color;
use crate::datatypes::hit_point::{HitPoint, self};
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

    pub fn bb_intersects(&self, bb_corner_1: Vector3, bb_corner_2: Vector3) -> bool {
        let inv_direction = Vector3::new(1.0 / self.direction.x, 1.0 / self.direction.y, 1.0 / self.direction.z);
        let t1 = (bb_corner_1.x - self.origin.x) * inv_direction.x;
        let t2 = (bb_corner_2.x - self.origin.x) * inv_direction.x;
        let t3 = (bb_corner_1.y - self.origin.y) * inv_direction.y;
        let t4 = (bb_corner_2.y - self.origin.y) * inv_direction.y;
        let t5 = (bb_corner_1.z - self.origin.z) * inv_direction.z;
        let t6 = (bb_corner_2.z - self.origin.z) * inv_direction.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        return tmax >= tmin;
    }

    pub fn cast_ray(camera: &Camera, mut pixel_projection: Vector3, 
        mut cached_first_hit: HitPoint, bvh: &BVH, sphere_objects: &Vec<Mesh>,
        x: usize, y: usize) -> Color {

        pixel_projection = camera.blur_strength *
            Vector3::random_perturb(Vector2::new(camera.width as f64, camera.height as f64)) + 
            Vector3::new(
                camera.projection_distance,
                y as f64 - (camera.width as f64)/2.0, 
                (camera.height as f64)/2.0 - x as f64
            ).normalize().rot(camera.rotation);


        let mut incoming_light: Color = Color::black();
        let mut ray_color: Color = Color::white();
        let mut ray: Ray = Ray::new(camera.position, pixel_projection);
        let mut hit_point: HitPoint = HitPoint::empty();

        for i in 0..camera.max_bounces + 1 {

            /*if i == 0 {
                hit_point = cached_first_hit;
            } else {*/
                hit_point = Mesh::ray_collision(ray, bvh, sphere_objects);
            //}

            if !hit_point.is_empty {

                let material: Material = hit_point.object.material;

                ray.origin = hit_point.point;
                let diffuse_direction = Vector3::random_hemisphere_normal(hit_point.normal);
                let specular_direction = ray.reflect(hit_point.normal);
                let is_specular_bounce = ((material.specular >= rand::thread_rng().gen_range(0.0..1.0)) as u8) as f64;
                ray.direction = Vector3::lerp(
                    diffuse_direction, specular_direction, material.smoothness * is_specular_bounce
                );

                let emitted_light: Color = material.emission_color;
                let light_strength: f64 = hit_point.normal * ray.direction;
                incoming_light = emitted_light * ray_color + incoming_light;

                if i > 3 && incoming_light.to_vector3().magnitude() < 0.015 {
                    break;
                }
                
                ray_color = ray_color * Color::lerp(
                    material.color * light_strength * camera.exposure, 
                    material.specular_color * light_strength * camera.exposure, 
                    is_specular_bounce
                );

            } else {
                incoming_light = camera.scene.env_color * ray_color + incoming_light;
                return incoming_light;
            }
        }

        incoming_light
    }
}
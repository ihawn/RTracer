use crate::datatypes::vector3::Vector3;
use crate::spacial::camera::Camera;
use crate::datatypes::color::Color;
use crate::datatypes::hit_point::HitPoint;
use crate::spacial::mesh::Mesh;
use crate::datatypes::material::Material;
use rand::Rng;
use uuid::Uuid;


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

    pub fn cast_ray(camera: Camera, pixel_projection: &Vector3) -> Color {

        let mut incoming_light: Color = Color::black();
        let mut ray_color: Color = Color::white();
        let mut ray: Ray = Ray::new(camera.position, *pixel_projection);
        
        let mut hit_skip_id = Uuid::new_v4();

        for _i in 0..camera.max_bounces + 1 {

            let hit_point: HitPoint = Mesh::ray_collision(
                ray, &camera.scene.meshes, hit_skip_id
            );
            hit_skip_id = hit_point.object.id;

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
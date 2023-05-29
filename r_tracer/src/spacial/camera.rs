use crate::datatypes::material;
use crate::datatypes::vector3::{Vector3, self};
use crate::datatypes::color::Color;
use crate::datatypes::material::Material;
use crate::datatypes::vector2d::Vector2D;
use crate::utilities::frame_handler::FrameHandler;
use crate::spacial::scene::Scene;
use crate::spacial::ray::Ray;
use crate::datatypes::hit_point::HitPoint;

use super::sphere::Sphere;

#[derive(Clone)]
pub struct Camera {
    pub position: Vector3,
    pub scene: Scene,
    pub projection_distance: f64,
    pub width: usize,
    pub height: usize,
    pub rotation: Vector3
}

impl Camera {
    pub fn new(width: usize, height: usize, scene: Scene) -> Camera {
        Camera {
             position: Vector3::new(-100.0, 0.0, 0.0),
             scene: scene,
             projection_distance: 500.0,
             width: width,
             height: height,
             rotation: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn render_scene(self) -> Vector2D<Color> {
        let black: Color = Color::new(0, 0, 0);
        let mut frame: Vector2D<Color> = Vector2D::new(
            self.width, 
            self.height, 
            black
        );

        for x in 0..frame.height {
            for y in 0..frame.width {
                frame.set(x, y, Self::cast_ray(self.clone(), x, y));
            }
        }

        frame
    }

    fn cast_ray(camera: Camera, x: usize, y: usize) -> Color {

        let start_ray_direction: Vector3 = Vector3::new(
            camera.projection_distance,
            y as f64 - (camera.width as f64)/2.0, 
            (camera.height as f64)/2.0 - x as f64
        ).rot(camera.rotation);


        let mut ray: Ray = Ray::new(camera.position, start_ray_direction);
        
        let hit_point: HitPoint = Self::ray_sphere_collision(
            ray, &camera.scene.spheres, -1
        );
        if hit_point.is_empty {
            return hit_point.object.material.color;
        }

        let original_hit_obj = hit_point.object.clone();

        ray.origin = hit_point.point;
        ray.direction = Vector3::random_hemisphere_normal(hit_point.normal);
        
        let max_bounces = 3;

        let mut incoming_light: Color = hit_point.object.material.emission_color;
        let mut ray_color: Color = hit_point.object.material.color;

        for i in 0..max_bounces {

            let hit_point: HitPoint = Self::ray_sphere_collision(
                ray, &camera.scene.spheres, hit_point.object.id
            );

            if !hit_point.is_empty {

                ray.origin = hit_point.point;
                ray.direction = Vector3::random_hemisphere_normal(hit_point.normal);

                let material: Material = hit_point.object.material;
                let emitted_light: Color = material.emission_color;
                incoming_light = emitted_light * ray_color + incoming_light;
                ray_color = hit_point.object.material.color * ray_color;

            } else {
                break;
            }
        }

        incoming_light
    }

    fn ray_sphere_collision(ray: Ray, objects: &Vec<Sphere>, skip_id: i32) -> HitPoint {
        let mut hit_points: Vec<HitPoint> = Vec::new();
        for sphere in objects {
            if sphere.id == skip_id {
                continue; //skip the object that was just reflected off
                          //will have to re-think this when we have concave objects
            }

            let r: f64 = sphere.radius;
            let object_direction: Vector3 = ray.origin - sphere.center;

            let a: f64 = ray.direction.self_dot();
            let b: f64 = 2.0*object_direction*ray.direction;
            let c: f64 = object_direction.square().component_add() - r*r ;
            
            let desc: f64 = b*b - 4.0*a*c;

            if desc >= 0.0 {
                let t1: f64 = (-b + desc.sqrt()) / (2.0 * a);
                let t2: f64 = (-b - desc.sqrt()) / (2.0 * a);
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
            Self::closest_front_hit_point(hit_points)
        } else {
            HitPoint::empty()
        }
    }

    fn closest_front_hit_point(hit_points: Vec<HitPoint>) -> HitPoint {
        let mut min_dist: f64 = hit_points[0].point.distance(hit_points[0].hitting_ray.origin);
        let mut min_i: usize = 0;
        for i in (1..hit_points.len()) {
            let dist = hit_points[i].point.distance(hit_points[1].hitting_ray.origin);
            if dist < min_dist
            && (hit_points[i].point - hit_points[i].hitting_ray.origin) * hit_points[i].hitting_ray.direction > 0.0 {
                min_i = i;
                min_dist = dist;
            }
        }

        hit_points[min_i]
    }

}

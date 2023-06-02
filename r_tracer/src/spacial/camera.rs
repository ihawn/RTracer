use crate::datatypes::material;
use minifb::{Window, WindowOptions, Error};
use crate::datatypes::vector3::{Vector3, self};
use crate::datatypes::color::Color;
use crate::datatypes::material::Material;
use crate::datatypes::vector2d::Vector2D;
use crate::utilities::frame_handler::FrameHandler;
use crate::spacial::scene::Scene;
use crate::spacial::ray::Ray;
use crate::datatypes::hit_point::HitPoint;
use rayon::prelude::*;
use std::sync::{Mutex, MutexGuard};
use rand::Rng;

use super::sphere::Sphere;

#[derive(Clone)]
pub struct Camera {
    pub position: Vector3,
    pub scene: Scene,
    pub projection_distance: f64,
    pub width: usize,
    pub height: usize,
    pub rotation: Vector3,
    pub max_bounces: u32,
    pub rays_per_pixel: u32
}

impl Camera {
    pub fn new(width: usize, height: usize, scene: Scene, max_bounces: u32, rays_per_pixel: u32) -> Camera {
        Camera {
             position: Vector3::new(400.0, 100.0, 425.0),
             scene: scene,
             projection_distance: 1500.0,
             width: width,
             height: height,
             rotation: Vector3::new(0.0, 60.0, 0.0),
             max_bounces: max_bounces,
             rays_per_pixel: rays_per_pixel
        }
    }

    pub fn render_scene(self, mut handler: FrameHandler, sample_count: u32) -> FrameHandler {
        let mut camera: Camera = self.clone();
        let mut new_render: Vector2D<Color> = Vector2D::new(self.width, self.height, Color::black());

        let mut pixel_projections: Vector2D<Vector3> = Self::get_pixel_projections(self.clone());
        (new_render, pixel_projections) = camera.render_sample(pixel_projections);
        let mut old_render: Vector2D<Color> = new_render;
        let mut pixel_accumulation: Vector2D<Color> = old_render;

        let height = self.height;
        let width = self.width;
        let mut weight: f64 = 1.0;
        
        for i in 0..sample_count {

            println!("Sample {}/{}", i + 1, sample_count);

            camera = self.clone();
            old_render = pixel_accumulation;
            (new_render, pixel_projections) = camera.render_sample(pixel_projections);
            weight = 1.0 / (i as f64 + 1.0);

            pixel_accumulation = old_render * (1.0 - weight) + new_render * weight;                
            
            let converted_values: Vec<u32> = pixel_accumulation.data.iter()
                .map(|color| color.as_buffer_color()).collect();

            let _update: Result<(), Error> = handler.window.update_with_buffer(
                &converted_values, width, height
            );
        }

        handler
    }

    pub fn render_sample(self, pixel_projections: Vector2D<Vector3>) -> (Vector2D<Color>, Vector2D<Vector3>) {
        let black: Color = Color::new(0.0, 0.0, 0.0);
        let mut frame: Vector2D<Color> = Vector2D::new(
            self.width, 
            self.height, 
            black
        );

        let vert: Vec<usize> = (0..frame.height).collect();
        let horz: Vec<usize> = (0..frame.width).collect();
        let vert_slice: &[usize] = &vert;
        let horz_slice: &[usize] = &horz;

        let frame: Mutex<Vector2D<Color>> = Mutex::new(frame);
        vert_slice.par_iter().for_each(|&x| {
            horz_slice.par_iter().for_each(|&y| {
                let mut pixel_color: Color = Color::black();
                for _s in 0..self.rays_per_pixel {
                    pixel_color += Self::cast_ray(
                        self.clone(), 
                        pixel_projections.get(x, y).unwrap()
                    );
                }
                pixel_color /= self.rays_per_pixel as u32;             
                let mut frame: MutexGuard<Vector2D<Color>> = frame.lock().unwrap();             
                frame.set(x, y, pixel_color);
            });
        });

        (frame.into_inner().unwrap(), pixel_projections)
    }

    fn cast_ray(camera: Camera, pixel_projection: &Vector3) -> Color {

        let mut incoming_light: Color = Color::black();
        let mut ray_color: Color = Color::white();
        let mut ray: Ray = Ray::new(camera.position, *pixel_projection);
        
        let mut hit_skip_id = -1;

        for i in 0..camera.max_bounces + 1 {

            let hit_point: HitPoint = Self::ray_sphere_collision(
                ray, &camera.scene.spheres, hit_skip_id
            );
            hit_skip_id = hit_point.object.id;

            if !hit_point.is_empty {

                let material: Material = hit_point.object.material;

                ray.origin = hit_point.point;
                let diffuse_direction = Vector3::random_hemisphere_normal(hit_point.normal);
                let specular_direction = ray.reflect(hit_point.normal);
                let is_specular_bounce = ((material.specular >= rand::thread_rng().gen()) as u8) as f64;
                ray.direction = Vector3::lerp(diffuse_direction, specular_direction, material.smoothness * is_specular_bounce);

                let emitted_light: Color = material.emission_color;
                let light_strength: f64 = hit_point.normal * ray.direction;
                incoming_light = emitted_light * ray_color + incoming_light;

                
                ray_color = ray_color * Color::lerp(
                    material.color * light_strength * 2.0, material.specular_color * light_strength * 2.0, is_specular_bounce
                );

            } else {
                incoming_light = camera.scene.env_color * ray_color + incoming_light;
                return incoming_light;
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

        if min_i > 0 {
            return hit_points[min_i];
        } else if (hit_points[0].point - hit_points[0].hitting_ray.origin) * hit_points[0].hitting_ray.direction > 0.0 {
            return hit_points[min_i];
        } else {
            return HitPoint::empty();
        }  
    }

    fn get_pixel_projections(camera: Camera) -> Vector2D<Vector3> {
        let mut projections: Vector2D<Vector3> = Vector2D::new(camera.width, camera.height, Vector3::zero());
        for x in 0..camera.height {
            for y in 0..camera.width {
                projections.set(x, y,
                    Vector3::new(
                        camera.projection_distance,
                        y as f64 - (camera.width as f64)/2.0, 
                        (camera.height as f64)/2.0 - x as f64
                    ).rot(camera.rotation)
                );
            }
        }
        projections
    }

}
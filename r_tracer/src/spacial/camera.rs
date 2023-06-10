use minifb::Error;
use crate::datatypes::vector3::Vector3;
use crate::datatypes::vector2::Vector2;
use crate::datatypes::color::Color;
use crate::datatypes::hit_point::HitPoint;
use crate::datatypes::vector2d::Vector2D;
use crate::datatypes::vector3d::Vector3D;
use crate::utilities::frame_handler::FrameHandler;
use crate::spacial::scene::Scene;
use crate::spacial::mesh::{Mesh, PrimitiveMeshType};
use crate::spacial::ray::Ray;
use crate::spacial::bvh::BVH;
use rayon::prelude::*;
use std::sync::{Mutex, MutexGuard};


#[derive(Clone)]
pub struct Camera {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scene: Scene,
    pub exposure: f64,
    pub width: usize,
    pub height: usize,
    pub max_bounces: u32,
    pub rays_per_pixel: u32,
    pub blur_strength: f64,
    pub dof_strength: f64,
    pub focal_distance: f64,
    pub fov: f64
}

impl Camera {
    pub fn new(
        position: Vector3, rotation: Vector3, scene: Scene,
        exposure: f64, width: usize, height: usize, max_bounces: 
        u32, rays_per_pixel: u32, blur_str: f64, dof_strength: 
        f64, focal_distance: f64, fov: f64
    ) -> Camera {
        Camera {
             position: position,
             rotation: rotation,
             scene: scene,
             exposure: exposure,
             width: width,
             height: height,
             max_bounces: max_bounces,
             rays_per_pixel: rays_per_pixel,
             blur_strength: blur_str,       
             dof_strength: dof_strength,
             focal_distance: focal_distance,
             fov: fov,
        }
    }

    pub fn render_scene(self, mut handler: FrameHandler, sample_count: u32) -> FrameHandler {

        let bvh: BVH = BVH::new(&self.scene.meshes);

        let mut new_render: Vector2D<Color> = Vector2D::new(self.width, self.height, Color::black());
        let mut spheres: Vec<Mesh> = vec![];
        for m in &self.scene.meshes {
            if m.mesh_type == PrimitiveMeshType::Sphere { spheres.push(*m) }
        }

        new_render = self.render_sample(&bvh, &spheres);
        let mut old_render: Vector2D<Color> = new_render;
        let mut pixel_accumulation: Vector2D<Color> = old_render;

        let height = self.height;
        let width = self.width;
        let mut weight: f64 = 1.0;
        
        for i in 0..sample_count {

            println!("Sample {}/{}", i + 1, sample_count);

            old_render = pixel_accumulation;
            new_render = self.render_sample(&bvh, &spheres);
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

    pub fn render_sample(self: &Camera, bvh: &BVH, sphere_objects: &Vec<Mesh>) 
        -> Vector2D<Color> {
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
                for s in 0..self.rays_per_pixel {
                    pixel_color += Ray::cast_ray(
                        &self,
                        &bvh, &sphere_objects, x, y
                    );
                }
                pixel_color /= self.rays_per_pixel;             
                let mut frame: MutexGuard<Vector2D<Color>> = frame.lock().unwrap();             
                frame.set(x, y, pixel_color);
            });
        });

        frame.into_inner().unwrap()
    }
}
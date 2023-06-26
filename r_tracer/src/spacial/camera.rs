use minifb::Error;
use crate::datatypes::vector3::Vector3;
use crate::datatypes::color::Color;
use crate::datatypes::vector2d::Vector2D;
use crate::utilities::frame_handler::FrameHandler;
use crate::utilities::postprocessing::remove_fireflies;
use crate::spacial::scene::Scene;
use crate::spacial::ray::Ray;
use crate::spacial::bvh::BVH;
use rayon::prelude::*;
use std::sync::{Mutex, MutexGuard, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};


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
    pub fov: f64,
    pub tile_size: usize
}

impl Camera {
    pub fn new(
        position: Vector3, rotation: Vector3, scene: Scene,
        exposure: f64, width: usize, height: usize, max_bounces: 
        u32, rays_per_pixel: u32, blur_str: f64, dof_strength: 
        f64, focal_distance: f64, fov: f64, tile_size: usize
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
             tile_size: tile_size
        }
    }

    pub fn render_scene(self, mut handler: FrameHandler, sample_count: u32) -> Vector2D<Color> {

        let bvh: BVH = BVH::new(&self.scene.meshes);
        let height: usize = self.height;
        let width: usize = self.width;
        let tile_size: usize = self.tile_size;
        let frame_out: Vector2D<Color>;
        
        if self.tile_size > 0 {
            let tiles: Vec<(usize, usize)> = Self::get_tiles(width, height, tile_size);
            let tile_slice: &[(usize, usize)] = &tiles;
            let frame: Vector2D<Color> = Vector2D::new(self.width, self.height, Color::black());

            let frame: Mutex<Vector2D<Color>> = Mutex::new(frame);
            let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
            let total_tiles = tile_slice.len();
            
            tile_slice.par_iter().for_each(|&t| {
                let tile_render = self.render_tile(
                    t.0,
                    usize::min(width, t.0 + tile_size),
                    t.1,
                    usize::min(height, t.1 + tile_size),
                    sample_count as usize,
                    &bvh,
                    (0..sample_count).map(|s| 1.0 / (s as f64 + 1.0)).collect()
                );
            
                let mut frame: MutexGuard<Vector2D<Color>> = frame.lock().unwrap();
                for x in 0..tile_render.width {
                    for y in 0..tile_render.height {
                        frame.set(
                            usize::min(height, t.1 + x),
                            usize::min(width, t.0 + y),
                            *tile_render.get(x, y).unwrap(),
                        );
                    }
                }
            
                let current_tile: usize = counter.fetch_add(1, Ordering::Relaxed);
                println!("Render progress: {}%", (100.0 * ((current_tile + 1) as f64) / (total_tiles as f64)) as usize);
            });
            
            let frame: MutexGuard<Vector2D<Color>> = frame.lock().unwrap();
            let converted_values: Vec<u32> = frame.data.iter()
            .map(|color| color.as_buffer_color()).collect();
            
            let _update: Result<(), Error> = handler.window.update_with_buffer(
                &converted_values.clone(), width, height
            );
            frame_out = frame.clone();
        } else {
            let mut new_render: Vector2D<Color> = Vector2D::new(self.width, self.height, Color::black());
            let mut old_render: Vector2D<Color> = new_render;
            let mut pixel_accumulation: Vector2D<Color> = old_render;
            let weight_slice: Vec<f64> = (0..sample_count).map(|s| 1.0 / (s as f64 + 1.0)).collect();

            let sample_count_usize: usize = sample_count as usize;
            for i in 0..sample_count_usize {

                println!("Sample {}/{}", i + 1, sample_count);

                old_render = pixel_accumulation;
                new_render = self.render_whole_sample(&bvh);
                old_render *= 1.0 - weight_slice[i];
                new_render *= weight_slice[i];
                pixel_accumulation = old_render + new_render;            
                
                let converted_values: Vec<u32> = remove_fireflies(&pixel_accumulation).data.iter()
                    .map(|color| color.as_buffer_color()).collect();

                let _update: Result<(), Error> = handler.window.update_with_buffer(
                    &converted_values, width, height
                );
            }
            frame_out = pixel_accumulation;
        }

        frame_out
    }

    pub fn render_tile(self: &Camera, start_x: usize, end_x: usize, 
        start_y: usize, end_y: usize, sample_count: usize,
         bvh: &BVH, weight_slice: Vec<f64>) 
    -> Vector2D<Color> {

        let mut frame: Vector2D<Color> = Vector2D::new(
            self.tile_size as usize, self.tile_size as usize, Color::black()
        );
        let mut old_frame: Vector2D<Color>;
        let mut pixel_accumulation: Vector2D<Color> = Vector2D::new(
            self.tile_size as usize, self.tile_size as usize, Color::black()
        );

        for s in 0..sample_count {
            old_frame = pixel_accumulation;
            for x in start_y..end_y {
                for y in start_x..end_x {
                    let mut pixel_color: Color = Color::black();
                    for _s in 0..self.rays_per_pixel {
                        pixel_color += Ray::cast_ray_from_camera(
                            &self,
                            &bvh, x, y
                        );
                    }
                    pixel_color /= self.rays_per_pixel;     
                    frame.set(x - start_y, y - start_x, pixel_color);
                }
            }
            old_frame *= 1.0 - weight_slice[s];
            frame *= weight_slice[s];
            pixel_accumulation = &old_frame + &frame;
        }

        pixel_accumulation
    }

    pub fn render_whole_sample(self: &Camera, bvh: &BVH) 
        -> Vector2D<Color> {
        let frame: Vector2D<Color> = Vector2D::new(
            self.width, 
            self.height, 
            Color::black()
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
                    pixel_color += Ray::cast_ray_from_camera(&self, &bvh, x, y);
                }
                pixel_color /= self.rays_per_pixel;             
                let mut frame: MutexGuard<Vector2D<Color>> = frame.lock().unwrap();             
                frame.set(x, y, pixel_color);
            });
        });

        frame.into_inner().unwrap()
    }

    fn get_tiles(width: usize, height: usize, tile_size: usize) -> Vec<(usize, usize)> {
        let mut tiles: Vec<(usize, usize)> = vec![];
        for x in (0..width).step_by(tile_size as usize) {
            for y in (0..height).step_by(tile_size as usize) {
                tiles.push((x, y))
            }
        }
        tiles
    }
}
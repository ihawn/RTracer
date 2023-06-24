use pk_stl::parse_stl;
use image::{Rgb, RgbImage};
use rayon::prelude::*;
use crate::spacial::tri::Tri;
use crate::datatypes::material::Material;
use crate::datatypes::vector3::Vector3;
use crate::datatypes::color::Color;
use crate::datatypes::vector2d::Vector2D;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub fn load_model(file_path: &str, material: Material) -> Vec<Tri>  {
    println!("Loading model {}", file_path);
    let content = fs::read(file_path).expect("Failed to read model file");
    let model = parse_stl(content.as_slice()).unwrap();

    println!("Computing vertex normals for {}", file_path);
    let i: usize = 0;
    let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let model_tris: Vec<Tri> = model.triangles.par_iter().map(|tri| {
        let vertex1: Vector3 = Vector3::new(tri.vertices[0].x.into(), tri.vertices[0].y.into(), tri.vertices[0].z.into());
        let vertex2: Vector3 = Vector3::new(tri.vertices[1].x.into(), tri.vertices[1].y.into(), tri.vertices[1].z.into());
        let vertex3: Vector3 = Vector3::new(tri.vertices[2].x.into(), tri.vertices[2].y.into(), tri.vertices[2].z.into());
        let face_normal = Vector3::new(tri.normal.x.into(), tri.normal.y.into(), tri.normal.z.into());
    
        let mut vertex1_normal: Vector3 = Vector3::zero();
        let mut vertex2_normal: Vector3 = Vector3::zero();
        let mut vertex3_normal: Vector3 = Vector3::zero();
    
        let e: f64 = 0.0001;
        for other_tri in &model.triangles {
            let other_vertex1: Vector3 = Vector3::new(other_tri.vertices[0].x.into(), other_tri.vertices[0].y.into(), other_tri.vertices[0].z.into());
            let other_vertex2: Vector3 = Vector3::new(other_tri.vertices[1].x.into(), other_tri.vertices[1].y.into(), other_tri.vertices[1].z.into());
            let other_vertex3: Vector3 = Vector3::new(other_tri.vertices[2].x.into(), other_tri.vertices[2].y.into(), other_tri.vertices[2].z.into());
            let other_face_normal = Vector3::new(other_tri.normal.x.into(), other_tri.normal.y.into(), other_tri.normal.z.into());
    
            if (vertex1 - other_vertex1).magnitude() < e
                || (vertex1 - other_vertex2).magnitude() < e
                || (vertex1 - other_vertex3).magnitude() < e
            {
                vertex1_normal += other_face_normal;
            }
            if (vertex2 - other_vertex1).magnitude() < e
                || (vertex2 - other_vertex2).magnitude() < e
                || (vertex2 - other_vertex3).magnitude() < e
            {
                vertex2_normal += other_face_normal;
            }
            if (vertex3 - other_vertex1).magnitude() < e
                || (vertex3 - other_vertex2).magnitude() < e
                || (vertex3 - other_vertex3).magnitude() < e
            {
                vertex3_normal += other_face_normal;
            }
        }
    
        vertex1_normal = vertex1_normal.normalize();
        vertex2_normal = vertex2_normal.normalize();
        vertex3_normal = vertex3_normal.normalize();
    
        let tri: Tri = Tri::new(
            vertex1, vertex2, vertex3, vertex1_normal, 
            vertex2_normal, vertex3_normal, face_normal, material
        );

        let current: usize = counter.fetch_add(1, Ordering::Relaxed);
        if current % 1000 == 0 {
            println!("Computing vertex normals: {}/{}", current, model.triangles.len());
        }
        tri
    }).collect();
    println!("Done");
    model_tris
}

pub fn compute_area(p1: Vector3, p2: Vector3, p3: Vector3) -> f64 {
    let v0 = p2 - p1;
    let v1 = p3 - p1;
    let cross_product = v0.cross(&v1);
    cross_product.magnitude() * 0.5
}


pub fn save_vector2d_as_png(vector: &Vector2D<Color>, filename: &str) -> Result<(), image::ImageError> {
    let mut image = RgbImage::new(vector.width as u32, vector.height as u32);
    for (i, color) in vector.data.iter().enumerate() {
        let x = (i % vector.width) as u32;
        let y = (i / vector.width) as u32;

        let rgb_color = Rgb([
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
        ]);

        image.put_pixel(x, y, rgb_color);
    }

    image.save(filename)
}
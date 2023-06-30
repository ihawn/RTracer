use pk_stl::parse_stl;
use obj::{load_obj, Obj, TexturedVertex};
use image::{Rgb, RgbImage};
use rayon::prelude::*;
use crate::spacial::tri::Tri;
use crate::datatypes::material::Material;
use crate::datatypes::vector3::Vector3;
use crate::datatypes::color::Color;
use crate::datatypes::vector2d::Vector2D;
use crate::utilities::postprocessing::remove_fireflies;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;

pub fn load_model(file_path: &str, material: Material) -> Vec<Tri> {
    if file_path.ends_with(".obj") {
        println!("Processing .obj file: {}", file_path);
        return import_obj(file_path, material)
    } else if file_path.ends_with(".stl") {
        println!("Processing .stl file: {}", file_path);
        return import_stl(file_path, material)
    } else {
        println!("Unsupported file extension");
        return vec![]
    }
}

fn import_stl(file_path: &str, material: Material) -> Vec<Tri> {
    let content = fs::read(file_path).expect("Failed to read model file");
    let model = parse_stl(content.as_slice()).unwrap();

    println!("Computing vertex normals for {}", file_path);
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
        if current % 1000 == 0 || model.triangles.len() == current {
            println!("Computing vertex normals: {}/{}", current, model.triangles.len());
        }
        tri
    }).collect();
    println!("Done");
    model_tris
}

fn import_obj(file_path: &str, material: Material) -> Vec<Tri> {
    let input = BufReader::new(File::open(file_path).expect("Failed to read OBJ"));
    let dome:  Obj<TexturedVertex> = load_obj(input).expect("Failed to load OBJ");
    let vertices = dome.vertices;
    let indices = dome.indices;
    let mut triangles: Vec<Tri> = Vec::new();

    for chunk in indices.chunks(3) {
        if chunk.len() != 3 {
            continue;
        }

        let p1_index = chunk[0] as usize;
        let p2_index = chunk[1] as usize;
        let p3_index = chunk[2] as usize;

        let vert1 = vertices[p1_index];
        let vert2 = vertices[p2_index];
        let vert3 = vertices[p3_index];

        let p1: Vector3 = Vector3::new(vert1.position[0] as f64, vert1.position[2] as f64, vert1.position[1] as f64);
        let p2: Vector3 = Vector3::new(vert2.position[0] as f64, vert2.position[2] as f64, vert2.position[1] as f64);
        let p3: Vector3 = Vector3::new(vert3.position[0] as f64, vert3.position[2] as f64, vert3.position[1] as f64);

        let p1_normal: Vector3 = Vector3::new(vert1.normal[0] as f64, vert1.normal[2] as f64, vert1.normal[1] as f64);
        let p2_normal: Vector3 = Vector3::new(vert2.normal[0] as f64, vert2.normal[2] as f64, vert2.normal[1] as f64);
        let p3_normal: Vector3 = Vector3::new(vert3.normal[0] as f64, vert3.normal[2] as f64, vert3.normal[1] as f64);


        let triangle = Tri::new(p1, p2, p3, p1_normal, p2_normal, p3_normal,
            Tri::compute_face_normal(p1, p2, p3), material,
        );

        triangles.push(triangle);
    }


    triangles
}

pub fn save_vector2d_as_png(vector: &Vector2D<Color>, filename: &str) -> Result<(), image::ImageError> {
    let mut image = RgbImage::new(vector.width as u32, vector.height as u32);
    for (i, color) in remove_fireflies(vector).data.iter().enumerate() {
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
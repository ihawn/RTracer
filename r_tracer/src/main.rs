use std::io::{stdin, stdout, Read, Write};
use std::os::windows::prelude::MetadataExt;
use std::thread;
use std::time::Duration;
use crate::datatypes::vector3::Vector3;

mod utilities {
    pub mod frame_handler;
}

mod datatypes {
    pub mod color;
    pub mod vector2d;
    pub mod vector2;
    pub mod vector3;
    pub mod hit_point;
    pub mod material;
}

mod spacial {
    pub mod camera;
    pub mod scene;
    pub mod sphere;
    pub mod ray;
}

use datatypes::material::Material;
use utilities::frame_handler::FrameHandler;
use datatypes::color::Color;
use datatypes::vector2d::Vector2D;
use spacial::sphere::Sphere;
use spacial::scene::Scene;
use spacial::camera::Camera;

fn main() {
    let size_x: usize = 480;
    let size_y: usize = 320;

    let col1_mat = Material::new(
        Color::new(1.0, 0.8, 0.2),
        Color::black(),
        0.0
    );

    let col2_mat = Material::new(
        Color::new(0.1, 1.0, 0.6),
        Color::black(),
        0.0
    );

    let col3_mat = Material::new(
        Color::new(0.9, 0.5, 0.5),
        Color::black(),
        0.0
    );

    let emiss_mat = Material::new(
        Color::black(),
        Color::white(),
        0.0
    );

    let red_sphere = Sphere::new(
        700.0, 0.0, 0.0, 100.0, col1_mat, 0
    );
    let green_sphere = Sphere::new(
        600.0, 200.0, 0.0, 100.0, col2_mat, 1
    );
    let blue_sphere = Sphere::new(
        650.0, 150.0, -1083.0, 1000.0, col3_mat, 2
    );
    let emiss_sphere = Sphere::new(
        1200.0, 500.0, 200.0, 600.0, emiss_mat, 3
    );

    let spheres: Vec<Sphere> = vec![red_sphere, green_sphere, blue_sphere, emiss_sphere];

    let scene: Scene = Scene::new(spheres);
    let mut camera: Camera = Camera::new(size_x, size_y, scene);

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");
    let mut colors = camera.render_scene();

    let converted_values: Vec<u32> = FrameHandler::buffer_from_color_vec(&colors);
    let _update = frame_handler.window.update_with_buffer(
        &converted_values,
        frame_handler.size_x, frame_handler.size_y
    );
    let mut samples = 1;


    /*for i in (0..10){

        colors = camera.render_scene();

        let converted_values: Vec<u32> = FrameHandler::buffer_from_color_vec(&colors);
        let _update = frame_handler.window.update_with_buffer(
            &converted_values,
            frame_handler.size_x, frame_handler.size_y
        );
    }*/
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
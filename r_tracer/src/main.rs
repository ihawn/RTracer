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
    let size_x: usize = 1800;
    let size_y: usize = 1200;

    let col1_mat = Material::new(
        Color::new(1.0, 0.1, 0.1),
        Color::black(),
        Color::white(),
        0.0, 1.0, 
        0.06
    );

    let col2_mat = Material::new(
        Color::new(0.1, 1.0, 0.6),
        Color::black(),
        Color::new(0.1, 1.0, 0.6),
        0.0, 0.7,
        1.0
    );

    let col3_mat = Material::new(
        Color::new(0.5, 0.5, 0.5),
        Color::black(),
        Color::white(),
        0.0, 0.0,
        1.0
    );

    let col4_mat = Material::new(
        Color::new(1.0, 1.0, 1.0),
        Color::black(),
        Color::new(1.0, 1.0, 1.0),
        0.0, 1.0,
        1.0
    );

    let col5_mat = Material::new(
        Color::new(0.3, 1.0, 1.0),
        Color::black(),
        Color::white(),
        0.0, 1.0,
        0.0
    );

    let emiss_mat_1 = Material::new(
        Color::black(),
        Color::white(),
        Color::white(),
        15.0, 0.0,
        0.0
    );

    let emiss_mat_2 = Material::new(
        Color::black(),
        Color::white(),
        Color::white(),
        30.0, 0.0,
        0.0
    );

    let red_sphere = Sphere::new(
        700.0, 0.0, 0.0, 100.0, col1_mat, 0
    );
    let green_sphere = Sphere::new(
        750.0, 200.0, -12.0, 75.0, col2_mat, 1
    );
    let blue_sphere = Sphere::new(
        650.0, 150.0, -1083.0, 1000.0, col3_mat, 2
    );
    let another_sphere = Sphere::new(
        525.0, 50.0, -45.0, 40.0, col4_mat, 3
    );
    let another_sphere2 = Sphere::new(
        700.0, 350.0, -50.0, 50.0, col5_mat, 7
    );
    let emiss_sphere_3 = Sphere::new(
        600.0, 150.0, -60.0, 65.0, emiss_mat_2, 4
    );
    let emiss_sphere_1 = Sphere::new(
        2000.0, 500.0, 200.0, 600.0, emiss_mat_1, 5
    );
    let emiss_sphere_2 = Sphere::new(
        525.0, -100.0, -50.0, 50.0, emiss_mat_2, 6
    );

    let spheres: Vec<Sphere> = vec![
        red_sphere, green_sphere, blue_sphere, 
        emiss_sphere_1, emiss_sphere_2,
        emiss_sphere_3, another_sphere,
        another_sphere2
    ];

    let scene: Scene = Scene::new(spheres, Color::white() * 0.3);
    let mut camera: Camera = Camera::new(
        size_x, size_y, scene, 
        10, 2
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");
    frame_handler = camera.render_scene(frame_handler, 50);
    
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
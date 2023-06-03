use std::io::{stdin, stdout, Read, Write};
use crate::datatypes::vector3::Vector3;
use datatypes::material::Material;
use utilities::frame_handler::FrameHandler;
use datatypes::color::Color;
use spacial::mesh::Mesh;
use spacial::scene::Scene;
use spacial::camera::Camera;


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
    pub mod mesh;
    pub mod ray;
}


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
        1.0, 0.0,
        0.0
    );

    let emiss_mat_2 = Material::new(
        Color::black(),
        Color::white(),
        Color::white(),
        1.0, 0.0,
        0.0
    );

    let red_sphere = Mesh::new_sphere(
        700.0, 0.0, 0.0, 100.0, col1_mat
    );
    let green_sphere = Mesh::new_sphere(
        750.0, 200.0, -12.0, 75.0, col2_mat
    );
    let blue_sphere = Mesh::new_sphere(
        650.0, 150.0, -1083.0, 1000.0, col3_mat
    );
    let another_sphere = Mesh::new_sphere(
        525.0, 50.0, -45.0, 40.0, col4_mat
    );
    let another_sphere2 = Mesh::new_sphere(
        700.0, 350.0, -50.0, 50.0, col5_mat
    );
    let emiss_sphere_3 = Mesh::new_sphere(
        600.0, 150.0, -60.0, 65.0, emiss_mat_2
    );
    let emiss_sphere_1 = Mesh::new_sphere(
        2000.0, 500.0, 200.0, 600.0, emiss_mat_1
    );
    let emiss_sphere_2 = Mesh::new_sphere(
        525.0, -100.0, -50.0, 50.0, emiss_mat_2
    );
    let tri_1 = Mesh::new_triangle(
        Vector3::new(700.0, 0.0, 0.0),
        Vector3::new(700.0, 350.0, -50.0),
        Vector3::new(525.0, -100.0, -50.0), col2_mat);

    let meshes: Vec<Mesh> = vec![
        red_sphere, green_sphere, blue_sphere, 
        emiss_sphere_1, emiss_sphere_2,
        emiss_sphere_3, another_sphere,
        another_sphere2, tri_1
    ];

    let scene: Scene = Scene::new(meshes, Color::white() * 0.3);
    let camera: Camera = Camera::new(
        Vector3::new(400.0, 100.0, 425.0),
        Vector3::new(0.0, 60.0, 0.0),
        scene, 1500.0, 1.4, 
        size_x, size_y,
        7, 4
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");
    frame_handler = camera.render_scene(frame_handler, 250);
    
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
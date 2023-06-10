use std::io::{stdin, stdout, Read, Write};
use crate::datatypes::vector3::Vector3;
use datatypes::material::Material;
use utilities::frame_handler::FrameHandler;
use datatypes::color::Color;
use spacial::mesh::{Mesh, self};
use spacial::scene::Scene;
use spacial::camera::Camera;
use utilities::model_loader::{load_model};
use std::time::{Duration, Instant};

mod utilities {
    pub mod frame_handler;
    pub mod model_loader;
}

mod datatypes {
    pub mod color;
    pub mod vector2d;
    pub mod vector3d;
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
    pub mod bvh;
}


pub fn render_suzanne(n: u64) {
    let red = Material::new(Color::new(1.0, 0.3, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.15
    );
    let green = Material::new(Color::new(0.3, 1.0, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.15
    );
    let blue = Material::new(Color::new(0.3, 0.3, 1.0), Color::black(), 
        Color::white(), 0.0, 0.9, 0.15
    );
    let white = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 1.0, 0.0
    );
    let mirror_rough = Material::new(Color::white(), Color::black(), 
        Color::white() * 0.9, 0.0, 0.93, 1.0
    );
    let emiss_mat_1 = Material::new(Color::black(), Color::white(),
        Color::white(), 5.0, 0.0, 0.0
    );
    let glossy_white4 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.045
    );

    let ceiling = load_model("../Models/ceil.stl", white);
    let floor = load_model("../Models/floor.stl", green);
    let side1 = load_model("../Models/side1.stl", mirror_rough);
    let side2 = load_model("../Models/side2.stl", blue);
    let side3 = load_model("../Models/side3.stl", white);
    let side4 = load_model("../Models/side4.stl", red);
    let top_light = load_model("../Models/top_light.stl", emiss_mat_1);
    let suzanne = load_model("../Models/suzanne.stl", glossy_white4);

    let mut meshes: Vec<Mesh> = vec![];
    meshes.extend(ceiling);
    meshes.extend(floor);
    meshes.extend(side1);
    meshes.extend(side2);
    meshes.extend(side3);
    meshes.extend(side4);
    meshes.extend(top_light);
    meshes.extend(suzanne);

    let size_x: usize = 600;
    let size_y: usize = 400;

    let scene: Scene = Scene::new(meshes, Color::white() * 0.3);
    let camera: Camera = Camera::new(
        Vector3::new(-200.0, 0.0, 55.0),
        Vector3::new(0.0, 28.0, 0.0),
        scene, 2.2, 
        size_x, size_y,
        5, 4, 0.3, 
        10.0, 180.0, 1.3
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");
    frame_handler = camera.render_scene(frame_handler, 1);
}

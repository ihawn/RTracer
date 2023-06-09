use std::io::{stdin, stdout, Read, Write};
use crate::datatypes::vector3::Vector3;
use datatypes::material::Material;
use utilities::frame_handler::FrameHandler;
use datatypes::color::Color;
use spacial::tri::{Tri, self};
use spacial::mesh_object::MeshObject;
use spacial::scene::Scene;
use spacial::camera::Camera;
use utilities::file_utilities::{load_model};
use std::time::{Duration, Instant};

pub mod utilities {
    pub mod frame_handler;
    pub mod file_utilities;
    pub mod postprocessing;
}

pub mod datatypes {
    pub mod color;
    pub mod vector2d;
    pub mod vector3d;
    pub mod vector2;
    pub mod vector3;
    pub mod hit_point;
    pub mod material;
}

pub mod spacial {
    pub mod camera;
    pub mod scene;
    pub mod tri;  
    pub mod mesh_object;
    pub mod ray;
    pub mod bvh;
}



pub fn render_suzanne(n: u64) {
    /*let red = Material::new(Color::new(1.0, 0.3, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.15, true
    );
    let green = Material::new(Color::new(0.3, 1.0, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.15, true
    );
    let blue = Material::new(Color::new(0.3, 0.3, 1.0), Color::black(), 
        Color::white(), 0.0, 0.9, 0.15, true
    );
    let white = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 1.0, 0.0, true
    );
    let mirror_rough = Material::new(Color::white(), Color::black(), 
        Color::white() * 0.9, 0.0, 0.93, 1.0, true
    );
    let emiss_mat_1 = Material::new(Color::black(), Color::white(),
        Color::white(), 5.0, 0.0, 0.0, true
    );
    let glossy_white4 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.045, true
    );

    let ceiling = load_model("../Models/ceil.stl", white);
    let floor = load_model("../Models/floor.stl", green);
    let side1 = load_model("../Models/side1.stl", mirror_rough);
    let side2 = load_model("../Models/side2.stl", blue);
    let side3 = load_model("../Models/side3.stl", white);
    let side4 = load_model("../Models/side4.stl", red);
    let top_light = load_model("../Models/top_light.stl", emiss_mat_1);
    let suzanne = load_model("../Models/suzanne.stl", glossy_white4);

    let mut meshes: Vec<MeshObject> = vec![];
    meshes.push(MeshObject::new(ceiling, false));
    meshes.push(MeshObject::new(floor, false));
    meshes.push(MeshObject::new(side1, false));
    meshes.push(MeshObject::new(side2, false));
    meshes.push(MeshObject::new(side3, false));
    meshes.push(MeshObject::new(side4, false));
    meshes.push(MeshObject::new(top_light, false));
    meshes.push(MeshObject::new(suzanne, false));

    let size_x: usize = 600;
    let size_y: usize = 400;

    let scene: Scene = Scene::new(meshes, vec![], Color::white() * 0.3);
    let camera: Camera = Camera::new(
        Vector3::new(-200.0, 0.0, -30.0),
        Vector3::new(0.0, 6.0, 0.0),
        scene, 2.7, 
        size_x, size_y,
        3, 2, 0.3, 
        0.0, 165.0, 1.3,
        0
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");
    camera.render_scene(frame_handler, 5);*/
}

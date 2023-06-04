use std::io::{stdin, stdout, Read, Write};
use crate::datatypes::vector3::Vector3;
use datatypes::material::Material;
use utilities::frame_handler::FrameHandler;
use datatypes::color::Color;
use spacial::mesh::{Mesh, self};
use spacial::scene::Scene;
use spacial::camera::Camera;
use utilities::model_loader::{load_model};


mod utilities {
    pub mod frame_handler;
    pub mod model_loader;
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
    pub mod bvh;
}


fn main() {
    let size_x: usize = 1800;
    let size_y: usize = 1200;

    /*let red_sphere = Mesh::new_sphere(
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
        Vector3::new(525.0, -100.0, -50.0), col2_mat
    );*/

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

    let mirror = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 1.0, 1.0
    );
    let mirror_rough = Material::new(Color::white(), Color::black(), 
        Color::white() * 0.9, 0.0, 0.9, 1.0
    );

    let glossy_white = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.06
    );

    let emiss_mat_1 = Material::new(Color::black(), Color::white(),
        Color::white(), 5.0, 0.0, 0.0
    );



    let ceiling = load_model("C:/Users/Isaac/Desktop/models/ceil.stl", white);
    let floor = load_model("C:/Users/Isaac/Desktop/models/floor.stl", green);
    let side1 = load_model("C:/Users/Isaac/Desktop/models/side1.stl", mirror_rough);
    let side2 = load_model("C:/Users/Isaac/Desktop/models/side2.stl", blue);
    let side3 = load_model("C:/Users/Isaac/Desktop/models/side3.stl", white);
    let side4 = load_model("C:/Users/Isaac/Desktop/models/side4.stl", red);
    let top_light = load_model("C:/Users/Isaac/Desktop/models/top_light.stl", emiss_mat_1);
    let suzanne = load_model("C:/Users/Isaac/Desktop/models/suzanne.stl", glossy_white);


    let mut meshes: Vec<Mesh> = vec![];
    meshes.extend(ceiling);
    meshes.extend(floor);
    meshes.extend(side1);
    meshes.extend(side2);
    meshes.extend(side3);
    meshes.extend(side4);
    meshes.extend(top_light);
    meshes.extend(suzanne);

    let scene: Scene = Scene::new(meshes, Color::white() * 0.3);
    let camera: Camera = Camera::new(
        Vector3::new(-175.0, 0.0, 30.0),
        Vector3::new(0.0, 20.0, 0.0),
        scene, 1200.0, 2.3, 
        size_x, size_y,
        10, 7
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");
    frame_handler = camera.render_scene(frame_handler, 1000);
    
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
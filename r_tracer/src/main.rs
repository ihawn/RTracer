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


fn main() {

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
        Color::white() * 0.9, 0.0, 0.93, 1.0
    );
    let mirror_rough2 = Material::new(Color::white(), Color::black(), 
    Color::white() * 0.9, 0.0, 0.8, 1.0
);

    let glossy_white1 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 1.0
    );
    let glossy_white2 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.5
    );
    let glossy_white3 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.1
    );
    let glossy_white4 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.045
    );


    let red1 = Material::new(Color::new(1.0, 0.3, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.035
    );
    let yellow1 = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.035
    );
    let green1 = Material::new(Color::new(0.3, 1.0, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.035
    );
    let blue1 = Material::new(Color::new(0.3, 0.3, 1.0), Color::black(), 
        Color::white(), 0.0, 0.9, 0.035
    );


    let red2 = Material::new(Color::new(1.0, 0.3, 0.3), Color::black(), 
        Color::new(1.0, 0.3, 0.3), 0.0, 1.0, 1.0
    );
    let yellow2 = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
        Color::new(1.0, 1.0, 0.3), 0.0, 1.0, 1.0
    );
    let green2 = Material::new(Color::new(0.3, 1.0, 0.3), Color::black(), 
        Color::new(0.3, 1.0, 0.3), 0.0, 1.0, 1.0
    );
    let blue2 = Material::new(Color::new(0.3, 0.3, 1.0), Color::black(), 
        Color::new(0.3, 0.3, 1.0), 0.0, 1.0, 1.0
    );


    let mirror1 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 1.0, 1.0
    );
    let mirror2 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 0.75, 1.0
    );
    let mirror3 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 0.5, 1.0
    );
    let mirror4 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 0.3, 1.0
    );




    let emiss_mat_1 = Material::new(Color::black(), Color::white(),
        Color::white(), 5.0, 0.0, 0.0
    );



    let ceiling = load_model("C:/Users/Isaac/Desktop/models/ceil.stl", white);
    let floor = load_model("C:/Users/Isaac/Desktop/models/floor.stl", green);
    let side1 = load_model("C:/Users/Isaac/Desktop/models/side1.stl", white);//mirror_rough);
    let side2 = load_model("C:/Users/Isaac/Desktop/models/side2.stl", blue);
    let side3 = load_model("C:/Users/Isaac/Desktop/models/side3.stl", white);
    let side4 = load_model("C:/Users/Isaac/Desktop/models/side4.stl", red);
    let top_light = load_model("C:/Users/Isaac/Desktop/models/top_light.stl", emiss_mat_1);
    let suzanne = load_model("C:/Users/Isaac/Desktop/models/suzanne.stl", glossy_white4);
    let ico_sphere = load_model("C:/Users/Isaac/Desktop/models/ico.stl", mirror);

    let sphere1 = Mesh::new_sphere(10.0, -75.0, -75.0, 20.0, blue2);
    let sphere2 = Mesh::new_sphere(10.0, -25.0, -75.0, 20.0, green2);
    let sphere3 = Mesh::new_sphere(10.0, 25.0, -75.0, 20.0, yellow2);
    let sphere4 = Mesh::new_sphere(10.0, 75.0, -75.0, 20.0, red2);

    let sphere5 = Mesh::new_sphere(10.0, -75.0, -25.0, 20.0, glossy_white1);
    let sphere6 = Mesh::new_sphere(10.0, -25.0, -25.0, 20.0, glossy_white2);
    let sphere7 = Mesh::new_sphere(10.0, 25.0, -25.0, 20.0, glossy_white3);
    let sphere8 = Mesh::new_sphere(10.0, 75.0, -25.0, 20.0, glossy_white4);

    let sphere9 = Mesh::new_sphere(10.0, -75.0, 25.0, 20.0, mirror4);
    let sphere10 = Mesh::new_sphere(10.0, -25.0, 25.0, 20.0, mirror3);
    let sphere11 = Mesh::new_sphere(10.0, 25.0, 25.0, 20.0, mirror2);
    let sphere12 = Mesh::new_sphere(10.0, 75.0, 25.0, 20.0, mirror1);

    let sphere13 = Mesh::new_sphere(10.0, -75.0, 75.0, 20.0, blue1);
    let sphere14 = Mesh::new_sphere(10.0, -25.0, 75.0, 20.0, green1);
    let sphere15 = Mesh::new_sphere(10.0, 25.0, 75.0, 20.0, yellow1);
    let sphere16 = Mesh::new_sphere(10.0, 75.0, 75.0, 20.0, red1);

    let sphere = Mesh::new_sphere(0.0, 0.0, 0.0, 60.0, mirror);


    let sphere17 = Mesh::new_sphere(75.0, 0.0, 0.0, 20.0, mirror);
    let sphere18 = Mesh::new_sphere(25.0, 0.0, 0.0, 20.0, mirror);
    let sphere19 = Mesh::new_sphere(-25.0, 0.0, 0.0, 20.0, mirror);
    let sphere20 = Mesh::new_sphere(-75.0, 0.0, 0.0, 20.0, mirror);

    let mut meshes: Vec<Mesh> = vec![];
    meshes.extend(ceiling);
    meshes.extend(floor);
    meshes.extend(side1);
    meshes.extend(side2);
    meshes.extend(side3);
    meshes.extend(side4);
    meshes.extend(top_light);
    meshes.extend(suzanne);
    //meshes.extend(ico_sphere);
    /*meshes.push(sphere1);
    meshes.push(sphere2);
    meshes.push(sphere3);
    meshes.push(sphere4);
    meshes.push(sphere5);
    meshes.push(sphere6);
    meshes.push(sphere7);
    meshes.push(sphere8);
    meshes.push(sphere9);
    meshes.push(sphere10);
    meshes.push(sphere11);
    meshes.push(sphere12);
    meshes.push(sphere13);
    meshes.push(sphere14);
    meshes.push(sphere15);
    meshes.push(sphere16);*/

    /*meshes.push(sphere17);
    meshes.push(sphere18);
    meshes.push(sphere19);
    meshes.push(sphere20);*/
    

    //meshes.push(sphere);

    let size_x: usize = 1200;
    let size_y: usize = 800;

    let scene: Scene = Scene::new(meshes, Color::white() * 0.3);
    let camera: Camera = Camera::new(
        Vector3::new(-200.0, 0.0, 55.0),
        Vector3::new(0.0, 28.0, 0.0),
        scene, 2.2, 
        size_x, size_y,
        3, 4, 0.2, 
        20.0, 180.0, 1.25
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");

    let start_time = Instant::now();
    frame_handler = camera.render_scene(frame_handler, 20);
    let elapsed_time = start_time.elapsed();

    let hours = elapsed_time.as_secs() / 3600;
    let minutes = (elapsed_time.as_secs() % 3600) / 60;
    let seconds = elapsed_time.as_secs() % 60;
    let milliseconds = elapsed_time.subsec_millis();
    println!(
        "Elapsed time: {:02}:{:02}:{:02}:{:03}",
        hours, minutes, seconds, milliseconds
    );
    
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
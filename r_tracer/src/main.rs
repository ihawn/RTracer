use std::io::{stdin, stdout, Read, Write};
use r_tracer::datatypes::vector3::Vector3;
use r_tracer::datatypes::material::Material;
use r_tracer::utilities::frame_handler::FrameHandler;
use r_tracer::datatypes::color::Color;
use r_tracer::spacial::mesh::{Mesh, self};
use r_tracer::spacial::scene::Scene;
use r_tracer::spacial::camera::Camera;
use r_tracer::spacial::mesh_object::MeshObject;
use r_tracer::utilities::model_loader::{load_model};
use std::time::{Duration, Instant};




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
    let yellow = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
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
        Color::white(), 0.0, 1.0, 0.05
    );


    let red1 = Material::new(Color::new(0.6, 0.3, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07
    );
    let yellow1 = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07
    );
    let green1 = Material::new(Color::new(0.3, 0.6, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07
    );
    let blue1 = Material::new(Color::new(0.3, 0.3, 0.6), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07
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
    let emiss_mat_2 = Material::new(Color::black(), Color::white(),
    Color::white(), 0.2, 0.0, 0.0
);

    let glass = Material::new_dieletric(Color::white(), 1.0, 1.5);
    let water = Material::new_dieletric(Color::white(), 1.0, 1.333);
    let blue_glass = Material::new_dieletric(Color::new(0.8, 0.8, 1.0), 1.0, 1.5);


    let light_ball = load_model("../Models/light_ball.stl", emiss_mat_1);
    //let fluid = load_model("../Models/fluid.stl", water);
    let test_tris = load_model("../Models/test_tris.stl", mirror);
    let suzanne_noeyes = load_model("../Models/suzanne_noeyes.stl", yellow1);
    let suzanne_eyes = load_model("../Models/suzanne_eyes.stl", emiss_mat_1);
    let suzanne = load_model("../Models/suzanne.stl", yellow1);
    let ceiling = load_model("../Models/ceil.stl", white);
    let floor = load_model("../Models/floor.stl", glossy_white4);//green);
    let side1 = load_model("../Models/side1.stl", green);//yellow);//mirror_rough);
    let side2 = load_model("../Models/side2.stl", blue);
    let side3 = load_model("../Models/side3.stl", white);
    let side4 = load_model("../Models/side4.stl", red);
    let top_light = load_model("../Models/top_light.stl", emiss_mat_1);
    let ico_sphere = load_model("../Models/ico.stl", mirror);

    let dave1 = load_model("../Models/dave1.stl", blue2);
    let dave2 = load_model("../Models/dave2.stl", green2);
    let dave3 = load_model("../Models/dave3.stl", red2);
    let dave4 = load_model("../Models/dave4.stl", glass);
    let dave5 = load_model("../Models/dave5.stl", blue);
    let dave6 = load_model("../Models/dave6.stl", green);
    let dave7 = load_model("../Models/dave7.stl", red);
    let dave8 = load_model("../Models/dave8.stl", mirror);
    let dave9 = load_model("../Models/dave9.stl", red);
    let dave10 = load_model("../Models/dave10.stl", yellow);
    let dave11 = load_model("../Models/dave11.stl", mirror);
    let dave12 = load_model("../Models/dave12.stl", mirror);
    let dave13 = load_model("../Models/dave13.stl", blue_glass);
    let dave14 = load_model("../Models/dave14.stl", blue2);
    let dave_cube = load_model("../Models/dave_cube.stl", emiss_mat_1);
    let cubes = load_model("../Models/cubes.stl", emiss_mat_1);

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
    let sphere11 = Mesh::new_sphere(10.0, 25.0, 25.0, 20.0, glass);
    let sphere12 = Mesh::new_sphere(10.0, 75.0, 25.0, 20.0, mirror1);

    let sphere13 = Mesh::new_sphere(10.0, -75.0, 75.0, 20.0, blue1);
    let sphere14 = Mesh::new_sphere(10.0, -25.0, 75.0, 20.0, green1);
    let sphere15 = Mesh::new_sphere(10.0, 25.0, 75.0, 20.0, yellow1);
    let sphere16 = Mesh::new_sphere(10.0, 75.0, 75.0, 20.0, red1);

    let sphere = Mesh::new_sphere(0.0, 0.0, 0.0, 60.0, mirror);


    let sphere17 = Mesh::new_sphere(75.0, 0.0, 0.0, 25.0, glass);
    let sphere18 = Mesh::new_sphere(25.0, 0.0, 0.0, 20.0, red);
    let sphere19 = Mesh::new_sphere(-25.0, 0.0, 0.0, 15.0, blue);
    let sphere20 = Mesh::new_sphere(-75.0, 0.0, 0.0, 10.0, green);

    let mut spheres: Vec<Mesh> = vec![];


    let mut meshes: Vec<MeshObject> = vec![];
    meshes.push(MeshObject::new(ceiling, false));
    meshes.push(MeshObject::new(floor, false));
    meshes.push(MeshObject::new(side1, false));
    meshes.push(MeshObject::new(side2, false));
    meshes.push(MeshObject::new(side3, false));
    meshes.push(MeshObject::new(side4, false));
    meshes.push(MeshObject::new(top_light, false));
    meshes.push(MeshObject::new(suzanne, true));
    //meshes.push(MeshObject::new(fluid, true));
    meshes.push(MeshObject::new(light_ball, true));
    //meshes.push(MeshObject::new(suzanne_eyes));
    //meshes.push(MeshObject::new(suzanne_noeyes));

    //meshes.push(MeshObject::new(dave8));


    let size_x: usize = 1200;
    let size_y: usize = 800;

    let scene: Scene = Scene::new(meshes, spheres, Color::white() * 0.3);
    let camera: Camera = Camera::new(
        Vector3::new(-200.0, 0.0, -30.0),
        Vector3::new(0.0, 6.0, 0.0),
        scene, 2.4, 
        size_x, size_y,
        15, 5, 0.3, 
        0.0, 165.0, 1.3
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");

    let start_time = Instant::now();
    frame_handler = camera.render_scene(frame_handler, 50000);
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
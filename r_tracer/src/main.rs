use std::io::{stdin, stdout, Read, Write};
use r_tracer::datatypes::vector3::Vector3;
use r_tracer::datatypes::material::Material;
use r_tracer::utilities::frame_handler::FrameHandler;
use r_tracer::datatypes::color::Color;
use r_tracer::datatypes::vector2d::Vector2D;
use r_tracer::spacial::mesh::Mesh;
use r_tracer::spacial::scene::Scene;
use r_tracer::spacial::camera::Camera;
use r_tracer::spacial::mesh_object::MeshObject;
use r_tracer::utilities::tracer_io::{load_model, save_vector2d_as_png};
use std::time::Instant;




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
        Color::white(), 0.0, 0.9, 0.15, 0.0, 0.0, true
    );
    let green = Material::new(Color::new(0.3, 1.0, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.15, 0.0, 0.0,true
    );
    let blue = Material::new(Color::new(0.3, 0.3, 1.0), Color::black(), 
        Color::white(), 0.0, 0.9, 0.15, 0.0, 0.0, true
    );
    let yellow = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.15, 0.0, 0.0, true
    );
    let white = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 1.0, 0.0, 0.0, 0.0, true
    );

    let mirror = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let mirror_rough = Material::new(Color::white(), Color::black(), 
        Color::white() * 0.9, 0.0, 0.93, 1.0, 0.0, 0.0, true
    );
    let mirror_rough2 = Material::new(Color::white(), Color::black(), 
    Color::white() * 0.9, 0.0, 0.8, 1.0, 0.0, 0.0, true
);

    let glossy_white1 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let glossy_white2 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.5, 0.0, 0.0, true
    );
    let glossy_white3 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.1, 0.0, 0.0, true
    );
    let glossy_white4 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.05, 0.0, 0.0, true
    );


    let red1 = Material::new(Color::new(0.6, 0.3, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07, 0.0, 0.0, true
    );
    let yellow1 = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07, 0.0, 0.0, true
    );
    let green1 = Material::new(Color::new(0.3, 0.6, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07, 0.0, 0.0, true
    );
    let blue1 = Material::new(Color::new(0.3, 0.3, 0.6), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07, 0.0, 0.0, true
    );


    let red2 = Material::new(Color::new(1.0, 0.3, 0.3), Color::black(), 
        Color::new(1.0, 0.3, 0.3), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let yellow2 = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
        Color::new(1.0, 1.0, 0.3), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let green2 = Material::new(Color::new(0.3, 1.0, 0.3), Color::black(), 
        Color::new(0.3, 1.0, 0.3), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let blue2 = Material::new(Color::new(0.3, 0.3, 1.0), Color::black(), 
        Color::new(0.3, 0.3, 1.0), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );


    let mirror1 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let mirror2 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 0.75, 1.0, 0.0, 0.0, true
    );
    let mirror3 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 0.5, 1.0, 0.0, 0.0, true
    );
    let mirror4 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 0.3, 1.0, 0.0, 0.0, true
    );




    let emiss_mat_1 = Material::new(Color::black(), Color::white(),
        Color::white(), 5.0, 0.0, 0.0, 0.0, 0.0, true
    );
    let emiss_mat_2 = Material::new(Color::black(), Color::white(),
        Color::white(), 0.2, 0.0, 0.0, 0.0, 0.0, true
    );

    let emiss_mat_red = Material::new(Color::black(), Color::new(1.0, 0.3, 0.3),
    Color::black(), 5.0, 0.0, 0.0, 0.0, 0.0, true
    );

    let emiss_mat_green = Material::new(Color::black(), Color::new(0.3, 1.0, 0.3),
    Color::black(), 5.0, 0.0, 0.0, 0.0, 0.0, true
    );

    let emiss_mat_blue = Material::new(Color::black(), Color::new(0.3, 0.3, 1.0),
    Color::black(), 5.0, 0.0, 0.0, 0.0, 0.0, true
    );

    let glass = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 0.9, 0.05, 0.95, 1.5, true
    );
    /*let frosted_glass = Material::new_dieletric(Color::white(), 0.9, 1.5);
    let water = Material::new_dieletric(Color::white(), 1.0, 1.333);
    let blue_glass = Material::new_dieletric(Color::new(0.8, 0.8, 1.0), 1.0, 1.5);*/


    let light_ball = load_model("../Models/light_ball.stl", emiss_mat_2);
    //let fluid = load_model("../Models/fluid.stl", glass);
    //let test_tris = load_model("../Models/test_tris.stl", mirror);
    //let suzanne_noeyes = load_model("../Models/suzanne_noeyes.stl", yellow1);
    //let suzanne_eyes = load_model("../Models/suzanne_eyes.stl", emiss_mat_1);
    let suzanne = load_model("../Models/suzanne.stl", glass);
    let test_plane = load_model("../Models/test_plane.stl", emiss_mat_1);
    //let fluid_splash = load_model("../Models/fluid_splash.stl", glass);
    let ceiling = load_model("../Models/ceil.stl", white);
    let floor = load_model("../Models/floor.stl", glossy_white4);//green);
    let side1 = load_model("../Models/side1.stl", green);//yellow);//mirror_rough);
    let side2 = load_model("../Models/side2.stl", blue);
    let side3 = load_model("../Models/side3.stl", white);
    let side4 = load_model("../Models/side4.stl", red);
    let top_light = load_model("../Models/top_light.stl", emiss_mat_1);
    let top_light1 = load_model("../Models/top_light1.stl", emiss_mat_red);
    let top_light2 = load_model("../Models/top_light2.stl", emiss_mat_green);
    let top_light3 = load_model("../Models/top_light3.stl", emiss_mat_blue);
    let bot_light = load_model("../Models/bot_light.stl", emiss_mat_1);
    let ico_sphere = load_model("../Models/ico.stl", mirror);


    let mut spheres: Vec<Mesh> = vec![];


    let mut meshes: Vec<MeshObject> = vec![];
    /*meshes.push(MeshObject::new(ceiling, false));
    meshes.push(MeshObject::new(floor, false));
    meshes.push(MeshObject::new(side1, false));
    meshes.push(MeshObject::new(side2, false));
    meshes.push(MeshObject::new(side3, false));
    meshes.push(MeshObject::new(side4, false));
    meshes.push(MeshObject::new(top_light, false));*/
    meshes.push(MeshObject::new(bot_light, false));
    /*meshes.push(MeshObject::new(top_light1, false));
    meshes.push(MeshObject::new(top_light2, false));
    meshes.push(MeshObject::new(top_light3, false));*/
    //meshes.push(MeshObject::new(suzanne, true));
    meshes.push(MeshObject::new(fluid_splash, true));
    //meshes.push(MeshObject::new(test_plane, true));
    //meshes.push(MeshObject::new(fluid, true));
    //meshes.push(MeshObject::new(light_ball, true));
    //meshes.push(MeshObject::new(suzanne_eyes));
    //meshes.push(MeshObject::new(suzanne_noeyes));

    //meshes.push(MeshObject::new(dave8));


    let size_x: usize = 1200;
    let size_y: usize = 800;

    let scene: Scene = Scene::new(meshes, spheres, Color::white() * 0.0);
    let camera: Camera = Camera::new(
        Vector3::new(-200.0, 0.0, -50.0),
        Vector3::new(0.0, -11.0, 0.0),
        scene, 2.7, 
        size_x, size_y,
        3, 3, 0.3, 
        0.0, 165.0, 1.3,
        32
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");

    let start_time = Instant::now();
    let frame: Vector2D<Color> = camera.render_scene(frame_handler, 100);
    let elapsed_time = start_time.elapsed();

    let hours = elapsed_time.as_secs() / 3600;
    let minutes = (elapsed_time.as_secs() % 3600) / 60;
    let seconds = elapsed_time.as_secs() % 60;
    let milliseconds = elapsed_time.subsec_millis();
    println!(
        "Elapsed time: {:02}:{:02}:{:02}:{:03}",
        hours, minutes, seconds, milliseconds
    );
    
    save_vector2d_as_png(&frame, "../Renders/render.png");
}

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
use std::io::{stdin, stdout, Read, Write};
use r_tracer::datatypes::vector3::Vector3;
use r_tracer::datatypes::material::Material;
use r_tracer::utilities::frame_handler::FrameHandler;
use r_tracer::datatypes::color::Color;
use r_tracer::datatypes::vector2d::Vector2D;
use r_tracer::spacial::scene::Scene;
use r_tracer::spacial::camera::Camera;
use r_tracer::spacial::mesh_object::MeshObject;
use r_tracer::utilities::file_utilities::{load_model, save_vector2d_as_png, import_texture};
use std::time::Instant;




fn main() {

    let emiss_mat_1 = Material::new(Color::black(), Color::white(),
        Color::white(), Color::black(), 1.0, 0.0, 0.0, 0.0, 
        0.0, 0.0, true, None, None, None, 
        None, None, None, None
    );


    let brick = Material::new(Color::black(), Color::black(),
        Color::black(),  Color::black(),
        0.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, true, Some(0), None, None, 
        None, Some(1), Some(2), Some(3)
    );


    let top_light = load_model("../Models/light1.stl", emiss_mat_1);
    let brick_sphere = load_model("../Models/brick_sphere.obj", brick);

    let mut meshes: Vec<MeshObject> = vec![];
    meshes.push(MeshObject::new(top_light, false));
    meshes.push(MeshObject::new(brick_sphere, true));



    let size_x: usize = 1200;
    let size_y: usize = 800;

    let mut maps: Vec<Vector2D<Color>> = vec![];
    let test_uv: Vector2D<Color> = import_texture("../Textures/uv_test.jpg");
    
    let brick_col: Vector2D<Color> = import_texture("../Textures/Bricks01/Bricks01_COL_VAR1_3K.jpg");
    let brick_normal: Vector2D<Color> = import_texture("../Textures/Bricks01/Bricks01_NRM_3K.jpg");
    let brick_smoothness: Vector2D<Color> = import_texture("../Textures/Bricks01/Bricks01_GLOSS_3K.jpg");
    let brick_specular: Vector2D<Color> = import_texture("../Textures/Bricks01/Bricks01_REFL_3K.jpg");

    maps.push(brick_col);
    maps.push(brick_normal);
    maps.push(brick_smoothness);
    maps.push(brick_specular);


    let scene: Scene = Scene::new(meshes, maps, Color::white() * 0.3);
    let camera: Camera = Camera::new(
        Vector3::new(-200.0, 0.0, 50.0),
        Vector3::new(0.0, 15.0, 0.0),
        scene, 2.8, 
        size_x, size_y,
        30, 3, 0.3, 
        0.0, 192.0, 1.3,
        0
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");

    let start_time = Instant::now();
    let frame: Vector2D<Color> = camera.render_scene(frame_handler, 300);
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

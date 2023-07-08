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
        0.0, 0.0, false, None, None, None, 
        None, None, None, None
    );


    let brick = Material::new(Color::black(), Color::black(),
        Color::black(),  Color::black(),
        0.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, true, Some(0), None, Some(0), 
        None, Some(1), Some(2), Some(3)
    );

    let stud_metal = Material::new(Color::black(), Color::black(),
        Color::black(),  Color::black(),
        0.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, true, Some(4), None, Some(4), 
        None, Some(5), Some(6), Some(7)
    );

    let mud = Material::new(Color::black(), Color::black(),
        Color::black(),  Color::black(),
        0.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, true, Some(8), None, Some(8), 
        None, Some(9), Some(10), Some(11)
    );

    let leaves = Material::new(Color::black(), Color::black(),
        Color::black(),  Color::black(),
        0.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, true, Some(12), None, Some(12), 
        None, Some(13), Some(14), Some(15)
    );

    let brick_sphere = load_model("../Models/brick_sphere.obj", brick);
    let stud_metal_sphere = load_model("../Models/stud_metal_sphere.obj", stud_metal);
    let mud_sphere = load_model("../Models/mud_sphere.obj", mud);
    let leaf_sphere = load_model("../Models/leaf_sphere.obj", leaves);

    let mut meshes: Vec<MeshObject> = vec![];
    meshes.push(MeshObject::new(brick_sphere, true));
    meshes.push(MeshObject::new(stud_metal_sphere, true));
    meshes.push(MeshObject::new(mud_sphere, true));
    meshes.push(MeshObject::new(leaf_sphere, true));



    let size_x: usize = 1200;
    let size_y: usize = 800;

    let environment_map: Vector2D<Color> = import_texture("../Textures/HDRIOutdoor/HdrOutdoorResidentialRiverwalkAfternoonClear001_JPG_3K_blur.jpg");

    let mut maps: Vec<Vector2D<Color>> = vec![];
    
    let brick_col: Vector2D<Color> = import_texture("../Textures/Bricks01/Bricks01_COL_VAR1_3K.jpg");
    let brick_normal: Vector2D<Color> = import_texture("../Textures/Bricks01/Bricks01_NRM_3K.jpg");
    let brick_smoothness: Vector2D<Color> = import_texture("../Textures/Bricks01/Bricks01_GLOSS_3K.jpg");
    let brick_specular: Vector2D<Color> = import_texture("../Textures/Bricks01/Bricks01_REFL_3K.jpg");

    let stud_metal_col: Vector2D<Color> = import_texture("../Textures/MetalPlateStudded001/MetalPlateStudded001_GLOSS_3K_SPECULAR.png");
    let stud_metal_normal: Vector2D<Color> = import_texture("../Textures/MetalPlateStudded001/MetalPlateStudded001_NRM_3K_SPECULAR.png");
    let stud_metal_smoothness: Vector2D<Color> = import_texture("../Textures/MetalPlateStudded001/MetalPlateStudded001_GLOSS_3K_SPECULAR.png");
    let stud_metal_specular: Vector2D<Color> = import_texture("../Textures/MetalPlateStudded001/MetalPlateStudded001_REFL_3K_SPECULAR.png");

    let mud_col: Vector2D<Color> = import_texture("../Textures/GroundTireTracksWet002/GroundTireTracksWet002_COL_3K.jpg");
    let mud_normal: Vector2D<Color> = import_texture("../Textures/GroundTireTracksWet002/GroundTireTracksWet002_NRM_3K.jpg");
    let mud_smoothness: Vector2D<Color> = import_texture("../Textures/GroundTireTracksWet002/GroundTireTracksWet002_GLOSS_3K.jpg");
    let mud_specular: Vector2D<Color> = import_texture("../Textures/GroundTireTracksWet002/GroundTireTracksWet002_REFL_3K.jpg");

    let leaf_col: Vector2D<Color> = import_texture("../Textures/LeavesClusterFall001/LeavesClusterFall001_COL_3K.jpg");
    let leaf_normal: Vector2D<Color> = import_texture("../Textures/LeavesClusterFall001/LeavesClusterFall001_NRM_3K.jpg");
    let leaf_smoothness: Vector2D<Color> = import_texture("../Textures/LeavesClusterFall001/LeavesClusterFall001_GLOSS_3K.jpg");
    let leaf_specular: Vector2D<Color> = import_texture("../Textures/LeavesClusterFall001/LeavesClusterFall001_REFL_3K.jpg");

    maps.push(brick_col);
    maps.push(brick_normal);
    maps.push(brick_smoothness);
    maps.push(brick_specular);

    maps.push(stud_metal_col);
    maps.push(stud_metal_normal);
    maps.push(stud_metal_smoothness);
    maps.push(stud_metal_specular);

    maps.push(mud_col);
    maps.push(mud_normal);
    maps.push(mud_smoothness);
    maps.push(mud_specular);

    maps.push(leaf_col);
    maps.push(leaf_normal);
    maps.push(leaf_smoothness);
    maps.push(leaf_specular);



    let scene: Scene = Scene::new(meshes, maps, Color::white() * 0.3, Some(environment_map));
    let camera: Camera = Camera::new(
        Vector3::new(200.0, 0.0, 50.0),
        Vector3::new(0.0, 15.0, 180.0),
        scene, 2.8, 
        size_x, size_y,
        30, 3, 0.3, 
        0.0, 192.0, 1.3,
        0
    );

    let frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");

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

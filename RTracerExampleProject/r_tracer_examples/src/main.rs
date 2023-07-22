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
    mud_render();
}


fn mud_render() {
    let mud = Material::new(Color::black(), Color::black(),
        Color::black(),  Color::black(),
        0.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, true, Some(0), None, Some(0), 
        None, Some(1), Some(2), Some(3)
    );

    let glassy = Material::new(Color::white() * 0.9, Color::black(),
        Color::white(),  Color::white(),
        0.0, 0.95, 0.25, 0.75, 
        1.5, 0.0, true, None, None, None, 
        None, Some(4), Some(5), None
    );

    let sun = Material::new(Color::black(), Color::new(1.0, 0.9, 0.8),
        Color::black(),  Color::black(),
        1.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, true, None, None, None, 
        None, None, None, None
    );

    let mud_plane = load_model("../Models/mud_tracks.obj", mud);
    let suzanne = load_model("../Models/suzanne_mud.obj", glassy);
    let light = load_model("../Models/giant_light.obj", sun);

    let mut meshes: Vec<MeshObject> = vec![];
    meshes.push(MeshObject::new(mud_plane, true));
    meshes.push(MeshObject::new(suzanne, true));
    meshes.push(MeshObject::new(light, false));


    let size_x: usize = 1800;
    let size_y: usize = 1200;

    let environment_map: Vector2D<Color> = import_texture("../Textures/HDRIOutdoor/HdrOutdoorResidentialRiverwalkAfternoonClear001_JPG_3K.jpg");

    let mut maps: Vec<Vector2D<Color>> = vec![];

    let mud_col: Vector2D<Color> = import_texture("../Textures/GroundTireTracksWet002/GroundTireTracksWet002_COL_3K.jpg");
    let mud_normal: Vector2D<Color> = import_texture("../Textures/GroundTireTracksWet002/GroundTireTracksWet002_NRM_3K.jpg");
    let mud_smoothness: Vector2D<Color> = import_texture("../Textures/GroundTireTracksWet002/GroundTireTracksWet002_GLOSS_3K.jpg");
    let mud_specular: Vector2D<Color> = import_texture("../Textures/GroundTireTracksWet002/GroundTireTracksWet002_GLOSS_3K.jpg");

    let scratch_normal: Vector2D<Color> = import_texture("../Textures/DirtWipes020/DirtWipes020_NRM_3K.jpg");
    let scratch_smoothness: Vector2D<Color> = import_texture("../Textures/DirtWipes020/DirtWipes020_OVERLAY_VAR2_3K.jpg");

    maps.push(mud_col);
    maps.push(mud_normal);
    maps.push(mud_smoothness);
    maps.push(mud_specular);
    maps.push(scratch_normal);
    maps.push(scratch_smoothness);


    let scene: Scene = Scene::new(meshes, maps, Color::white() * 0.3,  Some(environment_map));
    let camera: Camera = Camera::new(
        Vector3::new(53.27, 53.27, 15.0),
        Vector3::new(0.0, 15.0, -135.0),
        scene, 2.8, 
        size_x, size_y,
        30, 3, 0.3, 
        0.7, 24.0, 1.3,
        0
    );

    let frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");

    let start_time = Instant::now();
    let frame: Vector2D<Color> = camera.render_scene(frame_handler,  1000);
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


fn rock_render() {
    let rock = Material::new(Color::black(), Color::black(),
        Color::black(),  Color::black(),
        0.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, true, Some(0), None, Some(0), 
        None, Some(1), Some(2), Some(3)
    );

    let rock_sphere = load_model("../Models/mud_tracks.obj", rock);

    let mut meshes: Vec<MeshObject> = vec![];
    meshes.push(MeshObject::new(rock_sphere, true));


    let size_x: usize = 1200;
    let size_y: usize = 800;

    let environment_map: Vector2D<Color> = import_texture("../Textures/HDRIOutdoor/HdrOutdoorResidentialRiverwalkAfternoonClear001_JPG_3K.jpg");

    let mut maps: Vec<Vector2D<Color>> = vec![];

    let rock_col: Vector2D<Color> = import_texture("../Textures/CliffGreyChunky008/CliffGreyChunky008_COL_VAR1_3K.jpg");
    let rock_normal: Vector2D<Color> = import_texture("../Textures/CliffGreyChunky008/CliffGreyChunky008_NRM_3K.jpg");
    let rock_smoothness: Vector2D<Color> = import_texture("../Textures/CliffGreyChunky008/CliffGreyChunky008_GLOSS_3K.jpg");
    let rock_specular: Vector2D<Color> = import_texture("../Textures/CliffGreyChunky008/CliffGreyChunky008_REFL_3K.jpg");

    maps.push(rock_col);
    maps.push(rock_normal);
    maps.push(rock_smoothness);
    maps.push(rock_specular);


    let scene: Scene = Scene::new(meshes, maps, Color::white() * 0.3,  Some(environment_map));
    let camera: Camera = Camera::new(
        Vector3::new(-200.0, 0.0, 50.0),
        Vector3::new(0.0, 15.0, 0.0),
        scene, 2.8, 
        size_x, size_y,
        30, 3, 0.3, 
        0.0, 0.0, 1.3,
        0
    );

    let frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");

    let start_time = Instant::now();
    let frame: Vector2D<Color> = camera.render_scene(frame_handler,  5);
    let elapsed_time = start_time.elapsed();

    let hours = elapsed_time.as_secs() / 3600;
    let minutes = (elapsed_time.as_secs() % 3600) / 60;
    let seconds = elapsed_time.as_secs() % 60;
    let milliseconds = elapsed_time.subsec_millis();
    println!(
        "Elapsed time: {:02}:{:02}:{:02}:{:03}",
        hours, minutes, seconds, milliseconds
    );

    save_vector2d_as_png(&frame, "../Renders/rock_render.png");
}
# RTracer (Rust Tracer)

- This is a ray tracer written in rust from scratch
- Still under active development

  
![Mud](https://github.com/ihawn/RTracer/blob/main/Renders/mud.png)

# Getting Started
Example usage:

```rust
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


let rock = Material::new(Color::black(), Color::black(),
    Color::black(),  Color::black(),
    0.0, 0.0, 0.0, 0.0, 
    0.0, 1.0, true, Some(0), None, Some(0), 
    None, Some(1), Some(2), Some(3)
);

let rock_sphere = load_model("../Models/rock_sphere.obj", rock);

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
    0.0, 192.0, 1.3,
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

save_vector2d_as_png(&frame, "../Renders/render.png");
```


### Makes some pretty pictures:

  
![Splash](https://github.com/ihawn/RTracer/blob/main/Renders/splash.png)

![Brick](https://github.com/ihawn/RTracer/blob/main/Renders/brick.png)

use std::io::{stdin, stdout, Read, Write};
use std::thread;
use std::time::Duration;
use crate::datatypes::vector3::Vector3;

mod utilities {
    pub mod frame_handler;
}

mod datatypes {
    pub mod color;
    pub mod vector2d;
    pub mod vector2;
    pub mod vector3;
}

mod spacial {
    pub mod camera;
    pub mod scene;
    pub mod sphere;
}

use utilities::frame_handler::FrameHandler;
use datatypes::color::Color;
use datatypes::vector2d::Vector2D;
use spacial::sphere::Sphere;
use spacial::scene::Scene;
use spacial::camera::Camera;

fn main() {
    let size_x: usize = 480;
    let size_y: usize = 320;

    let sphere: Sphere = Sphere::new(700.0, 0.0, 150.0, 120.0);
    let scene: Scene = Scene::new(sphere);
    let mut camera: Camera = Camera::new(size_x, size_y, scene);

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");
    let mut colors = camera.render_scene();

    for i in (0..200){
        camera.scene.sphere.center.x -= 6.0;
        let p = camera.scene.sphere.center;
        println!("({}, {}, {})", p.x, p.y, p.z);

        colors = camera.render_scene();

        let converted_values: Vec<u32> = FrameHandler::buffer_from_color_vec(&colors);
        let _update = frame_handler.window.update_with_buffer(
            &converted_values,
            frame_handler.size_x, frame_handler.size_y
        );


        //thread::sleep(Duration::from_millis(25));
    }
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
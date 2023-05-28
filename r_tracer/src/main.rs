use std::io::{stdin, stdout, Read, Write};

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
    let size_y: usize = 480;

    let sphere: Sphere = Sphere::new(-5.0, 0.0, 0.0, 20.0);
    let scene: Scene = Scene::new(sphere);
    let camera: Camera = Camera::new(size_x, size_y, scene);
    let frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");

    let renderer = camera.render_scene(frame_handler);
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
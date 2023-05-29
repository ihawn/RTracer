use crate::datatypes::vector3::{Vector3, self};
use crate::datatypes::color::Color;
use crate::datatypes::vector2d::Vector2D;
use crate::utilities::frame_handler::FrameHandler;
use crate::spacial::scene::Scene;

#[derive(Copy, Clone)]
pub struct Camera {
    pub position: Vector3,
    pub scene: Scene,
    pub projection_distance: f64,
    pub width: usize,
    pub height: usize,
    pub rotation: Vector3
}

impl Camera {
    pub fn new(width: usize, height: usize, scene: Scene) -> Camera {
        Camera {
             position: Vector3::new(-100.0, 0.0, 0.0),
             scene: scene,
             projection_distance: 400.0,
             width: width,
             height: height,
             rotation: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn render_scene(self) -> Vector2D<Color> {
        let black: Color = Color::new(0, 0, 0);
        let mut frame: Vector2D<Color> = Vector2D::new(
            self.width, 
            self.height, 
            black
        );

        for x in 0..frame.height {
            for y in 0..frame.width {
                frame.set(x, y, self.cast_ray(x, y));
            }
        }

        frame
    }

    fn cast_ray(self, x: usize, y: usize) -> Color {

        let projection_point: Vector3 = Vector3::new(
            self.projection_distance,
            y as f64 - (self.width as f64)/2.0, 
            (self.height as f64)/2.0 - x as f64
        ).rot(self.rotation);

        let r: f64 = self.scene.sphere.radius;
        let object_direction: Vector3 = self.position - self.scene.sphere.center;

        let a: f64 = projection_point.self_dot();
        let b: f64 = 2.0*object_direction*projection_point;
        let c: f64 = object_direction.square().component_add() - r*r ;
        
        let desc: f64 = b*b - 4.0*a*c;

        if desc >= 0.0 {
            let t1: f64 = (-b + desc.sqrt()) / (2.0 * a);
            let t2: f64 = (-b - desc.sqrt()) / (2.0 * a);
            let pt1: Vector3 = Vector3::new(
                projection_point.x * t1,
                projection_point.y * t1,
                projection_point.z * t1
            );
            let pt2: Vector3 = Vector3::new(
                projection_point.x * t2,
                projection_point.y * t2,
                projection_point.z * t2
            );

            let d1 = self.position.distance(pt1);
            let d2 = self.position.distance(pt2);
            let mut shade: u8 = 0;

            if pt1.x > self.position.x + self.projection_distance &&
                pt2.x > self.position.x + self.projection_distance {
                    shade = 255;
                }

            Color::new(shade, 0, 0)
        } else {
            Color::new(0, 0, 0)
        }
    }
}

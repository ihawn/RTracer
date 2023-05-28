use crate::datatypes::vector3::Vector3;
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
    pub height: usize
}

impl Camera {
    pub fn new(width: usize, height: usize, scene: Scene) -> Camera {
        Camera {
             position: Vector3::new(-10.0, 0.0, 0.0),
             scene: scene,
             projection_distance: 15.0,
             width: width,
             height: height
        }
    }

    pub fn render_scene(self, handler: FrameHandler) -> FrameHandler {
        let black: Color = Color::new(0, 0, 0);
        let mut frame: Vector2D<Color> = Vector2D::new(
            self.width, 
            self.height, 
            black
        );

        for x in 0..frame.width {
            for y in 0..frame.height {
                frame.set(x, y, self.cast_ray(x, y));
            }
        }

        handler.update_window(&frame)
    }

    fn cast_ray(self, x: usize, y: usize) -> Color {
        let projection_point: Vector3 = Vector3::new(
            self.projection_distance,
            x as f64 - (self.width as f64)/2.0,
            y as f64 - (self.height as f64)/2.0
        );

        let object_direction: Vector3 = self.position - self.scene.sphere.center;

        let a: f64 = projection_point.self_dot();
        let b: f64 = 2.0*object_direction*projection_point;
        let c: f64 = object_direction.square().component_add() - self.scene.sphere.radius;
        
        let desc: f64 = b*b - 4.0*a*c;

        if desc >= 0.0 {
            let t: f64 = (-b + desc.sqrt()) / (2.0 * a);
            let shade: u8 = (t * 300.0).round() as u8;
            Color::new(shade, 0, 0)
        } else {
            Color::new(0, 0, 0)
        }
    }
}

fn square(num: f64) -> f64 { num * num }

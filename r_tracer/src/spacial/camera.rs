use crate::datatypes::vector3::{Vector3, self};
use crate::datatypes::color::Color;
use crate::datatypes::vector2d::Vector2D;
use crate::utilities::frame_handler::FrameHandler;
use crate::spacial::scene::Scene;
use crate::datatypes::hit_point::HitPoint;

#[derive(Clone)]
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
                frame.set(x, y, Self::cast_ray(self.clone(), x, y));
            }
        }

        frame
    }

    fn cast_ray(camera: Camera, x: usize, y: usize) -> Color {

        let mut hit_points: Vec<HitPoint> = Vec::new();
        for sphere in camera.scene.spheres {
            let projection_point: Vector3 = Vector3::new(
                camera.projection_distance + camera.position.x,
                y as f64 - (camera.width as f64)/2.0 + camera.position.y, 
                (camera.height as f64)/2.0 - x as f64 + camera.position.z
            ).rot(camera.rotation);

            let r: f64 = sphere.radius;
            let object_direction: Vector3 = camera.position - sphere.center;

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
                ) + camera.position;
                let pt2: Vector3 = Vector3::new(
                    projection_point.x * t2,
                    projection_point.y * t2,
                    projection_point.z * t2
                ) + camera.position;
    
                hit_points.push(
                    HitPoint::new(pt1, camera.position, projection_point, sphere)
                );
                hit_points.push(
                    HitPoint::new(pt2, camera.position, projection_point, sphere)
                );
            }
        }

        if hit_points.len() > 0 {
            let closest_hit: HitPoint = Self::closest_front_hit_point(hit_points);
            return closest_hit.object.material.color
        }

        Color::new(0, 0, 0)
    }

    fn closest_front_hit_point(hit_points: Vec<HitPoint>) -> HitPoint {
        let mut min_dist: f64 = hit_points[0].point.distance(hit_points[0].camera_origion);
        let mut min_i: usize = 0;
        for i in (1..hit_points.len()) {
            //still need to make sure hit point is in front of camera
            //for now we assume that it is
            let dist = hit_points[i].point.distance(hit_points[1].camera_origion);
            if dist < min_dist {
                min_i = i;
                min_dist = dist;
            }
        }

        hit_points[min_i]
    }

}

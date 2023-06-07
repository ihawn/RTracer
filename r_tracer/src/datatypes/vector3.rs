use std::f64::consts::PI;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::datatypes::color::Color;
use crate::datatypes::vector2::Vector2;
use std::ops::MulAssign;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x: x, y: y, z: z}
    }

    pub fn to_color(self) -> Color {
        Color::new(self.x, self.y, self.z)
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Vector3 {
        Vector3::new(1.0, 1.0, 1.0)
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        let cross_x = self.y * other.z - self.z * other.y;
        let cross_y = self.z * other.x - self.x * other.z;
        let cross_z = self.x * other.y - self.y * other.x;

        Vector3 {
            x: cross_x,
            y: cross_y,
            z: cross_z,
        }
    }

    pub fn self_dot(self) -> f64 {
        self * self
    }

    pub fn component_add(self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn square(self) -> Vector3 {
        Vector3::new(self.x * self.x, self.y * self.y, self.z * self.z)
    }

    pub fn distance(self, other: Vector3) -> f64 {
        (self - other).square().component_add().sqrt()
    }

    pub fn min(self, other: Vector3) -> Vector3 {
        Vector3::new(
            f64::min(self.x, other.x),
             f64::min(self.y, other.y), 
             f64::min(self.z, other.z)
        )
    }

    pub fn max(self, other: Vector3) -> Vector3 {
        Vector3::new(
            f64::max(self.x, other.x),
             f64::max(self.y, other.y), 
             f64::max(self.z, other.z)
        )
    }

    pub fn component_ave(self) -> f64 {
        (self.x + self.y + self.z) / 3.0
    }

    pub fn rot_x(self, x_degrees: f64) -> Vector3 {
        let theta_x: f64 = x_degrees*2.0*PI/360.0;
        Vector3::new(
            self.x,
            self.y*theta_x.cos() - self.z*theta_x.sin(),
            self.y*theta_x.sin() + self.z*theta_x.cos()
        )
    }

    pub fn rot_y(self, y_degrees: f64) -> Vector3 {
        let theta_y: f64 = y_degrees*2.0*PI/360.0;
        Vector3::new(
            self.x*theta_y.cos() + self.z*theta_y.sin(),
            self.y,
            self.z*theta_y.cos() - self.x*theta_y.sin()
        )
    }

    pub fn rot_z(self, z_degrees: f64) -> Vector3 {
        let theta_z: f64 = z_degrees*2.0*PI/360.0;
        Vector3::new(
            self.x*theta_z.cos() - self.y*theta_z.sin(),
            self.x*theta_z.sin() + self.y*theta_z.cos(),
            self.z
        )
    }

    pub fn rot(self, degrees: Vector3) -> Vector3 {
        self.rot_x(degrees.x).rot_y(degrees.y).rot_z(degrees.z)
    }

    pub fn random_normal() -> Vector3 {
        Vector3::new(
            StdRng::from_entropy().gen_range(0.0..1.0),
            StdRng::from_entropy().gen_range(0.0..1.0),
            StdRng::from_entropy().gen_range(0.0..1.0)
        ).normalize()
    }

    pub fn random_hemisphere_normal(normal: Vector3) -> Vector3 {
        let mut rng = rand::thread_rng();
        let mut random_vector = Vector3::new(
            rng.gen::<f64>() * 2.0 - 1.0,
            rng.gen::<f64>() * 2.0 - 1.0,
            rng.gen::<f64>() * 2.0 - 1.0,
        ).normalize();

        if random_vector * normal < 0.0 { random_vector *= -1.0 }
    
        random_vector
    }

    pub fn random_perturb(scale: Vector2) -> Vector3 {
        let rand_val1: f64 = rand::thread_rng().gen_range(0.0..1.0);
        let rand_val2: f64 = rand::thread_rng().gen_range(0.0..1.0);
        let angle: f64 = rand_val1*2.0*PI;
        let circle_pt: Vector2 = Vector2::new(angle.cos(), angle.sin());
        rand_val2.sqrt()*Vector3::new(0.0, circle_pt.y/scale.x, circle_pt.x/scale.y)
    }

    pub fn normalize(self) -> Vector3 {
        let magnitude: f64 = self.magnitude();
        
        if magnitude != 0.0 {
            Vector3::new(
                self.x / magnitude,
                self.y / magnitude,
                self.z / magnitude
            )
        } else {
            self
        }
    }

    pub fn lerp(v1: Vector3, v2: Vector3, t: f64) -> Vector3 {
        (1.0 - t)*v1 + t*v2
    }

    pub fn magnitude(self) -> f64 {
        self.square().component_add().sqrt()
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = f64;

    fn mul(self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
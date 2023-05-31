use std::f64::consts::PI;
use rand::{Rng, SeedableRng};
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use crate::datatypes::color::Color;

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
            StdRng::from_entropy().sample(Standard),
            StdRng::from_entropy().sample(Standard),
            StdRng::from_entropy().sample(Standard)
        ) //+ Vector3::one()
    }

    pub fn random_hemisphere_normal(normal: Vector3) -> Vector3 {
        let direction = Self::random_normal();
        let sign = f64::signum(normal * direction);
        sign * direction
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
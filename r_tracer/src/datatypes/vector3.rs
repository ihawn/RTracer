use crate::datatypes::color::Color;
use crate::datatypes::vector2::Vector2;
use std::f32::consts::PI;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
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

    pub fn self_dot(self) -> f32 {
        self * self
    }
    pub fn component_add(self) -> f32 {
        self.x + self.y + self.z
    }
    pub fn square(self) -> Vector3 {
        Vector3::new(self.x * self.x, self.y * self.y, self.z * self.z)
    }
    pub fn distance(self, other: Vector3) -> f32 {
        (self - other).square().component_add().sqrt()
    }
    pub fn min(self, other: Vector3) -> Vector3 {
        Vector3::new(
            f32::min(self.x, other.x),
             f32::min(self.y, other.y), 
             f32::min(self.z, other.z)
        )
    }
    pub fn max(self, other: Vector3) -> Vector3 {
        Vector3::new(
            f32::max(self.x, other.x),
             f32::max(self.y, other.y), 
             f32::max(self.z, other.z)
        )
    }
    pub fn component_ave(self) -> f32 {
        (self.x + self.y + self.z) / 3.0
    }
    pub fn rot_x(self, x_degrees: f32) -> Vector3 {
        let theta_x: f32 = x_degrees*2.0*PI/360.0;
        Vector3::new(
            self.x,
            self.y*theta_x.cos() - self.z*theta_x.sin(),
            self.y*theta_x.sin() + self.z*theta_x.cos()
        )
    }
    pub fn rot_y(self, y_degrees: f32) -> Vector3 {
        let theta_y: f32 = y_degrees*2.0*PI/360.0;
        Vector3::new(
            self.x*theta_y.cos() + self.z*theta_y.sin(),
            self.y,
            self.z*theta_y.cos() - self.x*theta_y.sin()
        )
    }
    pub fn rot_z(self, z_degrees: f32) -> Vector3 {
        let theta_z: f32 = z_degrees*2.0*PI/360.0;
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
            rng.gen::<f32>() * 2.0 - 1.0,
            rng.gen::<f32>() * 2.0 - 1.0,
            rng.gen::<f32>() * 2.0 - 1.0,
        ).normalize();
        if random_vector * normal < 0.0 { random_vector = -1.0 * random_vector }
    
        random_vector
    }
    pub fn random_perturb(scale: Vector2) -> Vector3 {
        let rand_val1: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let rand_val2: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let angle: f32 = rand_val1*2.0*PI;
        let circle_pt: Vector2 = Vector2::new(angle.cos(), angle.sin());
        rand_val2.sqrt()*Vector3::new(0.0, circle_pt.y/scale.x, circle_pt.x/scale.y)
    }
    pub fn random_in_unit_disk() -> Vector3 {
        loop {
            let x = rand::thread_rng().gen_range(-1.0..1.0);
            let y = rand::thread_rng().gen_range(-1.0..1.0);
            let point = Vector3 { x, y, z: 0.0 };
            if point.square().component_add() < 1.0 {
                return point;
            }
        }
    }

    pub fn normalize(self) -> Vector3 {
        let magnitude: f32 = self.magnitude();

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

    pub fn pointwise_multiply(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x*other.x, self.y*other.y, self.z*other.z)
    }

    pub fn lerp(v1: Vector3, v2: Vector3, t: f32) -> Vector3 {
        (1.0 - t)*v1 + t*v2
    }

    pub fn magnitude_squared(self) -> f32 {
        self.square().component_add()
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn morton_code(&self) -> u64 {
        let mut morton_code: u64 = 0;

        let fixed_x = (self.x * 1024.0).floor() as u64;
        let fixed_y = (self.y * 1024.0).floor() as u64;
        let fixed_z = (self.z * 1024.0).floor() as u64;

        for i in 0..10 {
            morton_code |= (fixed_x & (1 << i)) << (2 * i);
            morton_code |= (fixed_y & (1 << i)) << ((2 * i) + 1);
            morton_code |= (fixed_z & (1 << i)) << ((2 * i) + 2);
        }

        morton_code
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = f32;

    fn mul(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Vector3 {
        Vector3::new(self.x / other, self.y / other, self.z / other)
    }
}

impl std::ops::Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index for Vector3"),
        }
    }
}
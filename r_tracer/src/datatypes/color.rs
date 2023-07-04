use crate::datatypes::vector3::Vector3;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;

#[derive(Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn to_vector3(self) -> Vector3 {
        Vector3::new(self.red, self.green, self.blue)
    }

    pub fn as_buffer_color(self: &Color) -> u32 {
        let r = (self.red.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.green.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.blue.clamp(0.0, 1.0) * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }

    pub fn lerp(v1: Color, v2: Color, t: f64) -> Color {
        v1*(1.0 - t) + v2*t
    }

    pub fn to_greyscale(self: &Color) -> f64 {
        (0.2989 * self.red) + (0.5870 * self.green) + (0.1140 * self.blue)
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        let r = self.red * other.red;
        let g = self.green * other.green;
        let b = self.blue * other.blue;
        Color::new(r, g, b)
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        let r = self.red * other;
        let g = self.green * other;
        let b = self.blue * other;
        Color::new(r, g, b)
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r = self.red + other.red;
        let g = self.green + other.green;
        let b = self.blue + other.blue;
        Color::new(r, g, b)
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.red = self.red + other.red;
        self.green = self.green + other.green;
        self.blue = self.blue + other.blue;
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        self.red *= other;
        self.green *= other;
        self.blue *= other;
    }
}

impl DivAssign<u32> for Color {
    fn div_assign(&mut self, scalar: u32) {
        self.red /= scalar as f64;
        self.green /= scalar as f64;
        self.blue /= scalar as f64;
    }
}

impl std::ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        let r = self.red - other.red;
        let g = self.green - other.green;
        let b = self.blue - other.blue;
        Color::new(r, g, b)
    }
}
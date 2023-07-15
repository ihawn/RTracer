use crate::datatypes::vector3::Vector3;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;

#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
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

    pub fn lerp(v1: Color, v2: Color, t: f32) -> Color {
        v1*(1.0 - t) + v2*t
    }

    pub fn to_greyscale(self: &Color) -> f32 {
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

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
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

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, other: f32) {
        self.red *= other;
        self.green *= other;
        self.blue *= other;
    }
}

impl DivAssign<u32> for Color {
    fn div_assign(&mut self, scalar: u32) {
        self.red /= scalar as f32;
        self.green /= scalar as f32;
        self.blue /= scalar as f32;
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
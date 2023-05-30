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

    pub fn as_buffer_color(self: &Color) -> u32 {
        let r = (self.red.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.green.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.blue.clamp(0.0, 1.0) * 255.0) as u32;
        (r << 16) | (g << 8) | b
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
        let r = (self.red + other.red).clamp(0.0, 1.0);
        let g = (self.green + other.green).clamp(0.0, 1.0);
        let b = (self.blue + other.blue).clamp(0.0, 1.0);
        Color::new(r, g, b)
    }
}

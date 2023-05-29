#[derive(Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { red: r, green: g, blue: b }
    }

    pub fn black() -> Color {
        Color { red: 0, green: 0, blue: 0 }
    }

    pub fn white() -> Color {
        Color { red: 255, green: 255, blue: 255 }
    }

    pub fn as_buffer_color(self: &Color) -> u32 {
        (self.red as u32) << 16 | (self.green as u32) << 8 | (self.blue as u32)
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        let r: u8 = f64::round((self.red as f64)*(other.red as f64)/255.0) as u8;
        let g: u8 = f64::round((self.green as f64)*(other.green as f64)/255.0) as u8;
        let b: u8 = f64::round((self.blue as f64)*(other.blue as f64)/255.0) as u8;
        Color::new(r, g, b)
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r: u8 = u32::min(255, (self.red as u32 + other.red as u32)) as u8;
        let g: u8 = u32::min(255, (self.green as u32 + other.green as u32)) as u8;
        let b: u8 = u32::min(255, (self.blue as u32 + other.blue as u32)) as u8;
        Color::new(r, g, b)
    }
}
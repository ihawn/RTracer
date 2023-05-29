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

    pub fn as_buffer_color(self: &Color) -> u32 {
        (self.red as u32) << 16 | (self.green as u32) | (self.blue as u32)
    }
}
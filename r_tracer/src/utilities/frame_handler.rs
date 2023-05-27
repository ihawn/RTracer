use minifb::{Window, WindowOptions, Error};
use crate::datatypes::vector2d::Vector2D;
use crate::datatypes::color::Color;

pub struct FrameHandler {
    pub window: Window,
    size_x: usize,
    size_y: usize
}

impl FrameHandler {
    pub fn new(width: usize, height: usize, frame_label: &str) -> FrameHandler {
        FrameHandler {
            window: Window::new(
                frame_label,
                width,
                height, 
                 WindowOptions::default()
                ).unwrap_or_else(|e| {
                panic!("{}", e);
            }),
            size_x: width,
            size_y: height
        }
    }

    pub fn update_window(&mut self, pixel_values: Vector2D<Color>) {
        let converted_values: Vec<u32> = Self::buffer_from_color_vec(&pixel_values);
        let _update: Result<(), Error> = self.window.update_with_buffer(
            &converted_values,
            self.size_x, self.size_y
        );
    }

    fn buffer_from_color_vec(pixel_values: &Vector2D<Color>) -> Vec<u32> {
        pixel_values.data.iter().map(|color| color.as_buffer_color()).collect()
    }
}
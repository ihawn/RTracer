use minifb::{Window, WindowOptions};

pub struct FrameHandler {
    pub window: Window,
    pub size_x: usize,
    pub size_y: usize
}

impl Clone for FrameHandler {
    fn clone(&self) -> Self {
        FrameHandler::new(self.size_x, self.size_y, "RTracer")
    }
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
}


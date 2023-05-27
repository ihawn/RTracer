mod utilities {
    pub mod frame_handler;
}

mod datatypes {
    pub mod color;
    pub mod vector2d;
}

use utilities::frame_handler::FrameHandler;
use datatypes::color::Color;
use datatypes::vector2d::Vector2D;

fn main() {
    let size_x: usize = 480;
    let size_y: usize = 480;

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "test");
        
    while true {
        let col: Color = Color::new(255, 255, 0);
        let frame: Vector2D<Color> = Vector2D::new(size_x, size_y, col);
        frame_handler.update_window(frame);
    }
}
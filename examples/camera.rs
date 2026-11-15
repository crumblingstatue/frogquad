use {
    frogquad::{Camera2D, Color},
    glam::vec2,
};

struct CameraApp {}

impl frogquad::App for CameraApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::LIGHTGRAY);

        // Render some primitives in camera space

        fc.set_camera(&Camera2D {
            zoom: vec2(1., fc.screen_width() / fc.screen_height()),
            ..Default::default()
        });
        fc.draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, Color::BLUE);
        fc.draw_rectangle(-0.3, 0.3, 0.2, 0.2, Color::GREEN);
        fc.draw_circle(0., 0., 0.1, Color::YELLOW);

        // Back to screen space, render some text

        fc.set_default_camera();
        fc.draw_text("HELLO", 30.0, 200.0, 30.0, Color::BLACK);
    }
}

fn main() {
    frogquad::start("Camera", |_fc| CameraApp {});
}

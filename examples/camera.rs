use {
    frogquad::{
        Color,
        camera::{Camera2D, set_camera, set_default_camera},
        conf::Conf,
        shapes::{draw_circle, draw_line, draw_rectangle},
        text::draw_text,
        window::{clear_background, screen_height, screen_width},
    },
    glam::vec2,
};

struct CameraApp {}

impl frogquad::App for CameraApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, Color::LIGHTGRAY);

        // Render some primitives in camera space

        set_camera(
            fc,
            &Camera2D {
                zoom: vec2(1., screen_width(fc) / screen_height(fc)),
                ..Default::default()
            },
        );
        draw_line(fc, -0.4, 0.4, -0.8, 0.9, 0.05, Color::BLUE);
        draw_rectangle(fc, -0.3, 0.3, 0.2, 0.2, Color::GREEN);
        draw_circle(fc, 0., 0., 0.1, Color::YELLOW);

        // Back to screen space, render some text

        set_default_camera(fc);
        draw_text(fc, "HELLO", 30.0, 200.0, 30.0, Color::BLACK);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Camera"), |_fc| Box::new(CameraApp {}));
}

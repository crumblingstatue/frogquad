use frogquad::{
    Color,
    conf::Conf,
    shapes::{draw_circle, draw_line, draw_rectangle},
    text::draw_text,
    window::{clear_background, screen_height, screen_width},
};

struct BasicShapesApp {}

impl frogquad::App for BasicShapesApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, Color::LIGHTGRAY);

        draw_line(fc, 40.0, 40.0, 100.0, 200.0, 15.0, Color::BLUE);
        draw_rectangle(
            fc,
            screen_width(fc) / 2.0 - 60.0,
            100.0,
            120.0,
            60.0,
            Color::GREEN,
        );
        draw_circle(
            fc,
            screen_width(fc) - 30.0,
            screen_height(fc) - 30.0,
            15.0,
            Color::YELLOW,
        );

        draw_text(fc, "HELLO", 20.0, 20.0, 30.0, Color::DARKGRAY);
    }
}

fn main() {
    frogquad::start(Conf::from_title("BasicShapes"), |_fc| {
        Box::new(BasicShapesApp {})
    });
}

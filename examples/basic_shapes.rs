use frogquad::Color;

struct BasicShapesApp {}

impl frogquad::App for BasicShapesApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::LIGHTGRAY);

        fc.draw_line(40.0, 40.0, 100.0, 200.0, 15.0, Color::BLUE);
        fc.draw_rectangle(
            fc.screen_width() / 2.0 - 60.0,
            100.0,
            120.0,
            60.0,
            Color::GREEN,
        );
        fc.draw_circle(
            fc.screen_width() - 30.0,
            fc.screen_height() - 30.0,
            15.0,
            Color::YELLOW,
        );

        fc.draw_text("HELLO", 20.0, 20.0, 30.0, Color::DARKGRAY);
    }
}

fn main() {
    frogquad::start("BasicShapes", |_fc| BasicShapesApp {});
}

use frogquad::{
    color,
    conf::Conf,
    input::{TouchPhase, touches},
    shapes::draw_circle,
    text::draw_text,
    window::clear_background,
};

struct InputTouchApp {}

impl frogquad::App for InputTouchApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, color::LIGHTGRAY);

        for touch in touches(fc) {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => (color::GREEN, 80.0),
                TouchPhase::Stationary => (color::WHITE, 60.0),
                TouchPhase::Moved => (color::YELLOW, 60.0),
                TouchPhase::Ended => (color::BLUE, 80.0),
                TouchPhase::Cancelled => (color::BLACK, 80.0),
            };
            draw_circle(fc, touch.position.x, touch.position.y, size, fill_color);
        }

        draw_text(fc, "touch the screen!", 20.0, 20.0, 20.0, color::DARKGRAY);
    }
}

fn main() {
    frogquad::start(Conf::from_title("InputTouch"), |_fc| {
        Box::new(InputTouchApp {})
    });
}

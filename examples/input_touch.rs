use frogquad::{Color, conf::Conf, input::TouchPhase, text::draw_text};

struct InputTouchApp {}

impl frogquad::App for InputTouchApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::LIGHTGRAY);

        for touch in fc.touches() {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => (Color::GREEN, 80.0),
                TouchPhase::Stationary => (Color::WHITE, 60.0),
                TouchPhase::Moved => (Color::YELLOW, 60.0),
                TouchPhase::Ended => (Color::BLUE, 80.0),
                TouchPhase::Cancelled => (Color::BLACK, 80.0),
            };
            fc.draw_circle(touch.position.x, touch.position.y, size, fill_color);
        }

        draw_text(fc, "touch the screen!", 20.0, 20.0, 20.0, Color::DARKGRAY);
    }
}

fn main() {
    frogquad::start(Conf::from_title("InputTouch"), |_fc| {
        Box::new(InputTouchApp {})
    });
}

use frogquad::prelude::*;

struct InputTouchApp {}

impl frogquad::App for InputTouchApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, LIGHTGRAY);

        for touch in touches(fc) {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => (GREEN, 80.0),
                TouchPhase::Stationary => (WHITE, 60.0),
                TouchPhase::Moved => (YELLOW, 60.0),
                TouchPhase::Ended => (BLUE, 80.0),
                TouchPhase::Cancelled => (BLACK, 80.0),
            };
            draw_circle(fc, touch.position.x, touch.position.y, size, fill_color);
        }

        draw_text(fc, "touch the screen!", 20.0, 20.0, 20.0, DARKGRAY);
    }
}

fn main() {
    frogquad::start(Conf::from_title("InputTouch"), |_fc| {
        Box::new(InputTouchApp {})
    });
}

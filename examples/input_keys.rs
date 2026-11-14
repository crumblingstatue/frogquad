use {
    frogquad::{Color, FrogCx, conf::Conf, text::draw_text},
    miniquad::KeyCode,
};

struct InputKeysApp {
    x: f32,
    y: f32,
}

impl InputKeysApp {
    fn new(fc: &mut FrogCx) -> Self {
        Self {
            x: fc.screen_width() / 2.0,
            y: fc.screen_height() / 2.0,
        }
    }
}

impl frogquad::App for InputKeysApp {
    fn update(&mut self, fc: &mut FrogCx) {
        fc.clear_background(Color::LIGHTGRAY);

        if fc.is_key_down(KeyCode::Right) {
            self.x += 1.0;
        }
        if fc.is_key_down(KeyCode::Left) {
            self.x -= 1.0;
        }
        if fc.is_key_down(KeyCode::Down) {
            self.y += 1.0;
        }
        if fc.is_key_down(KeyCode::Up) {
            self.y -= 1.0;
        }

        fc.draw_circle(self.x, self.y, 15.0, Color::YELLOW);
        draw_text(
            fc,
            "move the ball with arrow keys",
            20.0,
            20.0,
            20.0,
            Color::DARKGRAY,
        );
    }
}

fn main() {
    frogquad::start(Conf::from_title("InputKeys"), |cx| {
        Box::new(InputKeysApp::new(cx))
    });
}

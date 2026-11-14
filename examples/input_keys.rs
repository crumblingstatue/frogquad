use {
    frogquad::{
        FrogCx, color,
        conf::Conf,
        input::is_key_down,
        shapes::draw_circle,
        text::draw_text,
        window::{clear_background, screen_height, screen_width},
    },
    miniquad::KeyCode,
};

struct InputKeysApp {
    x: f32,
    y: f32,
}

impl InputKeysApp {
    fn new(fc: &mut FrogCx) -> Self {
        Self {
            x: screen_width(fc) / 2.0,
            y: screen_height(fc) / 2.0,
        }
    }
}

impl frogquad::App for InputKeysApp {
    fn update(&mut self, fc: &mut FrogCx) {
        clear_background(fc, color::LIGHTGRAY);

        if is_key_down(fc, KeyCode::Right) {
            self.x += 1.0;
        }
        if is_key_down(fc, KeyCode::Left) {
            self.x -= 1.0;
        }
        if is_key_down(fc, KeyCode::Down) {
            self.y += 1.0;
        }
        if is_key_down(fc, KeyCode::Up) {
            self.y -= 1.0;
        }

        draw_circle(fc, self.x, self.y, 15.0, color::YELLOW);
        draw_text(
            fc,
            "move the ball with arrow keys",
            20.0,
            20.0,
            20.0,
            color::DARKGRAY,
        );
    }
}

fn main() {
    frogquad::start(Conf::from_title("InputKeys"), |cx| {
        Box::new(InputKeysApp::new(cx))
    });
}

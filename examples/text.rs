use {
    frogquad::{Color, Font, FrogCx, TextParams},
    pollster::FutureExt,
};

struct TextApp {
    angle: f32,
    font: Font,
}

impl TextApp {
    fn new(fc: &mut FrogCx) -> Self {
        let font = fc
            .load_ttf_font("./examples/DancingScriptRegular.ttf")
            .block_on()
            .unwrap();

        Self { font, angle: 0.0 }
    }
}

impl frogquad::App for TextApp {
    fn update(&mut self, fc: &mut FrogCx) {
        fc.clear_background(Color::BLACK);

        fc.draw_text_ex("Custom font size:", 20.0, 20.0, TextParams::default());
        let mut y = 20.0;

        for font_size in (30..100).step_by(20) {
            let text = "abcdef";
            let params = TextParams {
                font_size,
                ..Default::default()
            };

            y += font_size as f32;
            fc.draw_text_ex(text, 20.0, y, params);
        }

        fc.draw_text_ex("Dynamic font scale:", 20.0, 400.0, TextParams::default());
        let time = fc.time();
        fc.draw_text_ex(
            "abcd",
            20.0,
            450.0,
            TextParams {
                font_size: 50,
                font_scale: time.sin() as f32 / 2.0 + 1.0,
                ..Default::default()
            },
        );

        fc.draw_text_ex("Custom font:", 400.0, 20.0, TextParams::default());
        fc.draw_text_ex(
            "abcd",
            400.0,
            70.0,
            TextParams {
                font_size: 50,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        fc.draw_text_ex(
            "abcd",
            400.0,
            160.0,
            TextParams {
                font_size: 100,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        fc.draw_text_ex(
            "abcd",
            fc.screen_width() / 4.0 * 2.0,
            fc.screen_height() / 3.0 * 2.0,
            TextParams {
                font_size: 70,
                font: Some(&self.font),
                rotation: self.angle,
                ..Default::default()
            },
        );

        let center = fc
            .default_font()
            .get_text_center("abcd", 70, 1.0, self.angle * 2.0);
        fc.draw_text_ex(
            "abcd",
            fc.screen_width() / 4.0 * 3.0 - center.x,
            fc.screen_height() / 3.0 * 2.0 - center.y,
            TextParams {
                font_size: 70,
                rotation: self.angle * 2.0,
                ..Default::default()
            },
        );

        self.angle -= 0.030;
    }
}

fn main() {
    frogquad::start("Text", TextApp::new);
}

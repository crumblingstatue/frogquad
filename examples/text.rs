use {
    frogquad::{
        FrogCx, color,
        conf::Conf,
        text::{Font, TextParams, draw_text_ex, get_text_center, load_ttf_font},
        time::get_time,
        window::{clear_background, screen_height, screen_width},
    },
    pollster::FutureExt,
};

struct TextApp {
    angle: f32,
    font: Font,
}

impl TextApp {
    fn new(fc: &mut FrogCx) -> Self {
        let font = load_ttf_font(fc, "./examples/DancingScriptRegular.ttf")
            .block_on()
            .unwrap();

        Self { font, angle: 0.0 }
    }
}

impl frogquad::App for TextApp {
    fn update(&mut self, fc: &mut FrogCx) {
        clear_background(fc, color::BLACK);

        draw_text_ex(fc, "Custom font size:", 20.0, 20.0, TextParams::default());
        let mut y = 20.0;

        for font_size in (30..100).step_by(20) {
            let text = "abcdef";
            let params = TextParams {
                font_size,
                ..Default::default()
            };

            y += font_size as f32;
            draw_text_ex(fc, text, 20.0, y, params);
        }

        draw_text_ex(
            fc,
            "Dynamic font scale:",
            20.0,
            400.0,
            TextParams::default(),
        );
        let time = get_time(fc);
        draw_text_ex(
            fc,
            "abcd",
            20.0,
            450.0,
            TextParams {
                font_size: 50,
                font_scale: time.sin() as f32 / 2.0 + 1.0,
                ..Default::default()
            },
        );

        draw_text_ex(fc, "Custom font:", 400.0, 20.0, TextParams::default());
        draw_text_ex(
            fc,
            "abcd",
            400.0,
            70.0,
            TextParams {
                font_size: 50,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        draw_text_ex(
            fc,
            "abcd",
            400.0,
            160.0,
            TextParams {
                font_size: 100,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        draw_text_ex(
            fc,
            "abcd",
            screen_width(fc) / 4.0 * 2.0,
            screen_height(fc) / 3.0 * 2.0,
            TextParams {
                font_size: 70,
                font: Some(&self.font),
                rotation: self.angle,
                ..Default::default()
            },
        );

        let center = get_text_center("abcd", fc.default_font(), 70, 1.0, self.angle * 2.0);
        draw_text_ex(
            fc,
            "abcd",
            screen_width(fc) / 4.0 * 3.0 - center.x,
            screen_height(fc) / 3.0 * 2.0 - center.y,
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
    frogquad::start(Conf::from_title("Text"), |fc| Box::new(TextApp::new(fc)));
}

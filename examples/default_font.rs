use {
    frogquad::{Color, text::TextParams},
    pollster::FutureExt,
};

struct DefaultFontApp {}

impl frogquad::App for DefaultFontApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::WHITE);

        fc.draw_text(
            "Hello world in a new default font!",
            100.0,
            100.0,
            40.0,
            Color::BLACK,
        );

        fc.draw_text_ex(
            "And with extra formatting options",
            100.0,
            230.0,
            TextParams {
                font_size: 45,
                color: Color::RED,
                rotation: 0.27,
                ..Default::default()
            },
        );
    }
}

fn main() {
    frogquad::start("DefaultFont", |fc| {
        let font = fc
            .load_ttf_font("./examples/DancingScriptRegular.ttf")
            .block_on()
            .unwrap();
        fc.set_default_font(font);
        Box::new(DefaultFontApp {})
    });
}

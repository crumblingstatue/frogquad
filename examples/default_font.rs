use {
    frogquad::{
        Color,
        conf::Conf,
        text::{TextParams, draw_text, draw_text_ex, load_ttf_font, set_default_font},
    },
    pollster::FutureExt,
};

struct DefaultFontApp {}

impl frogquad::App for DefaultFontApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::WHITE);

        draw_text(
            fc,
            "Hello world in a new default font!",
            100.0,
            100.0,
            40.0,
            Color::BLACK,
        );

        draw_text_ex(
            fc,
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
    frogquad::start(Conf::from_title("DefaultFont"), |fc| {
        let font = load_ttf_font(fc, "./examples/DancingScriptRegular.ttf")
            .block_on()
            .unwrap();
        set_default_font(fc, font);
        Box::new(DefaultFontApp {})
    });
}

use {frogquad::prelude::*, pollster::FutureExt};

struct DefaultFontApp {}

impl frogquad::App for DefaultFontApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, WHITE);

        draw_text(
            fc,
            "Hello world in a new default font!",
            100.0,
            100.0,
            40.0,
            BLACK,
        );

        draw_text_ex(
            fc,
            "And with extra formatting options",
            100.0,
            230.0,
            TextParams {
                font_size: 45,
                color: RED,
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

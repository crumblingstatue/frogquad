use {
    frogquad::{
        Color,
        conf::Conf,
        shapes::draw_rectangle,
        text::{Font, TextParams, draw_text, draw_text_ex, load_ttf_font, measure_text},
    },
    pollster::FutureExt,
};

fn draw_text_annotated(
    fc: &mut frogquad::FrogCx,
    text: &str,
    font: Option<&Font>,
    x: f32,
    baseline: f32,
) {
    let size = measure_text(text, font.unwrap_or_else(|| fc.default_font()), 100, 1.0);

    // Full background rect
    draw_rectangle(
        fc,
        x,
        baseline - size.offset_y,
        size.width,
        size.height,
        Color::BLUE,
    );

    // Base line
    draw_rectangle(fc, x, baseline - 2.0, size.width, 4.0, Color::RED);

    // Base line annotation
    draw_rectangle(fc, x + size.width, baseline - 1.0, 120.0, 1.0, Color::GRAY);
    draw_text(
        fc,
        "baseline",
        x + size.width + 10.0,
        baseline - 5.0,
        30.0,
        Color::WHITE,
    );

    // Top line
    draw_rectangle(
        fc,
        x,
        baseline - 2.0 - size.offset_y,
        size.width,
        4.0,
        Color::RED,
    );

    // Top line annotation
    draw_rectangle(
        fc,
        x + size.width,
        baseline - size.offset_y - 1.0,
        120.0,
        1.0,
        Color::GRAY,
    );
    draw_text(
        fc,
        "topline",
        x + size.width + 10.0,
        baseline - size.offset_y - 5.0,
        30.0,
        Color::WHITE,
    );

    // Bottom line
    draw_rectangle(
        fc,
        x,
        baseline - 2.0 - size.offset_y + size.height,
        size.width,
        4.0,
        Color::RED,
    );

    // Bottom line annotation
    draw_rectangle(
        fc,
        x + size.width,
        baseline - size.offset_y + size.height - 1.0,
        120.0,
        1.0,
        Color::GRAY,
    );
    draw_text(
        fc,
        "bottomline",
        x + size.width + 10.0,
        baseline - size.offset_y + size.height - 5.0,
        30.0,
        Color::WHITE,
    );

    draw_text_ex(
        fc,
        text,
        x,
        baseline,
        TextParams {
            font_size: 100,
            font,
            ..Default::default()
        },
    );
}

struct TextMeasApp {
    font: Font,
}
impl TextMeasApp {
    fn new(fc: &mut frogquad::FrogCx) -> Self {
        let font = load_ttf_font(fc, "./examples/DancingScriptRegular.ttf")
            .block_on()
            .unwrap();
        Self { font }
    }
}

impl frogquad::App for TextMeasApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::BLACK);

        let text = "abcdIj";

        draw_text_annotated(fc, text, None, 40.0, 200.0);
        draw_text_annotated(fc, text, Some(&self.font), 400.0, 400.0);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Text"), |fc| {
        Box::new(TextMeasApp::new(fc))
    });
}

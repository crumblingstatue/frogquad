use frogquad::prelude::*;

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
        BLUE,
    );

    // Base line
    draw_rectangle(fc, x, baseline - 2.0, size.width, 4.0, RED);

    // Base line annotation
    draw_rectangle(fc, x + size.width, baseline - 1.0, 120.0, 1.0, GRAY);
    draw_text(
        fc,
        "baseline",
        x + size.width + 10.0,
        baseline - 5.0,
        30.0,
        WHITE,
    );

    // Top line
    draw_rectangle(fc, x, baseline - 2.0 - size.offset_y, size.width, 4.0, RED);

    // Top line annotation
    draw_rectangle(
        fc,
        x + size.width,
        baseline - size.offset_y - 1.0,
        120.0,
        1.0,
        GRAY,
    );
    draw_text(
        fc,
        "topline",
        x + size.width + 10.0,
        baseline - size.offset_y - 5.0,
        30.0,
        WHITE,
    );

    // Bottom line
    draw_rectangle(
        fc,
        x,
        baseline - 2.0 - size.offset_y + size.height,
        size.width,
        4.0,
        RED,
    );

    // Bottom line annotation
    draw_rectangle(
        fc,
        x + size.width,
        baseline - size.offset_y + size.height - 1.0,
        120.0,
        1.0,
        GRAY,
    );
    draw_text(
        fc,
        "bottomline",
        x + size.width + 10.0,
        baseline - size.offset_y + size.height - 5.0,
        30.0,
        WHITE,
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

fn main() {
    frogquad::start(Conf::from_title("Text"), main_());
}
async fn main_() {
    let font = load_ttf_font("./examples/DancingScriptRegular.ttf")
        .await
        .unwrap();

    loop {
        clear_background(fc, BLACK);

        let text = "abcdIj";

        draw_text_annotated(fc, text, None, 40.0, 200.0);
        draw_text_annotated(fc, text, Some(&font), 400.0, 400.0);

        next_frame().await
    }
}

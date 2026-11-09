use frogquad::prelude::*;

#[frogquad::main("DefaultFont")]
async fn main() {
    let fc = frogquad::get_context();
    let font = load_ttf_font(
        frogquad::get_context(),
        "./examples/DancingScriptRegular.ttf",
    )
    .await
    .unwrap();
    set_default_font(fc, font);

    loop {
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

        next_frame().await;
    }
}

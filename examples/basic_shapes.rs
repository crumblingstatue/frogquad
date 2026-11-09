use frogquad::prelude::*;

#[frogquad::main("BasicShapes")]
async fn main() {
    let fc = frogquad::get_context();
    loop {
        clear_background(fc, LIGHTGRAY);

        draw_line(fc, 40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(fc, screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(
            fc,
            screen_width() - 30.0,
            screen_height() - 30.0,
            15.0,
            YELLOW,
        );

        draw_text(fc, "HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

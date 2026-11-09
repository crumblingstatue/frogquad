use frogquad::prelude::*;

#[frogquad::main("InputKeys")]
async fn main() {
    let fc = frogquad::get_context();
    let mut x = screen_width(fc) / 2.0;
    let mut y = screen_height(fc) / 2.0;

    loop {
        clear_background(fc, LIGHTGRAY);

        if is_key_down(fc, KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(fc, KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(fc, KeyCode::Down) {
            y += 1.0;
        }
        if is_key_down(fc, KeyCode::Up) {
            y -= 1.0;
        }

        draw_circle(fc, x, y, 15.0, YELLOW);
        draw_text(
            fc,
            "move the ball with arrow keys",
            20.0,
            20.0,
            20.0,
            DARKGRAY,
        );
        next_frame().await
    }
}

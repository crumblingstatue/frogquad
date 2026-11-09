use frogquad::prelude::*;

#[frogquad::main("InputTouch")]
async fn main() {
    let fc = frogquad::get_context();
    loop {
        clear_background(LIGHTGRAY);

        for touch in touches(fc) {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => (GREEN, 80.0),
                TouchPhase::Stationary => (WHITE, 60.0),
                TouchPhase::Moved => (YELLOW, 60.0),
                TouchPhase::Ended => (BLUE, 80.0),
                TouchPhase::Cancelled => (BLACK, 80.0),
            };
            draw_circle(fc, touch.position.x, touch.position.y, size, fill_color);
        }

        draw_text(fc, "touch the screen!", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await
    }
}

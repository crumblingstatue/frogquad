use frogquad::prelude::*;

fn main() {
    frogquad::start(Conf::from_title("Camera"), main_());
}
async fn main_() {
    loop {
        clear_background(fc, LIGHTGRAY);

        // Render some primitives in camera space

        set_camera(
            fc,
            &Camera2D {
                zoom: vec2(1., screen_width(fc) / screen_height(fc)),
                ..Default::default()
            },
        );
        draw_line(fc, -0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(fc, -0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(fc, 0., 0., 0.1, YELLOW);

        // Back to screen space, render some text

        set_default_camera(fc);
        draw_text(fc, "HELLO", 30.0, 200.0, 30.0, BLACK);

        next_frame().await
    }
}

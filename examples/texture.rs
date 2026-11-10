use frogquad::prelude::*;

fn main() {
    frogquad::start("Texture", main_());
}
async fn main_() {
    let fc = frogquad::get_context();
    let texture: Texture2D = load_texture(fc, "examples/ferris.png").await.unwrap();

    loop {
        clear_background(fc, LIGHTGRAY);
        draw_texture(fc, &texture, 0., 0., WHITE);
        next_frame().await
    }
}

use frogquad::prelude::*;

fn main() {
    frogquad::start(Conf::from_title("Texture"), main_());
}
async fn main_() {
    let texture: Texture2D = load_texture(fc, "examples/ferris.png").await.unwrap();

    loop {
        clear_background(fc, LIGHTGRAY);
        draw_texture(fc, &texture, 0., 0., WHITE);
        next_frame().await
    }
}

use {
    frogquad::{FrogCx, prelude::*},
    pollster::FutureExt,
};

struct TextureApp {
    texture: Texture2D,
}

impl TextureApp {
    fn new(fc: &mut FrogCx) -> Self {
        Self {
            texture: load_texture(fc, "examples/ferris.png").block_on().unwrap(),
        }
    }
}

impl frogquad::App for TextureApp {
    fn update(&mut self, fc: &mut FrogCx) {
        clear_background(fc, LIGHTGRAY);
        draw_texture(fc, &self.texture, 0., 0., WHITE);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Texture"), |fc| {
        Box::new(TextureApp::new(fc))
    });
}

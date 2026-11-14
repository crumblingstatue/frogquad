use {
    frogquad::{
        Color, FrogCx,
        conf::Conf,
        texture::{Texture2D, draw_texture, load_texture},
    },
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
        fc.clear_background(Color::LIGHTGRAY);
        draw_texture(fc, &self.texture, 0., 0., Color::WHITE);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Texture"), |fc| {
        Box::new(TextureApp::new(fc))
    });
}

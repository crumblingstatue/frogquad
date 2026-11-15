use {
    frogquad::{Color, FrogCx, conf::Conf, texture::Texture2D},
    pollster::FutureExt,
};

struct TextureApp {
    texture: Texture2D,
}

impl TextureApp {
    fn new(fc: &mut FrogCx) -> Self {
        Self {
            texture: fc.load_texture("examples/ferris.png").block_on().unwrap(),
        }
    }
}

impl frogquad::App for TextureApp {
    fn update(&mut self, fc: &mut FrogCx) {
        fc.clear_background(Color::LIGHTGRAY);
        fc.draw_texture(&self.texture, 0., 0., Color::WHITE);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Texture"), |fc| {
        Box::new(TextureApp::new(fc))
    });
}

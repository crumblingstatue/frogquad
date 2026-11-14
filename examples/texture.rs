use {
    frogquad::{
        FrogCx, color,
        conf::Conf,
        texture::{Texture2D, draw_texture, load_texture},
        window::clear_background,
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
        clear_background(fc, color::LIGHTGRAY);
        draw_texture(fc, &self.texture, 0., 0., color::WHITE);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Texture"), |fc| {
        Box::new(TextureApp::new(fc))
    });
}

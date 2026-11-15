use pollster::FutureExt;
use {
    frogquad::{Color, FrogCx, rand, texture::Texture2D},
    glam::Vec2,
    miniquad::{FilterMode, MouseButton},
};

struct Rustaceane {
    pos: Vec2,
    speed: Vec2,
    color: Color,
}

struct RustaceanMarkApp {
    rustaceanes: Vec<Rustaceane>,
    rustacean_tex: Texture2D,
}

impl RustaceanMarkApp {
    fn new(fc: &mut FrogCx) -> Self {
        let rustaceanes: Vec<Rustaceane> = Vec::new();
        let rustacean_tex = fc
            .load_texture("examples/rustacean_happy.png")
            .block_on()
            .unwrap();
        rustacean_tex.set_filter(fc, FilterMode::Nearest);
        Self {
            rustaceanes,
            rustacean_tex,
        }
    }
}

impl frogquad::App for RustaceanMarkApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::default());

        if fc.is_mouse_button_down(MouseButton::Left) {
            for _i in 0..100 {
                self.rustaceanes.push(Rustaceane {
                    pos: Vec2::from(fc.mouse_position()),
                    speed: Vec2::new(
                        rand::gen_range(-250., 250.) / 60.,
                        rand::gen_range(-250., 250.) / 60.,
                    ),
                    color: Color::from_rgba(
                        rand::gen_range(50, 240),
                        rand::gen_range(80, 240),
                        rand::gen_range(100, 240),
                        255,
                    ),
                })
            }
        }

        for rustaceane in &mut self.rustaceanes {
            rustaceane.pos += rustaceane.speed;

            if ((rustaceane.pos.x + self.rustacean_tex.width(fc) / 2.) > fc.screen_width())
                || ((rustaceane.pos.x + self.rustacean_tex.width(fc) / 2.) < 0.)
            {
                rustaceane.speed.x *= -1.;
            }
            if ((rustaceane.pos.y + self.rustacean_tex.height(fc) / 2.) > fc.screen_height())
                || ((rustaceane.pos.y + self.rustacean_tex.height(fc) / 2.) < 0.)
            {
                rustaceane.speed.y *= -1.;
            }

            fc.draw_texture(
                &self.rustacean_tex,
                rustaceane.pos.x,
                rustaceane.pos.y,
                rustaceane.color,
            );
        }

        fc.draw_fps();
        fc.draw_text(
            format!("Rustaceanes: {}", self.rustaceanes.len()).as_str(),
            0.,
            32.,
            32.,
            Color::WHITE,
        );
    }
}

fn main() {
    frogquad::start("Rustaceanmark", |fc| Box::new(RustaceanMarkApp::new(fc)));
}

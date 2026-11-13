use frogquad::{FrogCx, prelude::*};
use pollster::FutureExt;

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
        let rustacean_tex = load_texture(fc, "examples/rustacean_happy.png")
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
        clear_background(fc, Color::default());

        if frogquad::input::is_mouse_button_down(fc, MouseButton::Left) {
            for _i in 0..100 {
                self.rustaceanes.push(Rustaceane {
                    pos: Vec2::from(frogquad::input::mouse_position(fc)),
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

            if ((rustaceane.pos.x + self.rustacean_tex.width(fc) / 2.) > screen_width(fc))
                || ((rustaceane.pos.x + self.rustacean_tex.width(fc) / 2.) < 0.)
            {
                rustaceane.speed.x *= -1.;
            }
            if ((rustaceane.pos.y + self.rustacean_tex.height(fc) / 2.) > screen_height(fc))
                || ((rustaceane.pos.y + self.rustacean_tex.height(fc) / 2.) < 0.)
            {
                rustaceane.speed.y *= -1.;
            }

            draw_texture(
                fc,
                &self.rustacean_tex,
                rustaceane.pos.x,
                rustaceane.pos.y,
                rustaceane.color,
            );
        }

        draw_fps(fc);
        draw_text(
            fc,
            format!("Rustaceanes: {}", self.rustaceanes.len()).as_str(),
            0.,
            32.,
            32.,
            WHITE,
        );
    }
}

fn main() {
    frogquad::start(Conf::from_title("Rustaceanmark"), |fc| {
        Box::new(RustaceanMarkApp::new(fc))
    });
}

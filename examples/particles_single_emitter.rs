use frogquad::{Color, FrogCx, camera::Camera2D, math::Rect};

use frogquad_particles::{BlendMode, Curve, Emitter, EmitterConfig};
use glam::vec2;

struct ParticlesSingleApp {
    emitter: Emitter,
}

impl ParticlesSingleApp {
    fn new(fc: &mut FrogCx) -> Self {
        Self {
            emitter: Emitter::new(
                EmitterConfig {
                    lifetime: 0.5,
                    amount: 5,
                    initial_direction_spread: 0.0,
                    initial_velocity: -50.0,
                    size: 2.0,
                    size_curve: Some(Curve {
                        points: vec![(0.0, 0.5), (0.5, 1.0), (1.0, 0.0)],
                        ..Default::default()
                    }),

                    blend_mode: BlendMode::Additive,
                    ..Default::default()
                },
                fc,
            ),
        }
    }
}

impl frogquad::App for ParticlesSingleApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::BLACK);

        let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 100.0, 100.0));

        fc.set_camera(&camera);

        self.emitter.draw(fc, vec2(50., 50.));
    }
}

fn main() {
    frogquad::start("Fountain", |fc| Box::new(ParticlesSingleApp::new(fc)));
}

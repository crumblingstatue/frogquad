use frogquad::{Color, FrogCx, conf::Conf};
use frogquad_particles::{self as particles, AtlasConfig, BlendMode, Emitter, EmitterConfig};
use glam::vec2;
use miniquad::KeyCode;
use pollster::FutureExt;

fn explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        one_shot: true,
        emitting: false,
        lifetime: 0.3,
        lifetime_randomness: 0.7,
        explosiveness: 0.95,
        amount: 30,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 200.0,
        size: 30.0,
        gravity: vec2(0.0, -1000.0),
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        blend_mode: BlendMode::Additive,
        ..Default::default()
    }
}

fn smoke() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 0.8,
        amount: 20,
        initial_direction_spread: 0.2,
        atlas: Some(AtlasConfig::new(4, 4, 0..8)),
        ..Default::default()
    }
}

fn fire() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 0.4,
        lifetime_randomness: 0.1,
        amount: 10,
        initial_direction_spread: 0.5,
        initial_velocity: 300.0,
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        size: 20.0,
        blend_mode: BlendMode::Additive,
        ..Default::default()
    }
}

struct ParticlesApp {
    one_shot_emitter: Emitter,
    flying_emitter_local: Emitter,
    flying_emitter_world: Emitter,
}

impl ParticlesApp {
    fn new(fc: &mut FrogCx) -> Self {
        let texture = fc
            .load_texture("examples/smoke_fire.png")
            .block_on()
            .unwrap();

        let one_shot_emitter = particles::Emitter::new(
            EmitterConfig {
                texture: Some(texture.clone()),
                ..explosion()
            },
            fc,
        );

        let flying_emitter_local = Emitter::new(
            EmitterConfig {
                local_coords: true,
                texture: Some(texture.clone()),
                ..smoke()
            },
            fc,
        );
        let flying_emitter_world = Emitter::new(
            EmitterConfig {
                local_coords: false,
                texture: Some(texture.clone()),
                ..fire()
            },
            fc,
        );
        Self {
            one_shot_emitter,
            flying_emitter_local,
            flying_emitter_world,
        }
    }
}

impl frogquad::App for ParticlesApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::BLACK);

        fc.draw_text("Local coord emitter", 20.0, 30.0, 30.0, Color::RED);

        fc.draw_text("World coord emitter", 20.0, 60.0, 30.0, Color::GREEN);

        fc.draw_text(
            "One shot emitter, press Space to emit",
            20.0,
            90.0,
            30.0,
            Color::YELLOW,
        );
        self.one_shot_emitter.draw(fc, vec2(650.0, 82.0));
        fc.draw_circle(650.0, 82.0, 15.0, Color::YELLOW);

        if fc.is_key_pressed(KeyCode::Space) {
            self.one_shot_emitter.config.emitting = true;
        }

        let local_emitter_pos = vec2(
            (fc.time() * 0.3).sin() as f32 * fc.screen_width() / 2.5 + fc.screen_width() / 2.0,
            (fc.time() * 0.5).cos() as f32 * fc.screen_height() / 2.5 + fc.screen_height() / 2.0,
        );
        self.flying_emitter_local.draw(fc, local_emitter_pos);
        fc.draw_circle(local_emitter_pos.x, local_emitter_pos.y, 15.0, Color::RED);

        let world_emitter_pos = vec2(
            (fc.time() * 0.6 + 1.0).sin() as f32 * fc.screen_width() / 2.5
                + fc.screen_width() / 2.0,
            (fc.time() * 0.4 + 1.0).cos() as f32 * fc.screen_height() / 2.5
                + fc.screen_height() / 2.0,
        );

        self.flying_emitter_world.draw(fc, world_emitter_pos);
        fc.draw_circle(world_emitter_pos.x, world_emitter_pos.y, 15.0, Color::GREEN);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Fountain"), |fc| {
        Box::new(ParticlesApp::new(fc))
    });
}

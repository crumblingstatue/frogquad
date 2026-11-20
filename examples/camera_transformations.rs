use {
    frogquad::{Camera2D, Color, FrogCx},
    glam::vec2,
    miniquad::KeyCode,
};

fn short_angle_dist(a0: f32, a1: f32) -> f32 {
    let max = 360.0;
    let da = (a1 - a0) % max;
    2.0 * da % max - da
}

fn angle_lerp(a0: f32, a1: f32, t: f32) -> f32 {
    a0 + short_angle_dist(a0, a1) * t
}

fn draw_cross(fc: &mut FrogCx, x: f32, y: f32, color: Color) {
    let size = 0.1;
    let thickness = 0.005;
    fc.draw_line(x - size, y, x + size, y, thickness, color);
    fc.draw_line(x, y - size, x, y + size, thickness, color);
}

struct CamTfApp {
    target: (f32, f32),
    zoom: f32,
    rotation: f32,
    smooth_rotation: f32,
    offset: (f32, f32),
}

impl CamTfApp {
    fn new() -> Self {
        Self {
            target: (0., 0.),
            zoom: 1.0,
            rotation: 0.0,
            smooth_rotation: 0.0,
            offset: (0., 0.),
        }
    }
}

impl frogquad::App for CamTfApp {
    fn update(&mut self, fc: &mut FrogCx) {
        if fc.is_key_down(KeyCode::W) {
            self.target.1 -= 0.1;
        }
        if fc.is_key_down(KeyCode::S) {
            self.target.1 += 0.1;
        }
        if fc.is_key_down(KeyCode::A) {
            self.target.0 += 0.1;
        }
        if fc.is_key_down(KeyCode::D) {
            self.target.0 -= 0.1;
        }
        if fc.is_key_down(KeyCode::Left) {
            self.offset.0 -= 0.1;
        }
        if fc.is_key_down(KeyCode::Right) {
            self.offset.0 += 0.1;
        }
        if fc.is_key_down(KeyCode::Up) {
            self.offset.1 += 0.1;
        }
        if fc.is_key_down(KeyCode::Down) {
            self.offset.1 -= 0.1;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if fc.is_key_down(KeyCode::Q) | fc.is_key_down(KeyCode::Escape) {
            fc.request_quit();
        }

        match fc.mouse_wheel() {
            glam::Vec2 { x: _, y } if y != 0.0 => {
                // Normalize mouse wheel values is browser (chromium: 53, firefox: 3)
                #[cfg(target_arch = "wasm32")]
                let y = if y < 0.0 {
                    -1.0
                } else if y > 0.0 {
                    1.0
                } else {
                    0.0
                };
                if fc.is_key_down(KeyCode::LeftControl) {
                    self.zoom *= 1.1f32.powf(y);
                } else {
                    self.rotation += 10.0 * y;
                    self.rotation = match self.rotation {
                        angle if angle >= 360.0 => angle - 360.0,
                        angle if angle < 0.0 => angle + 360.0,
                        angle => angle,
                    }
                }
            }
            _ => (),
        }

        self.smooth_rotation = angle_lerp(self.smooth_rotation, self.rotation, 0.1);

        fc.clear_background(Color::LIGHTGRAY);

        fc.set_camera(&Camera2D {
            target: vec2(self.target.0, self.target.1),
            ..Default::default()
        });
        draw_cross(fc, 0., 0., Color::RED);

        fc.set_camera(&Camera2D {
            target: vec2(self.target.0, self.target.1),
            rotation: self.smooth_rotation,
            ..Default::default()
        });
        draw_cross(fc, 0., 0., Color::GREEN);

        fc.set_camera(&Camera2D {
            target: vec2(self.target.0, self.target.1),
            rotation: self.smooth_rotation,
            zoom: vec2(
                self.zoom,
                self.zoom * fc.screen_width() / fc.screen_height(),
            ),
            ..Default::default()
        });
        draw_cross(fc, 0., 0., Color::BLUE);

        fc.set_camera(&Camera2D {
            target: vec2(self.target.0, self.target.1),
            rotation: self.smooth_rotation,
            zoom: vec2(
                self.zoom,
                self.zoom * fc.screen_width() / fc.screen_height(),
            ),
            offset: vec2(self.offset.0, self.offset.1),
            ..Default::default()
        });

        // Render some primitives in camera space
        fc.draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, Color::BLUE);
        fc.draw_rectangle(-0.3, 0.3, 0.2, 0.2, Color::GREEN);
        fc.draw_circle(0., 0., 0.1, Color::YELLOW);

        // Back to screen space, render some text
        fc.set_default_camera();
        fc.draw_text(
            format!(
                "target (WASD keys) = ({:+.2}, {:+.2})",
                self.target.0, self.target.1
            )
            .as_str(),
            10.0,
            10.0,
            15.0,
            Color::BLACK,
        );
        fc.draw_text(
            format!("rotation (mouse wheel) = {} degrees", self.rotation).as_str(),
            10.0,
            25.0,
            15.0,
            Color::BLACK,
        );
        fc.draw_text(
            format!("zoom (ctrl + mouse wheel) = {:.2}", self.zoom).as_str(),
            10.0,
            40.0,
            15.0,
            Color::BLACK,
        );
        fc.draw_text(
            format!(
                "offset (arrow keys) = ({:+.2}, {:+.2})",
                self.offset.0, self.offset.1
            )
            .as_str(),
            10.0,
            55.0,
            15.0,
            Color::BLACK,
        );
        fc.draw_text("HELLO", 30.0, 200.0, 30.0, Color::BLACK);
    }
}

fn main() {
    frogquad::start("Camera", |_| CamTfApp::new());
}

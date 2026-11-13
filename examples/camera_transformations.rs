use frogquad::{FrogCx, prelude::*};

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
    draw_line(fc, x - size, y, x + size, y, thickness, color);
    draw_line(fc, x, y - size, x, y + size, thickness, color);
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
        if is_key_down(fc, KeyCode::W) {
            self.target.1 -= 0.1;
        }
        if is_key_down(fc, KeyCode::S) {
            self.target.1 += 0.1;
        }
        if is_key_down(fc, KeyCode::A) {
            self.target.0 += 0.1;
        }
        if is_key_down(fc, KeyCode::D) {
            self.target.0 -= 0.1;
        }
        if is_key_down(fc, KeyCode::Left) {
            self.offset.0 -= 0.1;
        }
        if is_key_down(fc, KeyCode::Right) {
            self.offset.0 += 0.1;
        }
        if is_key_down(fc, KeyCode::Up) {
            self.offset.1 += 0.1;
        }
        if is_key_down(fc, KeyCode::Down) {
            self.offset.1 -= 0.1;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(fc, KeyCode::Q) | is_key_down(fc, KeyCode::Escape) {
            fc.request_quit();
        }

        match mouse_wheel(fc) {
            (_x, y) if y != 0.0 => {
                // Normalize mouse wheel values is browser (chromium: 53, firefox: 3)
                #[cfg(target_arch = "wasm32")]
                let y = if y < 0.0 {
                    -1.0
                } else if y > 0.0 {
                    1.0
                } else {
                    0.0
                };
                if is_key_down(fc, KeyCode::LeftControl) {
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

        clear_background(fc, LIGHTGRAY);

        set_camera(
            fc,
            &Camera2D {
                target: vec2(self.target.0, self.target.1),
                ..Default::default()
            },
        );
        draw_cross(fc, 0., 0., RED);

        set_camera(
            fc,
            &Camera2D {
                target: vec2(self.target.0, self.target.1),
                rotation: self.smooth_rotation,
                ..Default::default()
            },
        );
        draw_cross(fc, 0., 0., GREEN);

        set_camera(
            fc,
            &Camera2D {
                target: vec2(self.target.0, self.target.1),
                rotation: self.smooth_rotation,
                zoom: vec2(self.zoom, self.zoom * screen_width(fc) / screen_height(fc)),
                ..Default::default()
            },
        );
        draw_cross(fc, 0., 0., BLUE);

        set_camera(
            fc,
            &Camera2D {
                target: vec2(self.target.0, self.target.1),
                rotation: self.smooth_rotation,
                zoom: vec2(self.zoom, self.zoom * screen_width(fc) / screen_height(fc)),
                offset: vec2(self.offset.0, self.offset.1),
                ..Default::default()
            },
        );

        // Render some primitives in camera space
        draw_line(fc, -0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(fc, -0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(fc, 0., 0., 0.1, YELLOW);

        // Back to screen space, render some text
        set_default_camera(fc);
        draw_text(
            fc,
            format!(
                "target (WASD keys) = ({:+.2}, {:+.2})",
                self.target.0, self.target.1
            )
            .as_str(),
            10.0,
            10.0,
            15.0,
            BLACK,
        );
        draw_text(
            fc,
            format!("rotation (mouse wheel) = {} degrees", self.rotation).as_str(),
            10.0,
            25.0,
            15.0,
            BLACK,
        );
        draw_text(
            fc,
            format!("zoom (ctrl + mouse wheel) = {:.2}", self.zoom).as_str(),
            10.0,
            40.0,
            15.0,
            BLACK,
        );
        draw_text(
            fc,
            format!(
                "offset (arrow keys) = ({:+.2}, {:+.2})",
                self.offset.0, self.offset.1
            )
            .as_str(),
            10.0,
            55.0,
            15.0,
            BLACK,
        );
        draw_text(fc, "HELLO", 30.0, 200.0, 30.0, BLACK);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Camera"), |_| Box::new(CamTfApp::new()));
}

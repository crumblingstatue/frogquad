use frogquad::{FrogCx, prelude::*};

struct ArkanoidApp {
    stick: bool,
    platform_x: f32,
    dy: f32,
    dx: f32,
    ball_y: f32,
    ball_x: f32,
    blocks: [[bool; 10]; 10],
}

impl ArkanoidApp {
    fn new(fc: &mut FrogCx) -> Self {
        let blocks: [[bool; BLOCKS_W]; BLOCKS_H] = [[true; BLOCKS_W]; BLOCKS_H];
        let ball_x = 12.;
        let ball_y = 7.;
        let dx = 3.5;
        let dy = -3.5;
        let platform_x = 10.;
        let stick = true;

        // build camera with following coordinate system:
        // (0., 0)     .... (SCR_W, 0.)
        // (0., SCR_H) .... (SCR_W, SCR_H)
        set_camera(
            fc,
            &Camera2D {
                zoom: vec2(1. / SCR_W * 2., 1. / SCR_H * 2.),
                target: vec2(SCR_W / 2., SCR_H / 2.),
                ..Default::default()
            },
        );
        Self {
            blocks,
            ball_x,
            ball_y,
            dx,
            dy,
            platform_x,
            stick,
        }
    }
}

const BLOCKS_W: usize = 10;
const BLOCKS_H: usize = 10;
const SCR_W: f32 = 20.0;
const SCR_H: f32 = 20.0;

impl frogquad::App for ArkanoidApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        let platform_width = 5.;
        let platform_height = 0.2;
        clear_background(fc, SKYBLUE);

        let delta = get_frame_time(fc);

        if is_key_down(fc, KeyCode::Right) && self.platform_x < SCR_W - platform_width / 2. {
            self.platform_x += 3.0 * delta;
        }
        if is_key_down(fc, KeyCode::Left) && self.platform_x > platform_width / 2. {
            self.platform_x -= 3.0 * delta;
        }

        if !self.stick {
            self.ball_x += self.dx * delta;
            self.ball_y += self.dy * delta;
        } else {
            let (font_size, font_scale, font_aspect) = camera_font_scale(fc, 1.);
            let text_params = TextParams {
                font_size,
                font_scale,
                font_scale_aspect: font_aspect,
                ..Default::default()
            };
            draw_text_ex(
                fc,
                "Press space to start",
                SCR_W / 2. - 5.,
                SCR_H / 2.,
                text_params,
            );

            self.ball_x = self.platform_x;
            self.ball_y = SCR_H - 0.5;

            self.stick = !is_key_down(fc, KeyCode::Space);
        }

        if self.ball_x <= 0. || self.ball_x > SCR_W {
            self.dx *= -1.;
        }
        if self.ball_y <= 0.
            || (self.ball_y > SCR_H - platform_height - 0.15 / 2.
                && self.ball_x >= self.platform_x - platform_width / 2.
                && self.ball_x <= self.platform_x + platform_width / 2.)
        {
            self.dy *= -1.;
        }
        if self.ball_y >= SCR_H {
            self.ball_y = 10.;
            self.dy = -self.dy.abs();
            self.stick = true;
        }

        for j in 0..BLOCKS_H {
            for i in 0..BLOCKS_W {
                if self.blocks[j][i] {
                    let block_w = SCR_W / BLOCKS_W as f32;
                    let block_h = 7.0 / BLOCKS_H as f32;
                    let block_x = i as f32 * block_w + 0.05;
                    let block_y = j as f32 * block_h + 0.05;

                    draw_rectangle(fc, block_x, block_y, block_w - 0.1, block_h - 0.1, DARKBLUE);
                    if self.ball_x >= block_x
                        && self.ball_x < block_x + block_w
                        && self.ball_y >= block_y
                        && self.ball_y < block_y + block_h
                    {
                        self.dy *= -1.;
                        self.blocks[j][i] = false;
                    }
                }
            }
        }

        draw_circle(fc, self.ball_x, self.ball_y, 0.2, RED);
        draw_rectangle(
            fc,
            self.platform_x - platform_width / 2.,
            SCR_H - platform_height,
            platform_width,
            platform_height,
            DARKPURPLE,
        );
    }
}

fn main() {
    frogquad::start(Conf::from_title("Arkanoid"), |fc| {
        Box::new(ArkanoidApp::new(fc))
    });
}

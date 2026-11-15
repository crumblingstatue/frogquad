use {
    frogquad::{
        Color,
        camera::Camera2D,
        math::Rect,
        texture::{DrawTextureParams, RenderTarget},
    },
    glam::{Vec2, vec2},
    miniquad::FilterMode,
};

const VIRTUAL_WIDTH: f32 = 1280.0;
const VIRTUAL_HEIGHT: f32 = 720.0;

struct LetterboxApp {
    render_target: RenderTarget,
    render_target_cam: Camera2D,
}
impl LetterboxApp {
    fn new(fc: &mut frogquad::FrogCx) -> Self {
        // Setup 'render_target', used to hold the rendering result so we can resize it
        let render_target = fc.render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
        render_target.texture.set_filter(fc, FilterMode::Linear);

        // Setup camera for the virtual screen, that will render to 'render_target'
        let mut render_target_cam =
            Camera2D::from_display_rect(Rect::new(0., 0., VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
        render_target_cam.render_target = Some(render_target.clone());
        Self {
            render_target,
            render_target_cam,
        }
    }
}

impl frogquad::App for LetterboxApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        // Get required scaling value
        let scale: f32 = f32::min(
            fc.screen_width() / VIRTUAL_WIDTH,
            fc.screen_height() / VIRTUAL_HEIGHT,
        );

        // Mouse position in the virtual screen
        let virtual_mouse_pos = Vec2 {
            x: (fc.mouse_position().0 - (fc.screen_width() - (VIRTUAL_WIDTH * scale)) * 0.5)
                / scale,
            y: (fc.mouse_position().1 - (fc.screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.5)
                / scale,
        };

        // ------------------------------------------------------------------------
        // Begin drawing the virtual screen to 'render_target'
        // ------------------------------------------------------------------------
        fc.set_camera(&self.render_target_cam);

        fc.clear_background(Color::LIGHTGRAY);

        fc.draw_text("Hello Letterbox", 20.0, 20.0, 30.0, Color::DARKGRAY);
        fc.draw_circle(
            VIRTUAL_WIDTH / 2.0 - 65.0,
            VIRTUAL_HEIGHT / 2.0,
            35.0,
            Color::RED,
        );
        fc.draw_circle(
            VIRTUAL_WIDTH / 2.0 + 65.0,
            VIRTUAL_HEIGHT / 2.0,
            35.0,
            Color::BLUE,
        );
        fc.draw_circle(
            VIRTUAL_WIDTH / 2.0,
            VIRTUAL_HEIGHT / 2.0 - 65.0,
            35.0,
            Color::YELLOW,
        );

        fc.draw_circle(virtual_mouse_pos.x, virtual_mouse_pos.y, 15.0, Color::BLACK);

        // ------------------------------------------------------------------------
        // Begin drawing the window screen
        // ------------------------------------------------------------------------
        fc.set_default_camera();

        fc.clear_background(Color::BLACK); // Will be the letterbox color

        // Draw 'render_target' to window screen, porperly scaled and letterboxed
        fc.draw_texture_ex(
            &self.render_target.texture,
            (fc.screen_width() - (VIRTUAL_WIDTH * scale)) * 0.5,
            (fc.screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.5,
            Color::WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIRTUAL_WIDTH * scale, VIRTUAL_HEIGHT * scale)),
                flip_y: true, // Must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        );
    }
}

fn main() {
    frogquad::start("Letterbox", |fc| Box::new(LetterboxApp::new(fc)));
}

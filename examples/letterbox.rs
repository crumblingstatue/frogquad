use frogquad::prelude::*;

const VIRTUAL_WIDTH: f32 = 1280.0;
const VIRTUAL_HEIGHT: f32 = 720.0;

fn main() {
    frogquad::start(Conf::from_title("Letterbox"), main_());
}
async fn main_() {
    // Setup 'render_target', used to hold the rendering result so we can resize it
    let render_target = render_target(fc, VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_target.texture.set_filter(fc, FilterMode::Linear);

    // Setup camera for the virtual screen, that will render to 'render_target'
    let mut render_target_cam =
        Camera2D::from_display_rect(Rect::new(0., 0., VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
    render_target_cam.render_target = Some(render_target.clone());

    loop {
        // Get required scaling value
        let scale: f32 = f32::min(
            screen_width(fc) / VIRTUAL_WIDTH,
            screen_height(fc) / VIRTUAL_HEIGHT,
        );

        // Mouse position in the virtual screen
        let virtual_mouse_pos = Vec2 {
            x: (mouse_position(fc).0 - (screen_width(fc) - (VIRTUAL_WIDTH * scale)) * 0.5) / scale,
            y: (mouse_position(fc).1 - (screen_height(fc) - (VIRTUAL_HEIGHT * scale)) * 0.5)
                / scale,
        };

        // ------------------------------------------------------------------------
        // Begin drawing the virtual screen to 'render_target'
        // ------------------------------------------------------------------------
        set_camera(fc, &render_target_cam);

        clear_background(fc, LIGHTGRAY);

        draw_text(fc, "Hello Letterbox", 20.0, 20.0, 30.0, DARKGRAY);
        draw_circle(
            fc,
            VIRTUAL_WIDTH / 2.0 - 65.0,
            VIRTUAL_HEIGHT / 2.0,
            35.0,
            RED,
        );
        draw_circle(
            fc,
            VIRTUAL_WIDTH / 2.0 + 65.0,
            VIRTUAL_HEIGHT / 2.0,
            35.0,
            BLUE,
        );
        draw_circle(
            fc,
            VIRTUAL_WIDTH / 2.0,
            VIRTUAL_HEIGHT / 2.0 - 65.0,
            35.0,
            YELLOW,
        );

        draw_circle(fc, virtual_mouse_pos.x, virtual_mouse_pos.y, 15.0, BLACK);

        // ------------------------------------------------------------------------
        // Begin drawing the window screen
        // ------------------------------------------------------------------------
        set_default_camera(fc);

        clear_background(fc, BLACK); // Will be the letterbox color

        // Draw 'render_target' to window screen, porperly scaled and letterboxed
        draw_texture_ex(
            fc,
            &render_target.texture,
            (screen_width(fc) - (VIRTUAL_WIDTH * scale)) * 0.5,
            (screen_height(fc) - (VIRTUAL_HEIGHT * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIRTUAL_WIDTH * scale, VIRTUAL_HEIGHT * scale)),
                flip_y: true, // Must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        );
    }
}

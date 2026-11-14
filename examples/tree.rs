use {
    frogquad::{
        Color, FrogCx,
        camera::{Camera2D, set_camera},
        conf::Conf,
        shapes::{draw_circle, draw_rectangle},
        time::get_time,
        window::{clear_background, get_internal_gl},
    },
    glam::vec2,
};

fn tree(fc: &mut FrogCx, time: f64, deep: u32, angle: f32, tall: f32) {
    if deep >= 8 {
        return;
    }

    // we can use normal frogquad drawing API here
    draw_rectangle(fc, -0.01 / 2., 0., 0.01, tall, Color::DARKGRAY);

    // and we can also modify internal frogquad render state with "gl" reference
    get_internal_gl(fc)
        .quad_gl
        .push_model_matrix(glam::Mat4::from_translation(glam::vec3(0., tall, 0.)));

    // right leaf
    get_internal_gl(fc)
        .quad_gl
        .push_model_matrix(glam::Mat4::from_rotation_z(angle + time.sin() as f32 * 0.1));
    tree(fc, time, deep + 1, angle * 0.7, tall * 0.8);
    get_internal_gl(fc).quad_gl.pop_model_matrix();

    // left leaf
    get_internal_gl(fc)
        .quad_gl
        .push_model_matrix(glam::Mat4::from_rotation_z(
            -angle - time.cos() as f32 * 0.1,
        ));
    tree(fc, time, deep + 1, angle * 0.7, tall * 0.8);
    get_internal_gl(fc).quad_gl.pop_model_matrix();

    get_internal_gl(fc).quad_gl.pop_model_matrix();
}

struct TreeApp {}

impl frogquad::App for TreeApp {
    fn update(&mut self, fc: &mut FrogCx) {
        clear_background(fc, Color::LIGHTGRAY);

        draw_circle(fc, 0., 0., 0.03, Color::DARKGRAY);
        let time = get_time(fc);
        tree(fc, time, 0, 1., 0.3);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Tree"), |fc| {
        let camera = Camera2D {
            zoom: vec2(1., 1.),
            target: vec2(0.0, 0.5),
            ..Default::default()
        };

        set_camera(fc, &camera);
        Box::new(TreeApp {})
    });
}

use {
    frogquad::{Camera2D, Color, FrogCx},
    glam::vec2,
};

fn tree(fc: &mut FrogCx, time: f64, deep: u32, angle: f32, tall: f32) {
    if deep >= 8 {
        return;
    }

    // we can use normal frogquad drawing API here
    fc.draw_rectangle(-0.01 / 2., 0., 0.01, tall, Color::DARKGRAY);

    // and we can also modify internal frogquad render state with "gl" reference
    fc.gl
        .push_model_matrix(glam::Mat4::from_translation(glam::vec3(0., tall, 0.)));

    // right leaf
    fc.gl
        .push_model_matrix(glam::Mat4::from_rotation_z(angle + time.sin() as f32 * 0.1));
    tree(fc, time, deep + 1, angle * 0.7, tall * 0.8);
    fc.gl.pop_model_matrix();

    // left leaf
    fc.gl.push_model_matrix(glam::Mat4::from_rotation_z(
        -angle - time.cos() as f32 * 0.1,
    ));
    tree(fc, time, deep + 1, angle * 0.7, tall * 0.8);
    fc.gl.pop_model_matrix();

    fc.gl.pop_model_matrix();
}

struct TreeApp {}

impl frogquad::App for TreeApp {
    fn update(&mut self, fc: &mut FrogCx) {
        fc.clear_background(Color::LIGHTGRAY);

        fc.draw_circle(0., 0., 0.03, Color::DARKGRAY);
        let time = fc.time();
        tree(fc, time, 0, 1., 0.3);
    }
}

fn main() {
    frogquad::start("Tree", |fc| {
        let camera = Camera2D {
            zoom: vec2(1., 1.),
            target: vec2(0.0, 0.5),
            ..Default::default()
        };

        fc.set_camera(&camera);
        TreeApp {}
    });
}

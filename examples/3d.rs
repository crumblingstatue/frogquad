use {
    frogquad::{
        Color, FrogCx,
        camera::Camera3D,
        conf::Conf,
        models::{draw_cube, draw_cube_wires, draw_grid, draw_plane, draw_sphere},
        texture::{Texture2D, load_texture},
    },
    glam::{vec2, vec3},
    pollster::FutureExt,
};

struct App3d {
    ferris: Texture2D,
    rust_logo: Texture2D,
}

impl App3d {
    fn new(fc: &mut FrogCx) -> Self {
        Self {
            rust_logo: load_texture(fc, "examples/rust.png").block_on().unwrap(),
            ferris: load_texture(fc, "examples/ferris.png").block_on().unwrap(),
        }
    }
}

impl frogquad::App for App3d {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::LIGHTGRAY);

        // Going 3d!

        fc.set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(fc, 20, 1., Color::BLACK, Color::GRAY);

        draw_cube_wires(fc, vec3(0., 1., -6.), vec3(2., 2., 2.), Color::DARKGREEN);
        draw_cube_wires(fc, vec3(0., 1., 6.), vec3(2., 2., 2.), Color::DARKBLUE);
        draw_cube_wires(fc, vec3(2., 1., 2.), vec3(2., 2., 2.), Color::YELLOW);

        draw_plane(
            fc,
            vec3(-8., 0., -8.),
            vec2(5., 5.),
            Some(&self.ferris),
            Color::WHITE,
        );

        draw_cube(
            fc,
            vec3(-5., 1., -2.),
            vec3(2., 2., 2.),
            Some(&self.rust_logo),
            Color::WHITE,
        );
        draw_cube(
            fc,
            vec3(-5., 1., 2.),
            vec3(2., 2., 2.),
            Some(&self.ferris),
            Color::WHITE,
        );
        draw_cube(
            fc,
            vec3(2., 0., -2.),
            vec3(0.4, 0.4, 0.4),
            None,
            Color::BLACK,
        );

        draw_sphere(fc, vec3(-8., 0., 0.), 1., None, Color::BLUE);

        // Back to screen space, render some text

        fc.set_default_camera();
        fc.draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, Color::BLACK);
    }
}

fn main() {
    frogquad::start(Conf::from_title("3D"), |fc| Box::new(App3d::new(fc)));
}

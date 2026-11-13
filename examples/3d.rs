use frogquad::prelude::*;

fn main() {
    frogquad::start(Conf::from_title("3D"), main_());
}
async fn main_() {
    let rust_logo = load_texture(fc, "examples/rust.png").await.unwrap();
    let ferris = load_texture(fc, "examples/ferris.png").await.unwrap();

    loop {
        clear_background(fc, LIGHTGRAY);

        // Going 3d!

        set_camera(
            fc,
            &Camera3D {
                position: vec3(-20., 15., 0.),
                up: vec3(0., 1., 0.),
                target: vec3(0., 0., 0.),
                ..Default::default()
            },
        );

        draw_grid(fc, 20, 1., BLACK, GRAY);

        draw_cube_wires(fc, vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(fc, vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(fc, vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

        draw_plane(fc, vec3(-8., 0., -8.), vec2(5., 5.), Some(&ferris), WHITE);

        draw_cube(
            fc,
            vec3(-5., 1., -2.),
            vec3(2., 2., 2.),
            Some(&rust_logo),
            WHITE,
        );
        draw_cube(
            fc,
            vec3(-5., 1., 2.),
            vec3(2., 2., 2.),
            Some(&ferris),
            WHITE,
        );
        draw_cube(fc, vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);

        draw_sphere(fc, vec3(-8., 0., 0.), 1., None, BLUE);

        // Back to screen space, render some text

        set_default_camera(fc);
        draw_text(fc, "WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}

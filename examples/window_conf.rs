use frogquad::{get_context, prelude::*};

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: true,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[frogquad::main(window_conf)]
async fn main() {
    let fc = get_context();
    loop {
        clear_background(fc, WHITE);
        next_frame().await
    }
}

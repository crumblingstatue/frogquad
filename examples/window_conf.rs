use frogquad::prelude::*;

fn window_conf() -> miniquad::conf::Conf {
    miniquad::conf::Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: true,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn main() {
    frogquad::start(window_conf(), main_());
}
async fn main_() {
    loop {
        clear_background(fc, WHITE);
    }
}

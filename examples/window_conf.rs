use frogquad::{color, window::clear_background};

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

struct WindowConfApp {}

impl frogquad::App for WindowConfApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        loop {
            clear_background(fc, color::WHITE);
        }
    }
}

fn main() {
    frogquad::start(window_conf().into(), |_fc| Box::new(WindowConfApp {}));
}

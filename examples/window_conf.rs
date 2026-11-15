use frogquad::Color;

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
            fc.clear_background(Color::WHITE);
        }
    }
}

fn main() {
    frogquad::start(window_conf(), |_fc| Box::new(WindowConfApp {}));
}

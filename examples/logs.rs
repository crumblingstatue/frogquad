//! frogquad expose all the logging macros.
//! They will use browser console, android console or just stdout depending on the platform.
//! Those macros are the recommended way to output debug traces and logs.

use frogquad::{color, conf::Conf, window::clear_background};

struct LogsApp {}

impl frogquad::App for LogsApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, color::LIGHTGRAY);

        miniquad::debug!("Still alive!");
    }
}

fn main() {
    frogquad::start(Conf::from_title("Logs"), |_fc| {
        miniquad::debug!("This is a debug message");
        miniquad::info!("and info message");
        miniquad::error!("and errors, the red ones!");
        miniquad::warn!("Or warnings, the yellow ones.");
        Box::new(LogsApp {})
    });
}

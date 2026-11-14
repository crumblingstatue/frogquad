//! frogquad expose all the logging macros.
//! They will use browser console, android console or just stdout depending on the platform.
//! Those macros are the recommended way to output debug traces and logs.

use frogquad::prelude::*;

struct LogsApp {}

impl frogquad::App for LogsApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, LIGHTGRAY);

        debug!("Still alive!");
    }
}

fn main() {
    frogquad::start(Conf::from_title("Logs"), |_fc| {
        debug!("This is a debug message");
        info!("and info message");
        error!("and errors, the red ones!");
        warn!("Or warnings, the yellow ones.");
        Box::new(LogsApp {})
    });
}

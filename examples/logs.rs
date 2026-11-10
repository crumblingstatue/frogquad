//! frogquad expose all the logging macros.
//! They will use browser console, android console or just stdout depending on the platform.
//! Those macros are the recommended way to output debug traces and logs.

use frogquad::{get_context, prelude::*};

fn main() {
    frogquad::start("Logs", main_());
}
async fn main_() {
    let fc = get_context();
    debug!("This is a debug message");
    info!("and info message");
    error!("and errors, the red ones!");
    warn!("Or warnings, the yellow ones.");

    loop {
        clear_background(fc, LIGHTGRAY);

        debug!("Still alive!");

        next_frame().await
    }
}

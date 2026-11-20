//! Window and associated to window rendering context related functions.

use crate::FrogCx;

use crate::color::Color;

impl FrogCx {
    /// Fill window background with solid color.
    /// Note: even when "clear_background" is not called explicitly,
    /// the screen will be cleared at the beginning of the frame.
    pub fn clear_background(&mut self, color: Color) {
        self.gl.clear(&mut *self.backend, color);
    }
    /// Screen width divided by the DPI scale
    pub fn screen_width(&self) -> f32 {
        self.screen_width / miniquad::window::dpi_scale()
    }
    /// Screen height divided by the DPI scale
    pub fn screen_height(&self) -> f32 {
        self.screen_height / miniquad::window::dpi_scale()
    }
}

/// Request the window size to be the given value. This takes DPI into account.
///
/// Note that the OS might decide to give a different size. Additionally, the size in frogquad won't be updated until the next `next_frame().await`.
pub fn request_new_screen_size(width: f32, height: f32) {
    miniquad::window::set_window_size(
        (width * miniquad::window::dpi_scale()) as u32,
        (height * miniquad::window::dpi_scale()) as u32,
    );
    // We do not set the context.screen_width and context.screen_height here.
    // After `set_window_size` is called, EventHandlerFree::resize will be invoked, setting the size correctly.
    // Because the OS might decide to give a different screen dimension, setting the context.screen_* here would be confusing.
}

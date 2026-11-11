//! Window and associated to window rendering context related functions.

use crate::FrogCx;

use crate::color::Color;

// miniquad is re-exported for the use in combination with `get_internal_gl`
pub use miniquad;

/// Fill window background with solid color.
/// Note: even when "clear_background" is not called explicitly,
/// the screen will be cleared at the beginning of the frame.
pub fn clear_background(fc: &mut FrogCx, color: Color) {
    fc.gl.clear(&mut *fc.quad_context, color);
}

#[doc(hidden)]
pub fn gl_set_drawcall_buffer_capacity(fc: &mut FrogCx, max_vertices: usize, max_indices: usize) {
    fc.gl
        .update_drawcall_capacity(&mut *fc.quad_context, max_vertices, max_indices);
}

pub struct InternalGlContext<'a> {
    pub quad_context: &'a mut dyn miniquad::RenderingBackend,
    pub quad_gl: &'a mut crate::quad_gl::QuadGl,
}

/// Draw all the batched stuff and reset the internal state cache.
/// May be helpful for combining frogquad's drawing with raw miniquad/opengl calls.
pub fn flush_internal_gl(fc: &mut FrogCx) {
    fc.perform_render_passes();
}

pub fn get_internal_gl<'a>(fc: &'a mut FrogCx) -> InternalGlContext<'a> {
    InternalGlContext {
        quad_context: &mut *fc.quad_context,
        quad_gl: &mut fc.gl,
    }
}

pub fn screen_width(fc: &FrogCx) -> f32 {
    fc.screen_width / miniquad::window::dpi_scale()
}

pub fn screen_height(fc: &FrogCx) -> f32 {
    fc.screen_height / miniquad::window::dpi_scale()
}

pub fn screen_dpi_scale() -> f32 {
    miniquad::window::dpi_scale()
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

/// Toggle whether the window is fullscreen.
pub fn set_fullscreen(fullscreen: bool) {
    miniquad::window::set_fullscreen(fullscreen);
}

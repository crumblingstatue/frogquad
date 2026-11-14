//! Cross platform system time access and FPS counters.

use crate::{FrogCx, color::Color, text::draw_text};

/// Draws the current FPS on the screen. For extra customization, please use `draw_text` instead.
pub fn draw_fps(fc: &mut FrogCx) {
    let fps = get_fps(fc);
    draw_text(fc, format!("FPS: {fps}"), 0., 16., 32., Color::WHITE);
}

/// Returns current FPS
pub fn get_fps(fc: &mut FrogCx) -> i32 {
    (1. / fc.frame_time) as i32
}

/// Returns duration in seconds of the last frame drawn
pub fn get_frame_time(fc: &mut FrogCx) -> f32 {
    fc.frame_time as f32
}

/// Returns elapsed wall-clock time in seconds since start
///
/// Note that as real world time progresses during computation,
/// the value returned will change. Therefore if you want
/// your game logic update to happen at the same *in-game* time
/// for all game objects, you should call this function once
/// save the value and reuse it throughout your code.
pub fn get_time(fc: &mut FrogCx) -> f64 {
    miniquad::date::now() - fc.start_time
}

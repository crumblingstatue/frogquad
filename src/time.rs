//! Cross platform system time access and FPS counters.

use crate::{FrogCx, color::Color};

impl FrogCx {
    /// Draws the current FPS on the screen. For extra customization, please use `draw_text` instead.
    pub fn draw_fps(&mut self) {
        let fps = self.get_fps();
        self.draw_text(format!("FPS: {fps}"), 0., 16., 32., Color::WHITE);
    }

    /// Returns current FPS
    pub fn get_fps(&mut self) -> i32 {
        (1. / self.frame_time) as i32
    }

    /// Returns duration in seconds of the last frame drawn
    pub fn get_frame_time(&mut self) -> f32 {
        self.frame_time as f32
    }

    /// Returns elapsed wall-clock time in seconds since start
    ///
    /// Note that as real world time progresses during computation,
    /// the value returned will change. Therefore if you want
    /// your game logic update to happen at the same *in-game* time
    /// for all game objects, you should call this function once
    /// save the value and reuse it throughout your code.
    pub fn get_time(&mut self) -> f64 {
        miniquad::date::now() - self.start_time
    }
}

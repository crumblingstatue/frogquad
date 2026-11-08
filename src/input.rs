//! Cross-platform mouse, keyboard (and gamepads soon) module.

use std::collections::HashSet;

use crate::DroppedFile;
use crate::FrogCx;
use crate::Vec2;
use crate::prelude::screen_height;
use crate::prelude::screen_width;
pub use miniquad::{KeyCode, MouseButton};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TouchPhase {
    Started,
    Stationary,
    Moved,
    Ended,
    Cancelled,
}

impl From<miniquad::TouchPhase> for TouchPhase {
    fn from(miniquad_phase: miniquad::TouchPhase) -> TouchPhase {
        match miniquad_phase {
            miniquad::TouchPhase::Started => TouchPhase::Started,
            miniquad::TouchPhase::Moved => TouchPhase::Moved,
            miniquad::TouchPhase::Ended => TouchPhase::Ended,
            miniquad::TouchPhase::Cancelled => TouchPhase::Cancelled,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Touch {
    pub id: u64,
    pub phase: TouchPhase,
    pub position: Vec2,
}

/// Constrain mouse to window
pub fn set_cursor_grab(fc: &mut FrogCx, grab: bool) {
    fc.cursor_grabbed = grab;
    miniquad::window::set_cursor_grab(grab);
}

/// Set mouse cursor visibility
pub fn show_mouse(shown: bool) {
    miniquad::window::show_mouse(shown);
}

/// Return mouse position in pixels.
pub fn mouse_position(fc: &mut FrogCx) -> (f32, f32) {
    (
        fc.mouse_position.x / miniquad::window::dpi_scale(),
        fc.mouse_position.y / miniquad::window::dpi_scale(),
    )
}

/// Return mouse position in range [-1; 1].
pub fn mouse_position_local(fc: &mut FrogCx) -> Vec2 {
    let (pixels_x, pixels_y) = mouse_position(fc);

    convert_to_local(Vec2::new(pixels_x, pixels_y))
}

/// Returns the difference between the current mouse position and the mouse position on the previous frame.
pub fn mouse_delta_position(fc: &mut FrogCx) -> Vec2 {
    let current_position = mouse_position_local(fc);
    let last_position = fc.last_mouse_position.unwrap_or(current_position);

    // Calculate the delta
    last_position - current_position
}

/// This is set to true by default, meaning touches will raise mouse events in addition to raising touch events.
/// If set to false, touches won't affect mouse events.
pub fn is_simulating_mouse_with_touch(fc: &mut FrogCx) -> bool {
    fc.simulate_mouse_with_touch
}

/// This is set to true by default, meaning touches will raise mouse events in addition to raising touch events.
/// If set to false, touches won't affect mouse events.
pub fn simulate_mouse_with_touch(fc: &mut FrogCx, option: bool) {
    fc.simulate_mouse_with_touch = option;
}

/// Return touches with positions in pixels.
pub fn touches(fc: &mut FrogCx) -> Vec<Touch> {
    fc.touches.values().cloned().collect()
}

/// Return touches with positions in range [-1; 1].
pub fn touches_local(fc: &mut FrogCx) -> Vec<Touch> {
    fc.touches
        .values()
        .map(|touch| {
            let mut touch = touch.clone();
            touch.position = convert_to_local(touch.position);
            touch
        })
        .collect()
}

pub fn mouse_wheel(fc: &mut FrogCx) -> (f32, f32) {
    (fc.mouse_wheel.x, fc.mouse_wheel.y)
}

/// Detect if the key has been pressed once
pub fn is_key_pressed(fc: &mut FrogCx, key_code: KeyCode) -> bool {
    fc.keys_pressed.contains(&key_code)
}

/// Detect if the key is being pressed
pub fn is_key_down(fc: &mut FrogCx, key_code: KeyCode) -> bool {
    fc.keys_down.contains(&key_code)
}

/// Detect if the key has been released this frame
pub fn is_key_released(fc: &mut FrogCx, key_code: KeyCode) -> bool {
    fc.keys_released.contains(&key_code)
}

/// Detect if any key is being pressed
pub fn is_any_key_down(fc: &mut FrogCx) -> bool {
    fc.keys_down.len() > 0
}

/// Return the last pressed char.
/// Each "get_char_pressed" call will consume a character from the input queue.
pub fn get_char_pressed(fc: &mut FrogCx) -> Option<char> {
    fc.chars_pressed_queue.pop_front()
}

/// Return the last pressed key.
pub fn get_last_key_pressed(fc: &mut FrogCx) -> Option<KeyCode> {
    // TODO: this will return a random key from keys_pressed HashMap instead of the last one, fix me later
    fc.keys_pressed.iter().next().cloned()
}

pub fn get_keys_pressed(fc: &mut FrogCx) -> HashSet<KeyCode> {
    fc.keys_pressed.clone()
}

pub fn get_keys_down(fc: &mut FrogCx) -> HashSet<KeyCode> {
    fc.keys_down.clone()
}

pub fn get_keys_released(fc: &mut FrogCx) -> HashSet<KeyCode> {
    fc.keys_released.clone()
}

/// Clears input queue
pub fn clear_input_queue(fc: &mut FrogCx) {
    fc.chars_pressed_queue.clear();
    fc.mouse_pressed.clear();
    fc.keys_pressed.clear();
}

/// Detect if the button is being pressed
pub fn is_mouse_button_down(fc: &mut FrogCx, btn: MouseButton) -> bool {
    fc.mouse_down.contains(&btn)
}

/// Detect if the button has been pressed once
pub fn is_mouse_button_pressed(fc: &mut FrogCx, btn: MouseButton) -> bool {
    fc.mouse_pressed.contains(&btn)
}

/// Detect if the button has been released this frame
pub fn is_mouse_button_released(fc: &mut FrogCx, btn: MouseButton) -> bool {
    fc.mouse_released.contains(&btn)
}

/// Convert a position in pixels to a position in the range [-1; 1].
fn convert_to_local(pixel_pos: Vec2) -> Vec2 {
    Vec2::new(pixel_pos.x / screen_width(), pixel_pos.y / screen_height()) * 2.0
        - Vec2::new(1.0, 1.0)
}

/// Prevents quit
pub fn prevent_quit(fc: &mut FrogCx) {
    fc.prevent_quit_event = true;
}

/// Detect if quit has been requested
pub fn is_quit_requested(fc: &mut FrogCx) -> bool {
    fc.quit_requested
}

/// Gets the files which have been dropped on the window.
pub fn get_dropped_files(fc: &mut FrogCx) -> Vec<DroppedFile> {
    fc.dropped_files()
}

/// Functions for advanced input processing.
///
/// Functions in this module should be used by external tools that uses miniquad system, like different UI libraries. User shouldn't use this function.
pub mod utils {
    use crate::FrogCx;

    /// Register input subscriber. Returns subscriber identifier that must be used in `repeat_all_miniquad_input`.
    pub fn register_input_subscriber(fc: &mut FrogCx) -> usize {
        fc.input_events.push(vec![]);

        fc.input_events.len() - 1
    }

    /// Repeats all events that came since last call of this function with current value of `subscriber`. This function must be called at each frame.
    pub fn repeat_all_miniquad_input<T: miniquad::EventHandler>(
        fc: &mut FrogCx,
        t: &mut T,
        subscriber: usize,
    ) {
        for event in &fc.input_events[subscriber] {
            event.repeat(t);
        }
        fc.input_events[subscriber].clear();
    }
}

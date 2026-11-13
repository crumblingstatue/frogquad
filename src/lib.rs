//!
//! `frogquad` is a simple and easy to use game library for Rust programming language.
//!
//! `frogquad` attempts to avoid any rust-specific programming concepts like lifetimes/borrowing, making it very friendly for rust beginners.
//!
//! ## Supported platforms
//!
//! * PC: Windows/Linux/MacOS
//! * HTML5
//! * Android
//! * IOS
//!
//! ## Features
//!
//! * Same code for all supported platforms, no platform dependent defines required
//! * Efficient 2D rendering with automatic geometry batching
//! * Minimal amount of dependencies: build after `cargo clean` takes only 16s on x230(~6years old laptop)
//! * Immediate mode UI library included
//! * Single command deploy for both WASM and Android [build instructions](https://github.com/not-fl3/miniquad/#building-examples)
//! # Example
//! ```no_run
//! use frogquad::prelude::*;
//!
//! fn main() { frogquad::start(Conf::from_title("BasicShapes"), main_()); }
//! async fn main_() {
//!     let fc = frogquad::get_context();
//!     loop {
//!         clear_background(fc, RED);
//!
//!         draw_line(fc, 40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
//!         draw_rectangle(fc, screen_width(fc) / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
//!         draw_circle(fc, screen_width(fc) - 30.0, screen_height(fc) - 30.0, 15.0, YELLOW);
//!         draw_text(fc, "HELLO", 20.0, 20.0, 20.0, DARKGRAY);
//!
//!         next_frame().await
//!     }
//! }
//!```

use std::collections::{HashMap, HashSet, VecDeque};

mod exec;
mod quad_gl;
mod tobytes;

pub mod camera;
pub mod color;
pub mod file;
pub mod input;
pub mod material;
pub mod math;
pub mod models;
pub mod shapes;
pub mod text;
pub mod texture;
pub mod time;
pub mod window;

pub mod prelude;

mod error;

pub use error::Error;

/// Cross platform random generator.
pub mod rand {
    pub use quad_rand::*;
}

#[cfg(not(feature = "log-rs"))]
/// Logging macros, available with miniquad "log-impl" feature.
pub mod logging {
    pub use miniquad::{debug, error, info, trace, warn};
}
#[cfg(feature = "log-rs")]
// Use logging facade
pub use ::log as logging;
pub use miniquad;
use miniquad::{EventHandler, KeyCode, KeyMods, MouseButton, TouchPhase, date};

use crate::quad_gl::MagicSnapshotter;
use crate::text::Font;
use crate::texture::TexturesContext;
use crate::{
    color::{Color, colors::*},
    quad_gl::QuadGl,
    texture::TextureHandle,
};

use glam::{Mat4, Vec2, vec2};

/// Frog context C:
pub struct FrogCx {
    screen_width: f32,
    screen_height: f32,

    simulate_mouse_with_touch: bool,

    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,
    mouse_down: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,
    touches: HashMap<u64, input::Touch>,
    chars_pressed_queue: VecDeque<char>,
    mouse_position: Vec2,
    last_mouse_position: Option<Vec2>,
    mouse_wheel: Vec2,

    prevent_quit_event: bool,
    quit_requested: bool,

    cursor_grabbed: bool,

    input_events: Vec<Vec<MiniquadInputEvent>>,

    pub gl: QuadGl,
    snapshotter: MagicSnapshotter,
    camera_matrix: Option<Mat4>,

    fonts_storage: text::FontsStorage,

    pc_assets_folder: Option<String>,

    start_time: f64,
    last_frame_time: f64,
    frame_time: f64,

    camera_stack: Vec<camera::CameraState>,
    texture_batcher: texture::Batcher,

    pub quad_context: Box<dyn miniquad::RenderingBackend>,

    default_filter_mode: crate::quad_gl::FilterMode,
    pub textures: crate::texture::TexturesContext,

    update_on: conf::UpdateTrigger,

    dropped_files: Vec<DroppedFile>,
}

#[derive(Clone)]
enum MiniquadInputEvent {
    MouseMotion {
        x: f32,
        y: f32,
    },
    MouseWheel {
        x: f32,
        y: f32,
    },
    MouseButtonDown {
        x: f32,
        y: f32,
        btn: MouseButton,
    },
    MouseButtonUp {
        x: f32,
        y: f32,
        btn: MouseButton,
    },
    Char {
        character: char,
        modifiers: KeyMods,
        repeat: bool,
    },
    KeyDown {
        keycode: KeyCode,
        modifiers: KeyMods,
        repeat: bool,
    },
    KeyUp {
        keycode: KeyCode,
        modifiers: KeyMods,
    },
    Touch {
        phase: TouchPhase,
        id: u64,
        x: f32,
        y: f32,
    },
    WindowMinimized,
    WindowRestored,
}

impl MiniquadInputEvent {
    fn repeat<T: miniquad::EventHandler>(&self, t: &mut T) {
        use crate::MiniquadInputEvent::*;
        match self {
            MouseMotion { x, y } => t.mouse_motion_event(*x, *y),
            MouseWheel { x, y } => t.mouse_wheel_event(*x, *y),
            MouseButtonDown { x, y, btn } => t.mouse_button_down_event(*btn, *x, *y),
            MouseButtonUp { x, y, btn } => t.mouse_button_up_event(*btn, *x, *y),
            Char {
                character,
                modifiers,
                repeat,
            } => t.char_event(*character, *modifiers, *repeat),
            KeyDown {
                keycode,
                modifiers,
                repeat,
            } => t.key_down_event(*keycode, *modifiers, *repeat),
            KeyUp { keycode, modifiers } => t.key_up_event(*keycode, *modifiers),
            Touch { phase, id, x, y } => t.touch_event(*phase, *id, *x, *y),
            WindowMinimized => t.window_minimized_event(),
            WindowRestored => t.window_restored_event(),
        }
    }
}

impl FrogCx {
    const DEFAULT_BG_COLOR: Color = BLACK;

    pub fn new(
        update_on: conf::UpdateTrigger,
        default_filter_mode: miniquad::FilterMode,
        draw_call_vertex_capacity: usize,
        draw_call_index_capacity: usize,
    ) -> FrogCx {
        let mut ctx: Box<dyn miniquad::RenderingBackend> =
            miniquad::window::new_rendering_backend();
        let (screen_width, screen_height) = miniquad::window::screen_size();

        FrogCx {
            screen_width,
            screen_height,

            simulate_mouse_with_touch: true,

            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            chars_pressed_queue: VecDeque::new(),
            mouse_down: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_released: HashSet::new(),
            touches: HashMap::new(),
            mouse_position: vec2(0., 0.),
            last_mouse_position: None,
            mouse_wheel: vec2(0., 0.),

            prevent_quit_event: false,
            quit_requested: false,

            cursor_grabbed: false,

            input_events: Vec::new(),

            camera_matrix: None,
            gl: QuadGl::new(
                &mut *ctx,
                draw_call_vertex_capacity,
                draw_call_index_capacity,
            ),
            snapshotter: MagicSnapshotter::new(&mut *ctx),

            fonts_storage: text::FontsStorage::new(&mut *ctx),
            texture_batcher: texture::Batcher::new(&mut *ctx),
            camera_stack: vec![],

            pc_assets_folder: None,

            start_time: miniquad::date::now(),
            last_frame_time: miniquad::date::now(),
            frame_time: 1. / 60.,

            quad_context: ctx,

            default_filter_mode,
            textures: crate::texture::TexturesContext::new(),
            update_on,

            dropped_files: Vec::new(),
        }
    }

    /// Returns the handle for this texture.
    pub fn raw_miniquad_id(
        gl: &QuadGl,
        textures: &TexturesContext,
        handle: &TextureHandle,
    ) -> miniquad::TextureId {
        match handle {
            TextureHandle::Unmanaged(texture) => *texture,
            TextureHandle::Managed(texture) => {
                textures.texture(texture.0).unwrap_or(gl.white_texture)
            }
            TextureHandle::ManagedWeak(texture) => {
                textures.texture(*texture).unwrap_or(gl.white_texture)
            }
        }
    }

    /// Returns the files which have been dropped onto the window.
    pub fn dropped_files(&mut self) -> Vec<DroppedFile> {
        std::mem::take(&mut self.dropped_files)
    }

    fn begin_frame(&mut self) {
        let color = Self::DEFAULT_BG_COLOR;

        self.quad_context
            .clear(Some((color.r, color.g, color.b, color.a)), None, None);
        self.gl.reset();
    }

    fn end_frame(&mut self) {
        self.perform_render_passes();

        let screen_mat = self.pixel_perfect_projection_matrix();
        self.gl.draw(
            &mut *self.quad_context,
            &self.textures,
            screen_mat,
            &mut self.snapshotter,
        );

        self.quad_context.commit_frame();

        self.mouse_wheel = Vec2::new(0., 0.);
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_pressed.clear();
        self.mouse_released.clear();
        self.last_mouse_position = Some(crate::prelude::mouse_position_local(self));

        self.quit_requested = false;

        self.textures.garbage_collect(&mut *self.quad_context);

        // remove all touches that were Ended or Cancelled
        self.touches.retain(|_, touch| {
            touch.phase != input::TouchPhase::Ended && touch.phase != input::TouchPhase::Cancelled
        });

        // change all Started or Moved touches to Stationary
        for touch in self.touches.values_mut() {
            if touch.phase == input::TouchPhase::Started || touch.phase == input::TouchPhase::Moved
            {
                touch.phase = input::TouchPhase::Stationary;
            }
        }

        self.dropped_files.clear();
    }

    pub(crate) fn pixel_perfect_projection_matrix(&self) -> glam::Mat4 {
        let (width, height) = miniquad::window::screen_size();

        let dpi = miniquad::window::dpi_scale();

        glam::Mat4::orthographic_rh_gl(0., width / dpi, height / dpi, 0., -1., 1.)
    }

    pub fn projection_matrix(&self) -> glam::Mat4 {
        if let Some(matrix) = self.camera_matrix {
            matrix
        } else {
            self.pixel_perfect_projection_matrix()
        }
    }

    pub fn perform_render_passes(&mut self) {
        let matrix = self.projection_matrix();

        self.gl.draw(
            &mut *self.quad_context,
            &self.textures,
            matrix,
            &mut self.snapshotter,
        );
    }

    pub fn default_font(&self) -> &Font {
        &self.fonts_storage.default_font
    }

    pub fn request_quit(&mut self) {
        self.quit_requested = true;
    }
}

struct Stage {
    app: Box<dyn App>,
    fc: FrogCx,
}

impl EventHandler for Stage {
    fn resize_event(&mut self, width: f32, height: f32) {
        self.fc.screen_width = width;
        self.fc.screen_height = height;

        if miniquad::window::blocking_event_loop() {
            miniquad::window::schedule_update();
        }
    }

    fn raw_mouse_motion(&mut self, x: f32, y: f32) {
        if self.fc.cursor_grabbed {
            self.fc.mouse_position += Vec2::new(x, y);

            let event = MiniquadInputEvent::MouseMotion {
                x: self.fc.mouse_position.x,
                y: self.fc.mouse_position.y,
            };
            self.fc
                .input_events
                .iter_mut()
                .for_each(|arr| arr.push(event.clone()));
        }
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        if !self.fc.cursor_grabbed {
            self.fc.mouse_position = Vec2::new(x, y);

            self.fc
                .input_events
                .iter_mut()
                .for_each(|arr| arr.push(MiniquadInputEvent::MouseMotion { x, y }));
        }

        if self.fc.update_on.mouse_motion {
            miniquad::window::schedule_update();
        }
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.fc.mouse_wheel.x = x;
        self.fc.mouse_wheel.y = y;

        self.fc
            .input_events
            .iter_mut()
            .for_each(|arr| arr.push(MiniquadInputEvent::MouseWheel { x, y }));

        if self.fc.update_on.mouse_wheel {
            miniquad::window::schedule_update();
        }
    }

    fn mouse_button_down_event(&mut self, btn: MouseButton, x: f32, y: f32) {
        self.fc.mouse_down.insert(btn);
        self.fc.mouse_pressed.insert(btn);

        self.fc
            .input_events
            .iter_mut()
            .for_each(|arr| arr.push(MiniquadInputEvent::MouseButtonDown { x, y, btn }));

        if !self.fc.cursor_grabbed {
            self.fc.mouse_position = Vec2::new(x, y);
        }

        if self.fc.update_on.mouse_down {
            miniquad::window::schedule_update();
        }
    }

    fn mouse_button_up_event(&mut self, btn: MouseButton, x: f32, y: f32) {
        //     println!("btn = {}", btn as u32);
        self.fc.mouse_down.remove(&btn);
        self.fc.mouse_released.insert(btn);

        self.fc
            .input_events
            .iter_mut()
            .for_each(|arr| arr.push(MiniquadInputEvent::MouseButtonUp { x, y, btn }));

        if !self.fc.cursor_grabbed {
            self.fc.mouse_position = Vec2::new(x, y);
        }
        if self.fc.update_on.mouse_up {
            miniquad::window::schedule_update();
        }
    }

    fn mouse_leave_event(&mut self) {
        self.fc.mouse_released.extend(self.fc.mouse_down.drain());
    }

    fn mouse_enter_event(&mut self, btn: MouseButton, x: f32, y: f32) {
        if !self.fc.cursor_grabbed {
            self.fc.mouse_position = Vec2::new(x, y);
        }

        self.fc
            .input_events
            .iter_mut()
            .for_each(|arr| arr.push(MiniquadInputEvent::MouseButtonUp { x, y, btn }));
        if btn != MouseButton::Unknown {
            self.fc.mouse_down.insert(btn);
            self.fc.mouse_pressed.insert(btn);
        }

        if self.fc.update_on.mouse_up {
            miniquad::window::schedule_update();
        }
    }

    fn touch_event(&mut self, phase: TouchPhase, id: u64, x: f32, y: f32) {
        self.fc.touches.insert(
            id,
            input::Touch {
                id,
                phase: phase.into(),
                position: Vec2::new(x, y),
            },
        );

        if self.fc.simulate_mouse_with_touch {
            if phase == TouchPhase::Started {
                self.mouse_button_down_event(MouseButton::Left, x, y);
            }

            if phase == TouchPhase::Ended {
                self.mouse_button_up_event(MouseButton::Left, x, y);
            }

            if phase == TouchPhase::Moved {
                self.mouse_motion_event(x, y);
            }
        } else if self.fc.update_on.touch {
            miniquad::window::schedule_update();
        };

        self.fc
            .input_events
            .iter_mut()
            .for_each(|arr| arr.push(MiniquadInputEvent::Touch { phase, id, x, y }));

        if self.fc.update_on.mouse_down {
            miniquad::window::schedule_update();
        }
    }

    fn char_event(&mut self, character: char, modifiers: KeyMods, repeat: bool) {
        self.fc.chars_pressed_queue.push_back(character);

        self.fc.input_events.iter_mut().for_each(|arr| {
            arr.push(MiniquadInputEvent::Char {
                character,
                modifiers,
                repeat,
            });
        });
    }

    fn key_down_event(&mut self, keycode: KeyCode, modifiers: KeyMods, repeat: bool) {
        self.fc.keys_down.insert(keycode);
        if repeat == false {
            self.fc.keys_pressed.insert(keycode);
        }

        self.fc.input_events.iter_mut().for_each(|arr| {
            arr.push(MiniquadInputEvent::KeyDown {
                keycode,
                modifiers,
                repeat,
            });
        });
        if self
            .fc
            .update_on
            .specific_key
            .as_ref()
            .map_or(self.fc.update_on.key_down, |keys| keys.contains(&keycode))
        {
            miniquad::window::schedule_update();
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, modifiers: KeyMods) {
        self.fc.keys_down.remove(&keycode);
        self.fc.keys_released.insert(keycode);

        self.fc
            .input_events
            .iter_mut()
            .for_each(|arr| arr.push(MiniquadInputEvent::KeyUp { keycode, modifiers }));

        if miniquad::window::blocking_event_loop() {
            //miniquad::window::schedule_update();
        }
    }

    fn update(&mut self) {
        // Unless called every frame, cursor will not remain grabbed
        miniquad::window::set_cursor_grab(self.fc.cursor_grabbed);

        #[cfg(not(target_arch = "wasm32"))]
        {
            // TODO: consider making it a part of miniquad?
            std::thread::yield_now();
        }
    }

    fn files_dropped_event(&mut self) {
        for i in 0..miniquad::window::dropped_file_count() {
            self.fc.dropped_files.push(DroppedFile {
                path: miniquad::window::dropped_file_path(i),
                bytes: miniquad::window::dropped_file_bytes(i),
            });
        }
    }

    fn draw(&mut self) {
        {
            {
                self.fc.begin_frame();
            }

            self.app.update(&mut self.fc);

            {
                self.fc.end_frame();
            }
            self.fc.frame_time = date::now() - self.fc.last_frame_time;
            self.fc.last_frame_time = date::now();
            self.fc.gl.reset();
        }
    }

    fn window_restored_event(&mut self) {
        #[cfg(target_os = "android")]
        if miniquad::window::blocking_event_loop() {
            miniquad::window::schedule_update();
        }

        self.fc
            .input_events
            .iter_mut()
            .for_each(|arr| arr.push(MiniquadInputEvent::WindowRestored));
    }

    fn window_minimized_event(&mut self) {
        // Clear held down keys and button and announce them as released
        self.fc.mouse_released.extend(self.fc.mouse_down.drain());
        self.fc.keys_released.extend(self.fc.keys_down.drain());

        // Announce all touches as released
        for (_, touch) in self.fc.touches.iter_mut() {
            touch.phase = input::TouchPhase::Ended;
        }

        self.fc
            .input_events
            .iter_mut()
            .for_each(|arr| arr.push(MiniquadInputEvent::WindowMinimized));
    }

    fn quit_requested_event(&mut self) {
        if self.fc.prevent_quit_event {
            miniquad::window::cancel_quit();
            self.fc.quit_requested = true;
        }
    }
}

pub mod conf {
    #[derive(Default, Debug, Clone)]
    pub struct UpdateTrigger {
        pub key_down: bool,
        pub mouse_down: bool,
        pub mouse_up: bool,
        pub mouse_motion: bool,
        pub mouse_wheel: bool,
        pub specific_key: Option<Vec<crate::KeyCode>>,
        pub touch: bool,
    }

    #[derive(Debug)]
    pub struct Conf {
        pub miniquad_conf: miniquad::conf::Conf,
        /// With miniquad_conf.platform.blocking_event_loop = true,
        /// next_frame().await will never finish and will wait forever with
        /// zero CPU usage.
        /// update_on will tell frogquad when to proceed with the event loop.
        pub update_on: Option<UpdateTrigger>,
        pub default_filter_mode: miniquad::FilterMode,
        /// frogquad performs automatic and static batching for each
        /// draw_* call. For each draw call, it pre-allocate a huge cpu/gpu
        /// buffer to add vertices to. When it exceeds the buffer, it allocates the
        /// new one, marking the new draw call.
        ///
        /// Some examples when altering those values migh be convinient:
        /// - for huge 3d models that do not fit into a single draw call, increasing
        ///     the buffer size might be easier than splitting the model.
        /// - when each draw_* call got its own material,
        ///     buffer size might be reduced to save some memory
        pub draw_call_vertex_capacity: usize,
        pub draw_call_index_capacity: usize,
    }

    impl Default for Conf {
        fn default() -> Self {
            Self {
                miniquad_conf: miniquad::conf::Conf::default(),
                update_on: Some(UpdateTrigger::default()),
                default_filter_mode: miniquad::FilterMode::Linear,
                draw_call_vertex_capacity: 10000,
                draw_call_index_capacity: 5000,
            }
        }
    }

    impl Conf {
        pub fn from_title(title: &str) -> Self {
            Self {
                miniquad_conf: miniquad::conf::Conf {
                    window_title: title.to_string(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }
}

impl From<miniquad::conf::Conf> for conf::Conf {
    fn from(conf: miniquad::conf::Conf) -> conf::Conf {
        conf::Conf {
            miniquad_conf: conf,
            update_on: None,
            default_filter_mode: miniquad::FilterMode::Linear,
            draw_call_vertex_capacity: 10000,
            draw_call_index_capacity: 5000,
        }
    }
}

pub trait App {
    fn update(&mut self, fc: &mut FrogCx);
}

pub fn start(config: conf::Conf, app_init: impl FnOnce(&mut FrogCx) -> Box<dyn App> + 'static) {
    miniquad::start(config.miniquad_conf, move || {
        let mut fc = FrogCx::new(
            config.update_on.as_ref().cloned().unwrap_or_default(),
            config.default_filter_mode,
            config.draw_call_vertex_capacity,
            config.draw_call_index_capacity,
        );
        let app = app_init(&mut fc);

        Box::new(Stage { app, fc })
    });
}

/// Information about a dropped file.
pub struct DroppedFile {
    pub path: Option<std::path::PathBuf>,
    pub bytes: Option<Vec<u8>>,
}

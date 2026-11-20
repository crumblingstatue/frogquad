use crate::{
    FrogCx,
    math::Rect,
    texture::{RenderPass, RenderTarget},
};
use glam::{Mat4, Vec2, vec2, vec3};

/// Defines a camera used for viewing what is being rendered
pub trait Camera {
    /// The projection matrix of the camera
    fn matrix(&self, fc: &mut FrogCx) -> Mat4;
    /// Whether the camera is depth enabled
    fn depth_enabled(&self) -> bool;
    /// An optional render pass for the camera
    fn render_pass(&self) -> Option<RenderPass>;
    /// The viewport of the camera. If `None`, the default viewport is used
    fn viewport(&self) -> Option<(i32, i32, i32, i32)>;
}

/// A camera ideal for 2D drawing
#[derive(Debug)]
pub struct Camera2D {
    /// Rotation in degrees.
    pub rotation: f32,
    /// Scaling, should be (1.0, 1.0) by default.
    pub zoom: Vec2,
    /// Rotation and zoom origin.
    pub target: Vec2,
    /// Displacement from target.
    pub offset: Vec2,

    /// If "render_target" is set - camera will render to texture.
    ///
    /// Otherwise to the screen.
    pub render_target: Option<RenderTarget>,

    /// Part of the screen to render to.
    ///
    /// None means the whole screen.
    ///
    /// Viewport do not affect camera space, just the render position on the screen.
    ///
    /// Useful for things like splitscreen.
    pub viewport: Option<(i32, i32, i32, i32)>,
}

impl Camera2D {
    /// Will make camera space equals given rect.
    pub fn from_display_rect(rect: Rect) -> Camera2D {
        let target = vec2(rect.x + rect.w / 2., rect.y + rect.h / 2.);

        Camera2D {
            target,
            zoom: vec2(1. / rect.w * 2., -1. / rect.h * 2.),
            offset: vec2(0., 0.),
            rotation: 0.,

            render_target: None,
            viewport: None,
        }
    }
}

impl Default for Camera2D {
    fn default() -> Camera2D {
        Camera2D {
            zoom: vec2(1., 1.),
            offset: vec2(0., 0.),
            target: vec2(0., 0.),
            rotation: 0.,

            render_target: None,
            viewport: None,
        }
    }
}

impl Camera for Camera2D {
    fn matrix(&self, _fc: &mut FrogCx) -> Mat4 {
        // gleaned from https://github.com/raysan5/raylib/blob/master/src/core.c#L1528

        // The camera in world-space is set by
        //   1. Move it to target
        //   2. Rotate by -rotation and scale by (1/zoom)
        //      When setting higher scale, it's more intuitive for the world to become bigger (= camera become smaller),
        //      not for the camera getting bigger, hence the invert. Same deal with rotation.
        //   3. Move it by (-offset);
        //      Offset defines target transform relative to screen, but since we're effectively "moving" screen (camera)
        //      we need to do it into opposite direction (inverse transform)

        // Having camera transform in world-space, inverse of it gives the modelview transform.
        // Since (A*B*C)' = C'*B'*A', the modelview is
        //   1. Move to offset
        //   2. Rotate and Scale
        //   3. Move by -target
        let mat_origin = Mat4::from_translation(vec3(-self.target.x, -self.target.y, 0.0));
        let mat_rotation = Mat4::from_axis_angle(vec3(0.0, 0.0, 1.0), self.rotation.to_radians());
        let invert_y = if self.render_target.is_some() {
            1.0
        } else {
            -1.0
        };
        let mat_scale = Mat4::from_scale(vec3(self.zoom.x, self.zoom.y * invert_y, 1.0));
        let mat_translation = Mat4::from_translation(vec3(self.offset.x, self.offset.y, 0.0));

        mat_translation * ((mat_scale * mat_rotation) * mat_origin)
    }

    fn depth_enabled(&self) -> bool {
        false
    }

    fn render_pass(&self) -> Option<RenderPass> {
        self.render_target.as_ref().map(|rt| rt.render_pass.clone())
    }

    fn viewport(&self) -> Option<(i32, i32, i32, i32)> {
        self.viewport
    }
}

impl Camera2D {
    /// Returns the screen space position for a 2d camera world space position.
    ///
    /// Screen position in window space - from (0, 0) to (screen_width, screen_height()).
    /// When a viewport is set, returns coordinates within that viewport's screen region.
    pub fn world_to_screen(&self, fc: &mut FrogCx, point: Vec2) -> Vec2 {
        let dims = self
            .viewport()
            .map(|(vx, vy, vw, vh)| Rect {
                x: vx as f32,
                y: fc.screen_height() - (vy + vh) as f32,
                w: vw as f32,
                h: vh as f32,
            })
            .unwrap_or(Rect {
                x: 0.0,
                y: 0.0,
                w: fc.screen_width(),
                h: fc.screen_height(),
            });

        let mat = self.matrix(fc);
        let transform = mat.transform_point3(vec3(point.x, point.y, 0.));

        vec2(
            (transform.x / 2. + 0.5) * dims.w + dims.x,
            (0.5 - transform.y / 2.) * dims.h + dims.y,
        )
    }

    /// Returns the world space position for a 2d camera screen space position.
    ///
    /// Point is a screen space position, often mouse x and y.
    pub fn screen_to_world(&self, fc: &mut FrogCx, point: Vec2) -> Vec2 {
        let dims = self
            .viewport()
            .map(|(vx, vy, vw, vh)| Rect {
                x: vx as f32,
                y: fc.screen_height() - (vy + vh) as f32,
                w: vw as f32,
                h: vh as f32,
            })
            .unwrap_or(Rect {
                x: 0.0,
                y: 0.0,
                w: fc.screen_width(),
                h: fc.screen_height(),
            });

        let point = vec2(
            (point.x - dims.x) / dims.w * 2. - 1.,
            1. - (point.y - dims.y) / dims.h * 2.,
        );
        let inv_mat = self.matrix(fc).inverse();
        let transform = inv_mat.transform_point3(vec3(point.x, point.y, 0.));

        vec2(transform.x, transform.y)
    }
}

impl FrogCx {
    /// Set active camera.
    pub fn set_camera(&mut self, camera: &dyn Camera) {
        // flush previous camera draw calls
        self.perform_render_passes();

        self.gl
            .render_pass(camera.render_pass().map(|rt| rt.raw_miniquad_id()));

        self.gl.viewport(camera.viewport());
        self.gl.depth_test(camera.depth_enabled());
        self.camera_matrix = Some(camera.matrix(self));
    }

    /// Reset default camera mode.
    pub fn set_default_camera(&mut self) {
        // flush previous camera draw calls
        self.perform_render_passes();

        self.gl.render_pass(None);
        self.gl.viewport(None);
        self.gl.depth_test(false);
        self.camera_matrix = None;
    }
}

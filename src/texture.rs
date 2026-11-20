//! Loading and rendering textures. Also render textures, per-pixel image manipulations.

use crate::{
    Error, FrogCx, color::Color, file::load_file, math::Rect, quad_gl::FrogGl,
    text::atlas::SpriteKey,
};

pub use crate::quad_gl::FilterMode;
use crate::quad_gl::{DrawMode, Vertex};
use glam::{Vec2, vec2};
use miniquad::RenderingBackend;
use slotmap::{TextureIdSlotMap, TextureSlotId};
use std::sync::Arc;

mod slotmap;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TextureSlotGuarded(pub TextureSlotId);

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TextureHandle {
    // texture that belongs to frogquad and follows normal garbage collection rules
    Managed(Arc<TextureSlotGuarded>),
    ManagedWeak(TextureSlotId),
    // raw miniquad texture, there are no guarantees that this texture is not yet deleted
    Unmanaged(miniquad::TextureId),
}

pub(crate) enum TextureCmd {
    RemoveTexture(TextureHandle),
    RemoveRenderPass(miniquad::RenderPass),
}

pub(crate) type TextureCmdSend = std::sync::mpsc::Sender<TextureCmd>;
pub(crate) type TextureCmdRecv = std::sync::mpsc::Receiver<TextureCmd>;

/// Contains data for managing textures
pub struct FrogTextures {
    textures: TextureIdSlotMap,
    pub(crate) cmd_send: TextureCmdSend,
    cmd_recv: TextureCmdRecv,
}

impl FrogTextures {
    pub(crate) fn new() -> FrogTextures {
        let (send, recv) = std::sync::mpsc::channel();
        FrogTextures {
            textures: TextureIdSlotMap::new(),
            cmd_send: send,
            cmd_recv: recv,
        }
    }
    fn store_texture(&mut self, texture: miniquad::TextureId) -> TextureHandle {
        TextureHandle::Managed(Arc::new(TextureSlotGuarded(self.textures.insert(texture))))
    }
    pub(crate) fn texture(&self, texture: TextureSlotId) -> Option<miniquad::TextureId> {
        self.textures.get(texture)
    }

    pub(crate) fn garbage_collect(&mut self, ctx: &mut miniquad::Context) {
        while let Ok(cmd) = self.cmd_recv.try_recv() {
            match cmd {
                TextureCmd::RemoveTexture(handle) => match handle {
                    TextureHandle::Managed(texture_slot_guarded) => {
                        if let Some(texture) = self.textures.get(texture_slot_guarded.0) {
                            ctx.delete_texture(texture);
                        }
                        self.textures.remove(texture_slot_guarded.0);
                    }
                    TextureHandle::ManagedWeak(_texture_slot_id) => todo!(),
                    TextureHandle::Unmanaged(texture_id) => {
                        ctx.delete_texture(texture_id);
                    }
                },
                TextureCmd::RemoveRenderPass(render_pass) => ctx.delete_render_pass(render_pass),
            }
        }
    }
}

/// Image, data stored in CPU memory
#[derive(Clone)]
pub struct Image {
    pub(crate) bytes: Vec<u8>,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("bytes.len()", &self.bytes.len())
            .finish()
    }
}

impl Image {
    /// Creates an empty Image.
    ///
    /// ```
    /// let image = frogquad::texture::Image::empty();
    /// ```
    pub const fn empty() -> Image {
        Image {
            width: 0,
            height: 0,
            bytes: vec![],
        }
    }

    /// Creates an Image from a slice of bytes that contains an encoded image.
    ///
    /// If `format` is None, it will make an educated guess on the
    /// [ImageFormat][image::ImageFormat].
    ///
    /// # Example
    ///
    /// ```
    /// let icon = frogquad::texture::Image::from_file_with_format(
    ///     include_bytes!("../examples/rust.png"),
    ///     Some(image::ImageFormat::Png),
    ///     );
    /// ```
    pub fn from_file_with_format(
        bytes: &[u8],
        format: Option<image::ImageFormat>,
    ) -> Result<Image, Error> {
        let img = if let Some(fmt) = format {
            image::load_from_memory_with_format(bytes, fmt)?.to_rgba8()
        } else {
            image::load_from_memory(bytes)?.to_rgba8()
        };
        let width = img.width() as u16;
        let height = img.height() as u16;
        let bytes = img.into_raw();

        Ok(Image {
            width,
            height,
            bytes,
        })
    }

    /// Creates an Image filled with the provided [Color].
    pub fn gen_image_color(width: u16, height: u16, color: Color) -> Image {
        let mut bytes = vec![0; width as usize * height as usize * 4];
        for i in 0..width as usize * height as usize {
            bytes[i * 4] = (color.r * 255.) as u8;
            bytes[i * 4 + 1] = (color.g * 255.) as u8;
            bytes[i * 4 + 2] = (color.b * 255.) as u8;
            bytes[i * 4 + 3] = (color.a * 255.) as u8;
        }
        Image {
            width,
            height,
            bytes,
        }
    }

    /// Updates this image from a slice of [Color]s.
    pub fn update(&mut self, colors: &[Color]) {
        assert!(self.width as usize * self.height as usize == colors.len());

        for i in 0..colors.len() {
            self.bytes[i * 4] = (colors[i].r * 255.) as u8;
            self.bytes[i * 4 + 1] = (colors[i].g * 255.) as u8;
            self.bytes[i * 4 + 2] = (colors[i].b * 255.) as u8;
            self.bytes[i * 4 + 3] = (colors[i].a * 255.) as u8;
        }
    }

    /// Returns the width of this image.
    pub const fn width(&self) -> usize {
        self.width as usize
    }

    /// Returns the height of this image.
    pub const fn height(&self) -> usize {
        self.height as usize
    }

    /// Returns this image's data as a slice of 4-byte arrays.
    pub fn get_image_data(&self) -> &[[u8; 4]] {
        use std::slice;
        assert!(self.width as usize * self.height as usize * 4 == self.bytes.len());

        unsafe {
            slice::from_raw_parts(
                self.bytes.as_ptr() as *const [u8; 4],
                self.width as usize * self.height as usize,
            )
        }
    }

    /// Returns this image's data as a mutable slice of 4-byte arrays.
    pub fn get_image_data_mut(&mut self) -> &mut [[u8; 4]] {
        use std::slice;
        assert!(self.width as usize * self.height as usize * 4 == self.bytes.len());

        unsafe {
            slice::from_raw_parts_mut(
                self.bytes.as_mut_ptr() as *mut [u8; 4],
                self.width as usize * self.height as usize,
            )
        }
    }

    /// Modifies a pixel [Color] in this image.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        assert!(x < self.width as u32);
        assert!(y < self.height as u32);

        let width = self.width;

        self.get_image_data_mut()[(y * width as u32 + x) as usize] = color.into();
    }

    /// Returns a pixel [Color] from this image.
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.get_image_data()[(y * self.width as u32 + x) as usize].into()
    }

    /// Returns an Image from a rect inside this image.
    pub fn sub_image(&self, rect: Rect) -> Image {
        let width = rect.w as usize;
        let height = rect.h as usize;
        let mut bytes = vec![0; width * height * 4];

        let x = rect.x as usize;
        let y = rect.y as usize;
        let mut n = 0;
        for y in y..y + height {
            for x in x..x + width {
                bytes[n] = self.bytes[y * self.width as usize * 4 + x * 4];
                bytes[n + 1] = self.bytes[y * self.width as usize * 4 + x * 4 + 1];
                bytes[n + 2] = self.bytes[y * self.width as usize * 4 + x * 4 + 2];
                bytes[n + 3] = self.bytes[y * self.width as usize * 4 + x * 4 + 3];
                n += 4;
            }
        }
        Image {
            width: width as u16,
            height: height as u16,
            bytes,
        }
    }

    /// Blends this image with another image (of identical dimensions)
    /// Inspired by  OpenCV saturated blending
    pub fn blend(&mut self, other: &Image) {
        assert!(
            self.width as usize * self.height as usize
                == other.width as usize * other.height as usize
        );

        for i in 0..self.bytes.len() / 4 {
            let c1: Color = Color {
                r: self.bytes[i * 4] as f32 / 255.,
                g: self.bytes[i * 4 + 1] as f32 / 255.,
                b: self.bytes[i * 4 + 2] as f32 / 255.,
                a: self.bytes[i * 4 + 3] as f32 / 255.,
            };
            let c2: Color = Color {
                r: other.bytes[i * 4] as f32 / 255.,
                g: other.bytes[i * 4 + 1] as f32 / 255.,
                b: other.bytes[i * 4 + 2] as f32 / 255.,
                a: other.bytes[i * 4 + 3] as f32 / 255.,
            };
            let new_color: Color = Color {
                r: f32::min(c1.r * c1.a + c2.r * c2.a, 1.),
                g: f32::min(c1.g * c1.a + c2.g * c2.a, 1.),
                b: f32::min(c1.b * c1.a + c2.b * c2.a, 1.),
                a: f32::max(c1.a, c2.a) + (1. - f32::max(c1.a, c2.a)) * f32::min(c1.a, c2.a),
            };
            self.bytes[i * 4] = (new_color.r * 255.) as u8;
            self.bytes[i * 4 + 1] = (new_color.g * 255.) as u8;
            self.bytes[i * 4 + 2] = (new_color.b * 255.) as u8;
            self.bytes[i * 4 + 3] = (new_color.a * 255.) as u8;
        }
    }

    /// Overlays an image on top of this one.
    /// Slightly different from blending two images,
    /// overlaying a completely transparent image has no effect
    /// on the original image, though blending them would.
    pub fn overlay(&mut self, other: &Image) {
        assert!(
            self.width as usize * self.height as usize
                == other.width as usize * other.height as usize
        );

        for i in 0..self.bytes.len() / 4 {
            let c1: Color = Color {
                r: self.bytes[i * 4] as f32 / 255.,
                g: self.bytes[i * 4 + 1] as f32 / 255.,
                b: self.bytes[i * 4 + 2] as f32 / 255.,
                a: self.bytes[i * 4 + 3] as f32 / 255.,
            };
            let c2: Color = Color {
                r: other.bytes[i * 4] as f32 / 255.,
                g: other.bytes[i * 4 + 1] as f32 / 255.,
                b: other.bytes[i * 4 + 2] as f32 / 255.,
                a: other.bytes[i * 4 + 3] as f32 / 255.,
            };
            let new_color: Color = Color {
                r: f32::min(c1.r * (1. - c2.a) + c2.r * c2.a, 1.),
                g: f32::min(c1.g * (1. - c2.a) + c2.g * c2.a, 1.),
                b: f32::min(c1.b * (1. - c2.a) + c2.b * c2.a, 1.),
                a: f32::min(c1.a + c2.a, 1.),
            };

            self.bytes[i * 4] = (new_color.r * 255.) as u8;
            self.bytes[i * 4 + 1] = (new_color.g * 255.) as u8;
            self.bytes[i * 4 + 2] = (new_color.b * 255.) as u8;
            self.bytes[i * 4 + 3] = (new_color.a * 255.) as u8;
        }
    }

    /// Saves this image as a PNG file.
    /// This method is not supported on web and will panic.
    pub fn export_png(&self, path: &str) {
        let mut bytes = vec![0; self.width as usize * self.height as usize * 4];

        // flip the image before saving
        for y in 0..self.height as usize {
            for x in 0..self.width as usize * 4 {
                bytes[y * self.width as usize * 4 + x] =
                    self.bytes[(self.height as usize - y - 1) * self.width as usize * 4 + x];
            }
        }

        image::save_buffer(
            path,
            &bytes[..],
            self.width as _,
            self.height as _,
            image::ColorType::Rgba8,
        )
        .unwrap();
    }
}

impl FrogCx {
    /// Loads an [Image] from a file into CPU memory.
    pub async fn load_image(&mut self, path: &str) -> Result<Image, Error> {
        let bytes = load_file(self, path).await?;

        Image::from_file_with_format(&bytes, None)
    }

    /// Loads a [Texture2D] from a file into GPU memory.
    pub async fn load_texture(&mut self, path: &str) -> Result<Texture2D, Error> {
        let bytes = load_file(self, path).await?;

        Ok(Texture2D::from_file_with_format(self, &bytes[..], None))
    }
}

/// A render pass, mostly corresponding to [`miniquad::RenderPass`].
#[derive(Debug, Clone)]
pub struct RenderPass {
    pub(crate) render_pass: Arc<miniquad::RenderPass>,
    tex_cmd_send: TextureCmdSend,
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        // Safety: if strong_count < 2, this is the last strong ref.
        // No new strong references can be created after this point.
        if Arc::strong_count(&self.render_pass) < 2 {
            let _ = self
                .tex_cmd_send
                .send(TextureCmd::RemoveRenderPass(*self.render_pass));
        }
    }
}

/// Extended options for creating a render target
#[derive(Debug, Clone)]
pub struct RenderTargetParams {
    /// 1 means no multi sampling.
    /// Note that sample_count > 1 is not supported on GL2, GLES2 and WebGL1
    pub sample_count: i32,

    /// depth: true creates a depth render target attachment and allows
    /// such a render target being used for a depth-testing cameras
    pub depth: bool,
}
impl Default for RenderTargetParams {
    fn default() -> RenderTargetParams {
        RenderTargetParams {
            sample_count: 1,
            depth: false,
        }
    }
}

impl RenderPass {
    /// Returns the miniquad handle for this render pass.
    pub fn raw_miniquad_id(&self) -> miniquad::RenderPass {
        *self.render_pass
    }
}

/// A render target that can be rendered to
#[derive(Clone, Debug)]
pub struct RenderTarget {
    /// The texture of the render target
    pub texture: Texture2D,
    pub(crate) render_pass: RenderPass,
}

impl FrogCx {
    /// A shortcut to create a render target with sample_count: 1 and no depth buffer
    pub fn render_target(&mut self, width: u32, height: u32) -> RenderTarget {
        self.render_target_ex(width, height, RenderTargetParams::default())
    }

    /// A shortcut to create a render target with no depth buffer and `sample_count: 4`
    pub fn render_target_msaa(&mut self, width: u32, height: u32) -> RenderTarget {
        self.render_target_ex(
            width,
            height,
            RenderTargetParams {
                sample_count: 4,
                ..Default::default()
            },
        )
    }
    /// Create a render target with extended options
    pub fn render_target_ex(
        &mut self,
        width: u32,
        height: u32,
        params: RenderTargetParams,
    ) -> RenderTarget {
        let color_texture = self.backend.new_render_texture(miniquad::TextureParams {
            width,
            height,
            sample_count: params.sample_count,
            ..Default::default()
        });
        let depth_texture = if params.depth {
            Some(self.backend.new_render_texture(miniquad::TextureParams {
                width,
                height,
                format: miniquad::TextureFormat::Depth,
                sample_count: params.sample_count,
                ..Default::default()
            }))
        } else {
            None
        };

        let (render_pass, texture) = if params.sample_count > 1 {
            let color_resolve_texture = self.backend.new_render_texture(miniquad::TextureParams {
                width,
                height,
                ..Default::default()
            });
            (
                self.backend.new_render_pass_mrt(
                    &[color_texture],
                    Some(&[color_resolve_texture]),
                    depth_texture,
                ),
                color_resolve_texture,
            )
        } else {
            (
                self.backend
                    .new_render_pass_mrt(&[color_texture], None, depth_texture),
                color_texture,
            )
        };

        let texture = Texture2D {
            texture: self.textures.store_texture(texture),
        };

        let render_pass = RenderPass {
            render_pass: Arc::new(render_pass),
            tex_cmd_send: self.textures.cmd_send.clone(),
        };
        RenderTarget {
            texture,
            render_pass,
        }
    }
}

/// Extended options for drawing a texture
#[derive(Debug, Clone)]
pub struct DrawTextureParams {
    /// Size at the destination
    pub dest_size: Option<Vec2>,

    /// Part of texture to draw. If None - draw the whole texture.
    /// Good use example: drawing an image from texture atlas.
    /// Is None by default
    pub source: Option<Rect>,

    /// Rotation in radians
    pub rotation: f32,

    /// Mirror on the X axis
    pub flip_x: bool,

    /// Mirror on the Y axis
    pub flip_y: bool,

    /// Rotate around this point.
    /// When `None`, rotate around the texture's center.
    /// When `Some`, the coordinates are in screen-space.
    /// E.g. pivot (0,0) rotates around the top left corner of the screen, not of the
    /// texture.
    pub pivot: Option<Vec2>,
}

impl Default for DrawTextureParams {
    fn default() -> DrawTextureParams {
        DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: 0.,
            pivot: None,
            flip_x: false,
            flip_y: false,
        }
    }
}

impl FrogCx {
    /// Draw a texture at the specified position, with the specified color
    pub fn draw_texture(&mut self, texture: &Texture2D, x: f32, y: f32, color: Color) {
        self.draw_texture_ex(texture, x, y, color, Default::default());
    }
    /// Draw a texture with extended options
    pub fn draw_texture_ex(
        &mut self,
        texture: &Texture2D,
        x: f32,
        y: f32,
        color: Color,
        params: DrawTextureParams,
    ) {
        let [mut width, mut height] = texture.size(self).to_array();

        let Rect {
            x: mut sx,
            y: mut sy,
            w: mut sw,
            h: mut sh,
        } = params.source.unwrap_or(Rect {
            x: 0.,
            y: 0.,
            w: width,
            h: height,
        });

        let texture_opt = self
            .texture_batcher
            .get(&self.gl, &self.textures, &mut *self.backend, texture)
            .map(|(batched_texture, uv)| {
                let [batched_width, batched_height] = batched_texture.size(self).to_array();
                sx = ((sx / width) * uv.w + uv.x) * batched_width;
                sy = ((sy / height) * uv.h + uv.y) * batched_height;
                sw = (sw / width) * uv.w * batched_width;
                sh = (sh / height) * uv.h * batched_height;

                width = batched_width;
                height = batched_height;

                batched_texture
            });
        let texture = texture_opt.as_ref().unwrap_or(texture);

        let (mut w, mut h) = match params.dest_size {
            Some(dst) => (dst.x, dst.y),
            _ => (sw, sh),
        };
        let mut x = x;
        let mut y = y;
        if params.flip_x {
            x += w;
            w = -w;
        }
        if params.flip_y {
            y += h;
            h = -h;
        }

        let pivot = params.pivot.unwrap_or(vec2(x + w / 2., y + h / 2.));
        let m = pivot;
        let p = [
            vec2(x, y) - pivot,
            vec2(x + w, y) - pivot,
            vec2(x + w, y + h) - pivot,
            vec2(x, y + h) - pivot,
        ];
        let r = params.rotation;
        let p = [
            vec2(
                p[0].x * r.cos() - p[0].y * r.sin(),
                p[0].x * r.sin() + p[0].y * r.cos(),
            ) + m,
            vec2(
                p[1].x * r.cos() - p[1].y * r.sin(),
                p[1].x * r.sin() + p[1].y * r.cos(),
            ) + m,
            vec2(
                p[2].x * r.cos() - p[2].y * r.sin(),
                p[2].x * r.sin() + p[2].y * r.cos(),
            ) + m,
            vec2(
                p[3].x * r.cos() - p[3].y * r.sin(),
                p[3].x * r.sin() + p[3].y * r.cos(),
            ) + m,
        ];
        #[rustfmt::skip]
    let vertices = [
        Vertex::new(p[0].x, p[0].y, 0.,  sx      /width,  sy      /height, color),
        Vertex::new(p[1].x, p[1].y, 0., (sx + sw)/width,  sy      /height, color),
        Vertex::new(p[2].x, p[2].y, 0., (sx + sw)/width, (sy + sh)/height, color),
        Vertex::new(p[3].x, p[3].y, 0.,  sx      /width, (sy + sh)/height, color),
    ];
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        self.gl.texture(&self.textures, Some(texture));
        self.gl.set_draw_mode(DrawMode::Triangles);
        self.gl.geometry(&vertices, &indices);
    }

    /// Get pixel data from screen buffer and return an Image (screenshot)
    pub fn get_screen_data(&mut self) -> Image {
        self.perform_render_passes();

        let texture_id = self.backend.new_render_texture(miniquad::TextureParams {
            width: self.screen_width as _,
            height: self.screen_height as _,
            ..Default::default()
        });

        let texture = Texture2D {
            texture: self.textures.store_texture(texture_id),
        };

        texture.grab_screen(&mut *self.backend, &self.gl, &self.textures);

        texture.get_texture_data(&self.gl, &self.textures, &mut *self.backend)
    }
}

/// Texture, data stored in GPU memory
#[derive(Clone, Debug, PartialEq)]
pub struct Texture2D {
    pub(crate) texture: TextureHandle,
}

impl Texture2D {
    pub fn weak_clone(&self) -> Texture2D {
        match &self.texture {
            TextureHandle::Unmanaged(id) => Texture2D::unmanaged(*id),
            TextureHandle::Managed(t) => Texture2D {
                texture: TextureHandle::ManagedWeak(t.0),
            },
            TextureHandle::ManagedWeak(t) => Texture2D {
                texture: TextureHandle::ManagedWeak(*t),
            },
        }
    }
    pub(crate) const fn unmanaged(texture: miniquad::TextureId) -> Texture2D {
        Texture2D {
            texture: TextureHandle::Unmanaged(texture),
        }
    }
    /// Creates an empty Texture2D.
    ///
    /// # Example
    /// ```no_run
    ///
    /// # let fc: &mut frogquad::FrogCx = unimplemented!();
    /// let texture = frogquad::texture::Texture2D::empty(fc);
    /// ```
    pub fn empty(fc: &mut FrogCx) -> Texture2D {
        Texture2D::unmanaged(fc.gl.white_texture)
    }

    /// Creates a Texture2D from a slice of bytes that contains an encoded image.
    ///
    /// If `format` is None, it will make an educated guess on the
    /// [ImageFormat][image::ImageFormat].
    ///
    /// # Example
    /// ```no_run
    ///
    /// # let fc: &mut frogquad::FrogCx = unimplemented!();
    /// let texture = frogquad::texture::Texture2D::from_file_with_format(
    ///     fc,
    ///     include_bytes!("../examples/rust.png"),
    ///     None,
    ///     );
    /// ```
    pub fn from_file_with_format(
        fc: &mut FrogCx,
        bytes: &[u8],
        format: Option<image::ImageFormat>,
    ) -> Texture2D {
        let img = if let Some(fmt) = format {
            image::load_from_memory_with_format(bytes, fmt)
                .unwrap_or_else(|e| panic!("{}", e))
                .to_rgba8()
        } else {
            image::load_from_memory(bytes)
                .unwrap_or_else(|e| panic!("{}", e))
                .to_rgba8()
        };
        let width = img.width() as u16;
        let height = img.height() as u16;
        let bytes = img.into_raw();

        Self::from_rgba8(fc, width, height, &bytes)
    }

    /// Creates a Texture2D from an [Image].
    pub fn from_image(fc: &mut FrogCx, image: &Image) -> Texture2D {
        Texture2D::from_rgba8(fc, image.width, image.height, &image.bytes)
    }

    /// Creates a Texture2D from a miniquad
    /// [Texture](https://docs.rs/miniquad/0.3.0-alpha/miniquad/graphics/struct.Texture.html)
    pub const fn from_miniquad_texture(texture: miniquad::TextureId) -> Texture2D {
        Texture2D {
            texture: TextureHandle::Unmanaged(texture),
        }
    }

    /// Creates a Texture2D from a slice of bytes in an R,G,B,A sequence,
    /// with the given width and height.
    ///
    /// # Example
    ///
    /// ```no_run
    ///
    /// # let fc: &mut frogquad::FrogCx = unimplemented!();
    /// // Create a 2x2 texture from a byte slice with 4 rgba pixels
    /// let bytes: Vec<u8> = vec![255, 0, 0, 192, 0, 255, 0, 192, 0, 0, 255, 192, 255, 255, 255, 192];
    /// let texture = frogquad::texture::Texture2D::from_rgba8(fc, 2, 2, &bytes);
    /// ```
    pub fn from_rgba8(fc: &mut FrogCx, width: u16, height: u16, bytes: &[u8]) -> Texture2D {
        let texture = fc.backend.new_texture_from_rgba8(width, height, bytes);
        let texture = fc.textures.store_texture(texture);
        let texture = Texture2D { texture };
        texture.set_filter(fc, fc.default_filter_mode);

        fc.texture_batcher.add_unbatched(&texture);

        texture
    }

    /// Uploads [Image] data to this texture.
    pub fn update(&self, fc: &mut FrogCx, image: &Image) {
        let ctx = &mut fc.backend;
        let (width, height) = ctx.texture_size(FrogCx::raw_miniquad_id(&fc.gl, &fc.textures, self));

        assert_eq!(width, image.width as u32);
        assert_eq!(height, image.height as u32);

        ctx.texture_update(
            FrogCx::raw_miniquad_id(&fc.gl, &fc.textures, self),
            &image.bytes,
        );
    }

    // Updates the texture from an array of bytes.
    pub fn update_from_bytes(&self, fc: &mut FrogCx, width: u32, height: u32, bytes: &[u8]) {
        let ctx = &mut fc.backend;
        let (texture_width, texture_height) =
            ctx.texture_size(FrogCx::raw_miniquad_id(&fc.gl, &fc.textures, self));

        assert_eq!(texture_width, width);
        assert_eq!(texture_height, height);

        ctx.texture_update(FrogCx::raw_miniquad_id(&fc.gl, &fc.textures, self), bytes);
    }

    /// Uploads [Image] data to part of this texture.
    pub fn update_part(
        &self,
        fc: &mut FrogCx,
        image: &Image,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
    ) {
        let ctx = &mut fc.backend;

        ctx.texture_update_part(
            FrogCx::raw_miniquad_id(&fc.gl, &fc.textures, self),
            x_offset,
            y_offset,
            width,
            height,
            &image.bytes,
        );
    }

    /// Returns the width of this texture.
    pub fn width(&self, fc: &mut FrogCx) -> f32 {
        let ctx = &mut fc.backend;
        let (width, _) = ctx.texture_size(FrogCx::raw_miniquad_id(&fc.gl, &fc.textures, self));

        width as f32
    }

    /// Returns the height of this texture.
    pub fn height(&self, fc: &mut FrogCx) -> f32 {
        let ctx = &mut fc.backend;
        let (_, height) = ctx.texture_size(FrogCx::raw_miniquad_id(&fc.gl, &fc.textures, self));

        height as f32
    }

    pub fn size(&self, fc: &mut FrogCx) -> Vec2 {
        let ctx = &mut fc.backend;
        let (width, height) = ctx.texture_size(FrogCx::raw_miniquad_id(&fc.gl, &fc.textures, self));

        vec2(width as f32, height as f32)
    }

    /// Sets the [FilterMode] of this texture.
    ///
    /// Use Nearest if you need integer-ratio scaling for pixel art, for example.
    ///
    /// # Example
    /// ```no_run
    ///
    /// # let fc: &mut frogquad::FrogCx = unimplemented!();
    /// let texture = frogquad::texture::Texture2D::empty(fc);
    /// texture.set_filter(fc, miniquad::FilterMode::Linear);
    /// ```
    pub fn set_filter(&self, fc: &mut FrogCx, filter_mode: FilterMode) {
        let ctx = &mut fc.backend;

        ctx.texture_set_filter(
            FrogCx::raw_miniquad_id(&fc.gl, &fc.textures, self),
            filter_mode,
            miniquad::MipmapFilterMode::None,
        );
    }

    /// Updates this texture from the screen.
    #[allow(unreachable_patterns)]
    pub fn grab_screen(
        &self,
        rend: &mut dyn RenderingBackend,
        gl: &FrogGl,
        textures: &FrogTextures,
    ) {
        use miniquad::TextureFormat;
        let texture = FrogCx::raw_miniquad_id(gl, textures, self);
        let params = rend.texture_params(texture);
        let raw_id = match unsafe { rend.texture_raw_id(texture) } {
            miniquad::RawId::OpenGl(id) => id,
            _ => unimplemented!(),
        };
        let internal_format = match params.format {
            TextureFormat::RGB8 => miniquad::gl::GL_RGB,
            TextureFormat::RGBA8 => miniquad::gl::GL_RGBA,
            TextureFormat::RGBA16F => miniquad::gl::GL_RGBA,
            TextureFormat::Depth => miniquad::gl::GL_DEPTH_COMPONENT,
            TextureFormat::Depth32 => miniquad::gl::GL_DEPTH_COMPONENT,
            #[cfg(target_arch = "wasm32")]
            TextureFormat::Alpha => miniquad::gl::GL_ALPHA,
            #[cfg(not(target_arch = "wasm32"))]
            TextureFormat::Alpha => miniquad::gl::GL_R8,
        };
        unsafe {
            miniquad::gl::glBindTexture(miniquad::gl::GL_TEXTURE_2D, raw_id);
            miniquad::gl::glCopyTexImage2D(
                miniquad::gl::GL_TEXTURE_2D,
                0,
                internal_format,
                0,
                0,
                params.width as _,
                params.height as _,
                0,
            );
        }
    }

    /// Returns an [Image] from the pixel data in this texture.
    ///
    /// This operation can be expensive.
    pub fn get_texture_data(
        &self,
        gl: &FrogGl,
        textures: &FrogTextures,
        rend: &mut dyn RenderingBackend,
    ) -> Image {
        let (width, height) = rend.texture_size(FrogCx::raw_miniquad_id(gl, textures, self));
        let mut image = Image {
            width: width as _,
            height: height as _,
            bytes: vec![0; width as usize * height as usize * 4],
        };
        rend.texture_read_pixels(
            FrogCx::raw_miniquad_id(gl, textures, self),
            &mut image.bytes,
        );
        image
    }
}

pub(crate) struct Batcher {
    unbatched: Vec<Texture2D>,
    atlas: crate::text::atlas::Atlas,
}

impl Batcher {
    pub fn new(rend: &mut dyn miniquad::RenderingBackend, tex_ctx: &FrogTextures) -> Batcher {
        Batcher {
            unbatched: vec![],
            atlas: crate::text::atlas::Atlas::new(rend, tex_ctx, miniquad::FilterMode::Linear),
        }
    }

    pub fn add_unbatched(&mut self, texture: &Texture2D) {
        self.unbatched.push(texture.weak_clone());
    }

    pub fn get(
        &mut self,
        gl: &FrogGl,
        tex_ctx: &FrogTextures,
        rend: &mut dyn RenderingBackend,
        texture: &Texture2D,
    ) -> Option<(Texture2D, Rect)> {
        let id = SpriteKey::Texture(FrogCx::raw_miniquad_id(gl, tex_ctx, texture));
        let uv_rect = self.atlas.get_uv_rect(id, rend)?;
        Some((Texture2D::unmanaged(self.atlas.texture(rend)), uv_rect))
    }
}

impl FrogCx {
    /// Build an atlas out of all currently loaded texture
    /// Later on all draw_texture calls with texture available in the atlas will use
    /// the one from the atlas
    /// NOTE: the GPU memory and texture itself in Texture2D will still be allocated
    /// and Texture->Image conversions will work with Texture2D content, not the atlas
    pub fn build_textures_atlas(&mut self) {
        for texture in self.texture_batcher.unbatched.drain(0..) {
            let sprite: Image =
                texture.get_texture_data(&self.gl, &self.textures, &mut *self.backend);
            let id =
                SpriteKey::Texture(FrogCx::raw_miniquad_id(&self.gl, &self.textures, &texture));

            self.texture_batcher.atlas.cache_sprite(id, sprite);
        }
    }

    pub fn set_default_filter_mode(&mut self, filter: FilterMode) {
        self.default_filter_mode = filter;
        self.texture_batcher
            .atlas
            .set_filter(filter, &mut *self.backend);
    }
}

use {
    frogquad::{Camera2D, Color, FrogCx},
    glam::vec2,
};

struct RawMqApp {
    stage: raw_miniquad::Stage,
}

impl RawMqApp {
    fn new(fc: &mut FrogCx) -> Self {
        Self {
            stage: { raw_miniquad::Stage::new(&mut *fc.quad_context) },
        }
    }
}

impl frogquad::App for RawMqApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::LIGHTGRAY);

        // Render some primitives in camera space

        fc.set_camera(&Camera2D {
            zoom: vec2(1., fc.screen_width() / fc.screen_height()),
            ..Default::default()
        });
        fc.draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, Color::BLUE);
        fc.draw_rectangle(-0.3, 0.3, 0.2, 0.2, Color::GREEN);
        fc.draw_circle(0., 0., 0.1, Color::YELLOW);

        {
            // Ensure that frogquad's shapes are not going to be lost
            fc.perform_render_passes();

            let t = fc.time();
            fc.quad_context.apply_pipeline(&self.stage.pipeline);

            fc.quad_context
                .begin_default_pass(miniquad::PassAction::Nothing);
            fc.quad_context.apply_bindings(&self.stage.bindings);

            for i in 0..10 {
                let t = t + i as f64 * 0.3;

                fc.quad_context
                    .apply_uniforms(miniquad::UniformsSource::table(
                        &raw_miniquad::shader::Uniforms {
                            offset: (t.sin() as f32 * 0.5, (t * 3.).cos() as f32 * 0.5),
                        },
                    ));
                fc.quad_context.draw(0, 6, 1);
            }
            fc.quad_context.end_render_pass();
        }

        // Back to screen space, render some text

        fc.set_default_camera();
        fc.draw_text("HELLO", 30.0, 200.0, 30.0, Color::BLACK);
    }
}

fn main() {
    frogquad::start("Raw miniquad", RawMqApp::new);
}

mod raw_miniquad {
    use miniquad::*;

    #[repr(C)]
    struct Vec2 {
        x: f32,
        y: f32,
    }
    #[repr(C)]
    struct Vertex {
        pos: Vec2,
        uv: Vec2,
    }

    pub struct Stage {
        pub pipeline: Pipeline,
        pub bindings: Bindings,
    }

    impl Stage {
        pub fn new(ctx: &mut dyn RenderingBackend) -> Stage {
            #[rustfmt::skip]
            let vertices: [Vertex; 4] = [
                Vertex { pos : Vec2 { x: -0.5, y: -0.5 }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  0.5, y: -0.5 }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  0.5, y:  0.5 }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x: -0.5, y:  0.5 }, uv: Vec2 { x: 0., y: 1. } },
            ];
            let vertex_buffer = ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&vertices),
            );

            let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
            let index_buffer = ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&indices[..]),
            );

            let pixels: [u8; 4 * 4 * 4] = [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
                0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            ];
            let texture = ctx.new_texture_from_rgba8(4, 4, &pixels);

            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![texture],
            };

            let shader = ctx
                .new_shader(
                    miniquad::ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                    shader::meta(),
                )
                .unwrap();

            let pipeline = ctx.new_pipeline(
                &[BufferLayout::default()],
                &[
                    VertexAttribute::new("pos", VertexFormat::Float2),
                    VertexAttribute::new("uv", VertexFormat::Float2),
                ],
                shader,
                Default::default(),
            );

            Stage { pipeline, bindings }
        }
    }

    pub mod shader {
        use miniquad::*;

        pub const VERTEX: &str = r#"#version 100
attribute vec2 pos;
attribute vec2 uv;

uniform vec2 offset;

varying lowp vec2 texcoord;

void main() {
    gl_Position = vec4(pos + offset, 0, 1);
    texcoord = uv;
}"#;

        pub const FRAGMENT: &str = r#"#version 100
varying lowp vec2 texcoord;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, texcoord);
}"#;

        pub fn meta() -> ShaderMeta {
            ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
                },
            }
        }

        #[repr(C)]
        pub struct Uniforms {
            pub offset: (f32, f32),
        }
    }
}

use {
    frogquad::{FrogCx, prelude::*},
    pollster::FutureExt,
};

struct ScreenTexExample {
    lens_material: Material,
    texture: Texture2D,
}

impl ScreenTexExample {
    fn new(fc: &mut FrogCx) -> Self {
        let texture: Texture2D = load_texture(fc, "examples/chess.png").block_on().unwrap();

        let lens_material = load_material(
            fc,
            ShaderSource::Glsl {
                vertex: LENS_VERTEX_SHADER,
                fragment: LENS_FRAGMENT_SHADER,
            },
            MaterialParams {
                uniforms: vec![UniformDesc::new("Center", UniformType::Float2)],
                ..Default::default()
            },
        )
        .unwrap();
        Self {
            texture,
            lens_material,
        }
    }
}

impl frogquad::App for ScreenTexExample {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, WHITE);
        draw_texture_ex(
            fc,
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(fc), screen_height(fc))),
                ..Default::default()
            },
        );

        let lens_center = mouse_position(fc);

        self.lens_material
            .set_uniform(&mut fc.gl, "Center", lens_center);

        gl_use_material(&mut fc.gl, &self.lens_material);
        draw_circle(fc, lens_center.0, lens_center.1, 250.0, RED);
        gl_use_default_material(&mut fc.gl);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Texture"), |fc| {
        Box::new(ScreenTexExample::new(fc))
    });
}

const LENS_FRAGMENT_SHADER: &str = r#"#version 100
precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;
varying vec2 center;

uniform sampler2D _ScreenTexture;

void main() {
    float gradient = length(uv);
    vec2 uv_zoom = (uv_screen - center) * gradient + center;

    gl_FragColor = texture2D(_ScreenTexture, uv_zoom);
}
"#;

const LENS_VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 center;
varying lowp vec2 uv;
varying lowp vec2 uv_screen;

uniform mat4 Model;
uniform mat4 Projection;

uniform vec2 Center;

void main() {
    vec4 res = Projection * Model * vec4(position, 1);
    vec4 c = Projection * Model * vec4(Center, 0, 1);

    uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
    center = c.xy / 2.0 + vec2(0.5, 0.5);
    uv = texcoord;

    gl_Position = res;
}
";

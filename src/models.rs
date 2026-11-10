//! 3D shapes and models, loading 3d models from files, drawing 3D primitives.

use crate::FrogCx;
use crate::color::Color;

use crate::{quad_gl::DrawMode, texture::Texture2D};
use glam::{Quat, Vec2, Vec3, Vec4, vec2, vec3, vec4};

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub uv: Vec2,
    pub color: [u8; 4],
    /// Normal is not used by frogquad and is completely optional.
    /// Might be usefull for custom shaders.
    /// While normal is not used by frogquad, it is completely safe to use it
    /// to pass arbitary user data, hence Vec4.
    pub normal: Vec4,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32, color: Color) -> Vertex {
        Vertex {
            position: vec3(x, y, z),
            uv: vec2(u, v),
            color: color.into(),
            normal: vec4(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn new2(position: Vec3, uv: Vec2, color: Color) -> Vertex {
        Vertex {
            position,
            uv,
            color: color.into(),
            normal: vec4(0.0, 0.0, 0.0, 0.0),
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub texture: Option<Texture2D>,
}

pub fn draw_mesh(fc: &mut FrogCx, mesh: &Mesh) {
    fc.gl.texture(&fc.textures, mesh.texture.as_ref());
    fc.gl.draw_mode(DrawMode::Triangles);
    fc.gl.geometry(&mesh.vertices[..], &mesh.indices[..]);
}

fn draw_quad(fc: &mut FrogCx, vertices: [Vertex; 4]) {
    let indices = [0, 1, 2, 0, 2, 3];
    fc.gl.draw_mode(DrawMode::Triangles);
    fc.gl.geometry(&vertices, &indices);
}

pub fn draw_line_3d(fc: &mut FrogCx, start: Vec3, end: Vec3, color: Color) {
    let uv = vec2(0., 0.);
    let indices = [0, 1];

    let line = [Vertex::new2(start, uv, color), Vertex::new2(end, uv, color)];

    fc.gl.texture(&fc.textures, None);
    fc.gl.draw_mode(DrawMode::Lines);
    fc.gl.geometry(&line, &indices);
}

/// Draw a grid centered at (0, 0, 0)
pub fn draw_grid(
    fc: &mut FrogCx,
    slices: u32,
    spacing: f32,
    axes_color: Color,
    other_color: Color,
) {
    draw_grid_ex(
        fc,
        slices,
        spacing,
        axes_color,
        other_color,
        vec3(0., 0., 0.),
        Quat::IDENTITY,
    );
}

/// Draw a rotated grid centered at a specified point
pub fn draw_grid_ex(
    fc: &mut FrogCx,
    slices: u32,
    spacing: f32,
    axes_color: Color,
    other_color: Color,
    center: Vec3,
    rotation: Quat,
) {
    let half_slices = (slices as i32) / 2;
    for i in -half_slices..half_slices + 1 {
        let color = if i == 0 { axes_color } else { other_color };

        let start = vec3(i as f32 * spacing, 0., -half_slices as f32 * spacing);
        let end = vec3(i as f32 * spacing, 0., half_slices as f32 * spacing);

        draw_line_3d(
            fc,
            rotation.mul_vec3(start) + center,
            rotation.mul_vec3(end) + center,
            color,
        );

        let start = vec3(-half_slices as f32 * spacing, 0., i as f32 * spacing);
        let end = vec3(half_slices as f32 * spacing, 0., i as f32 * spacing);

        draw_line_3d(
            fc,
            rotation.mul_vec3(start) + center,
            rotation.mul_vec3(end) + center,
            color,
        );
    }
}

pub fn draw_plane(
    fc: &mut FrogCx,
    center: Vec3,
    size: Vec2,
    texture: Option<&Texture2D>,
    color: Color,
) {
    let v1 = Vertex::new2(center + vec3(-size.x, 0., -size.y), vec2(0., 0.), color);
    let v2 = Vertex::new2(center + vec3(-size.x, 0., size.y), vec2(0., 1.), color);
    let v3 = Vertex::new2(center + vec3(size.x, 0., size.y), vec2(1., 1.), color);
    let v4 = Vertex::new2(center + vec3(size.x, 0., -size.y), vec2(1., 0.), color);

    {
        fc.gl.texture(&fc.textures, texture);
    }
    draw_quad(fc, [v1, v2, v3, v4]);
}

/// Draw an affine (2D) parallelogram at given position, as two triangles.
///
/// The drawn parallelogram will have the vertices: `offset`, `offset + e1`, `offset + e2` and `offset + e1 + e2`
///
/// # Arguments
///
/// * `offset` - Offset of the first point from the origin
/// * `e1`, `e2` - Base vectors for the parallelogram
/// * `texture` - Optional [Texture2D] to apply, which will be streched on the entire shape (todo!
/// support custom uv values per vertex)
/// * `color` - The [Color] to draw the parallelogram
///
/// # Examples
///
/// Draw an axis aligned rectangle
/// ```no_run
/// # use frogquad::prelude::*;
/// let fc = frogquad::get_context();
/// draw_affine_parallelogram(fc, Vec3::ZERO, 3. * Vec3::X, 5. * Vec3::Z, None, RED);
/// ```
pub fn draw_affine_parallelogram(
    fc: &mut FrogCx,
    offset: Vec3,
    e1: Vec3,
    e2: Vec3,
    texture: Option<&Texture2D>,
    color: Color,
) {
    let v1 = Vertex::new2(offset, vec2(0., 0.), color);
    let v2 = Vertex::new2(offset + e1, vec2(0., 1.), color);
    let v3 = Vertex::new2(offset + e1 + e2, vec2(1., 1.), color);
    let v4 = Vertex::new2(offset + e2, vec2(1., 0.), color);

    {
        fc.gl.texture(&fc.textures, texture);
    }
    draw_quad(fc, [v1, v2, v3, v4]);
}

/// Draw an affine (3D) parallelepiped at given position, using six parallelograms.
///
/// The drawn parallelepiped will be built from the followwing parallelograms:
///
/// * `offset, offset + e1, offset + e2`
/// * `offset, offset + e2, offset + e3`
/// * `offset, offset + e1, offset + e3`
/// * `offset, offset + e1 + e2, offset + e1 + e3`
/// * `offset, offset + e2 + e1, offset + e2 + e3`
/// * `offset, offset + e3 + e1, offset + e3 + e2`
///
/// # Arguments
///
/// * `offset` - Offset of the first point from the origin
/// * `e1`, `e2`, `e3` - Base vectors for the parallelepiped
/// * `texture` - Optional [Texture2D] to apply, which will repeat on each face (todo!
/// support custom uv values per vertex, multiple textures?)
/// * `color` - The [Color] to draw the parallelepiped (todo! support color per face?)
///
/// # Examples
///
/// Draw an axis aligned cube
/// ```no_run
/// # use frogquad::prelude::*;
/// let fc = frogquad::get_context();
/// draw_affine_parallelepiped(fc, Vec3::ZERO, 3. * Vec3::X, 2. * Vec3::Y, 5. * Vec3::Z, None, RED);
/// ```
pub fn draw_affine_parallelepiped(
    fc: &mut FrogCx,
    offset: Vec3,
    e1: Vec3,
    e2: Vec3,
    e3: Vec3,
    texture: Option<&Texture2D>,
    color: Color,
) {
    draw_affine_parallelogram(fc, offset, e1, e2, texture, color);
    draw_affine_parallelogram(fc, offset, e1, e3, texture, color);
    draw_affine_parallelogram(fc, offset, e2, e3, texture, color);

    draw_affine_parallelogram(fc, offset + e1, e2, e3, texture, color);
    draw_affine_parallelogram(fc, offset + e2, e1, e3, texture, color);
    draw_affine_parallelogram(fc, offset + e3, e1, e2, texture, color);
}

pub fn draw_cube(
    fc: &mut FrogCx,
    position: Vec3,
    size: Vec3,
    texture: Option<&Texture2D>,
    color: Color,
) {
    fc.gl.texture(&fc.textures, texture);

    let (x, y, z) = (position.x, position.y, position.z);
    let (width, height, length) = (size.x, size.y, size.z);

    // Front face
    let bl_pos = vec3(x - width / 2., y - height / 2., z + length / 2.);
    let bl_uv = vec2(0., 0.);
    let br_pos = vec3(x + width / 2., y - height / 2., z + length / 2.);
    let br_uv = vec2(1., 0.);

    let tr_pos = vec3(x + width / 2., y + height / 2., z + length / 2.);
    let tr_uv = vec2(1., 1.);

    let tl_pos = vec3(x - width / 2., y + height / 2., z + length / 2.);
    let tl_uv = vec2(0., 1.);

    draw_quad(
        fc,
        [
            Vertex::new2(bl_pos, bl_uv, color),
            Vertex::new2(br_pos, br_uv, color),
            Vertex::new2(tr_pos, tr_uv, color),
            Vertex::new2(tl_pos, tl_uv, color),
        ],
    );

    // Back face
    let bl_pos = vec3(x - width / 2., y - height / 2., z - length / 2.);
    let bl_uv = vec2(0., 0.);
    let br_pos = vec3(x + width / 2., y - height / 2., z - length / 2.);
    let br_uv = vec2(1., 0.);

    let tr_pos = vec3(x + width / 2., y + height / 2., z - length / 2.);
    let tr_uv = vec2(1., 1.);

    let tl_pos = vec3(x - width / 2., y + height / 2., z - length / 2.);
    let tl_uv = vec2(0., 1.);

    draw_quad(
        fc,
        [
            Vertex::new2(bl_pos, bl_uv, color),
            Vertex::new2(br_pos, br_uv, color),
            Vertex::new2(tr_pos, tr_uv, color),
            Vertex::new2(tl_pos, tl_uv, color),
        ],
    );

    // Top face
    let bl_pos = vec3(x - width / 2., y + height / 2., z - length / 2.);
    let bl_uv = vec2(0., 1.);
    let br_pos = vec3(x - width / 2., y + height / 2., z + length / 2.);
    let br_uv = vec2(0., 0.);

    let tr_pos = vec3(x + width / 2., y + height / 2., z + length / 2.);
    let tr_uv = vec2(1., 0.);

    let tl_pos = vec3(x + width / 2., y + height / 2., z - length / 2.);
    let tl_uv = vec2(1., 1.);

    draw_quad(
        fc,
        [
            Vertex::new2(bl_pos, bl_uv, color),
            Vertex::new2(br_pos, br_uv, color),
            Vertex::new2(tr_pos, tr_uv, color),
            Vertex::new2(tl_pos, tl_uv, color),
        ],
    );

    // Bottom face
    let bl_pos = vec3(x - width / 2., y - height / 2., z - length / 2.);
    let bl_uv = vec2(0., 1.);
    let br_pos = vec3(x - width / 2., y - height / 2., z + length / 2.);
    let br_uv = vec2(0., 0.);

    let tr_pos = vec3(x + width / 2., y - height / 2., z + length / 2.);
    let tr_uv = vec2(1., 0.);

    let tl_pos = vec3(x + width / 2., y - height / 2., z - length / 2.);
    let tl_uv = vec2(1., 1.);

    draw_quad(
        fc,
        [
            Vertex::new2(bl_pos, bl_uv, color),
            Vertex::new2(br_pos, br_uv, color),
            Vertex::new2(tr_pos, tr_uv, color),
            Vertex::new2(tl_pos, tl_uv, color),
        ],
    );

    // Right face
    let bl_pos = vec3(x + width / 2., y - height / 2., z - length / 2.);
    let bl_uv = vec2(0., 1.);
    let br_pos = vec3(x + width / 2., y + height / 2., z - length / 2.);
    let br_uv = vec2(0., 0.);

    let tr_pos = vec3(x + width / 2., y + height / 2., z + length / 2.);
    let tr_uv = vec2(1., 0.);

    let tl_pos = vec3(x + width / 2., y - height / 2., z + length / 2.);
    let tl_uv = vec2(1., 1.);

    draw_quad(
        fc,
        [
            Vertex::new2(bl_pos, bl_uv, color),
            Vertex::new2(br_pos, br_uv, color),
            Vertex::new2(tr_pos, tr_uv, color),
            Vertex::new2(tl_pos, tl_uv, color),
        ],
    );

    // Left face
    let bl_pos = vec3(x - width / 2., y - height / 2., z - length / 2.);
    let bl_uv = vec2(0., 1.);
    let br_pos = vec3(x - width / 2., y + height / 2., z - length / 2.);
    let br_uv = vec2(0., 0.);

    let tr_pos = vec3(x - width / 2., y + height / 2., z + length / 2.);
    let tr_uv = vec2(1., 0.);

    let tl_pos = vec3(x - width / 2., y - height / 2., z + length / 2.);
    let tl_uv = vec2(1., 1.);

    draw_quad(
        fc,
        [
            Vertex::new2(bl_pos, bl_uv, color),
            Vertex::new2(br_pos, br_uv, color),
            Vertex::new2(tr_pos, tr_uv, color),
            Vertex::new2(tl_pos, tl_uv, color),
        ],
    );
}

pub fn draw_cube_wires(fc: &mut FrogCx, position: Vec3, size: Vec3, color: Color) {
    let (x, y, z) = (position.x, position.y, position.z);
    let (width, height, length) = (size.x, size.y, size.z);

    // Front Face

    // Bottom Line
    draw_line_3d(
        fc,
        vec3(x - width / 2., y - height / 2., z + length / 2.),
        vec3(x + width / 2., y - height / 2., z + length / 2.),
        color,
    );

    // Left Line
    draw_line_3d(
        fc,
        vec3(x + width / 2., y - height / 2., z + length / 2.),
        vec3(x + width / 2., y + height / 2., z + length / 2.),
        color,
    );

    // Top Line
    draw_line_3d(
        fc,
        vec3(x + width / 2., y + height / 2., z + length / 2.),
        vec3(x - width / 2., y + height / 2., z + length / 2.),
        color,
    );

    // Right Line
    draw_line_3d(
        fc,
        vec3(x - width / 2., y + height / 2., z + length / 2.),
        vec3(x - width / 2., y - height / 2., z + length / 2.),
        color,
    );

    // Back Face
    // Bottom Line
    draw_line_3d(
        fc,
        vec3(x - width / 2., y - height / 2., z - length / 2.),
        vec3(x + width / 2., y - height / 2., z - length / 2.),
        color,
    );

    // Left Line
    draw_line_3d(
        fc,
        vec3(x + width / 2., y - height / 2., z - length / 2.),
        vec3(x + width / 2., y + height / 2., z - length / 2.),
        color,
    );

    // Top Line
    draw_line_3d(
        fc,
        vec3(x + width / 2., y + height / 2., z - length / 2.),
        vec3(x - width / 2., y + height / 2., z - length / 2.),
        color,
    );

    // Right Line
    draw_line_3d(
        fc,
        vec3(x - width / 2., y + height / 2., z - length / 2.),
        vec3(x - width / 2., y - height / 2., z - length / 2.),
        color,
    );

    // Top Face
    // Left Line
    draw_line_3d(
        fc,
        vec3(x - width / 2., y + height / 2., z + length / 2.),
        vec3(x - width / 2., y + height / 2., z - length / 2.),
        color,
    );

    // Right Line
    draw_line_3d(
        fc,
        vec3(x + width / 2., y + height / 2., z + length / 2.),
        vec3(x + width / 2., y + height / 2., z - length / 2.),
        color,
    );

    // Bottom Face
    // Left Line
    draw_line_3d(
        fc,
        vec3(x - width / 2., y - height / 2., z + length / 2.),
        vec3(x - width / 2., y - height / 2., z - length / 2.),
        color,
    );

    // Right Line
    draw_line_3d(
        fc,
        vec3(x + width / 2., y - height / 2., z + length / 2.),
        vec3(x + width / 2., y - height / 2., z - length / 2.),
        color,
    );
}

#[derive(Debug, Clone)]
pub struct DrawSphereParams {
    pub rings: usize,
    pub slices: usize,
    pub draw_mode: DrawMode,
}

impl Default for DrawSphereParams {
    fn default() -> DrawSphereParams {
        DrawSphereParams {
            rings: 16,
            slices: 16,
            draw_mode: DrawMode::Triangles,
        }
    }
}

pub fn draw_sphere(
    fc: &mut FrogCx,
    center: Vec3,
    radius: f32,
    texture: Option<&Texture2D>,
    color: Color,
) {
    draw_sphere_ex(fc, center, radius, texture, color, Default::default());
}

pub fn draw_sphere_wires(
    fc: &mut FrogCx,
    center: Vec3,
    radius: f32,
    texture: Option<&Texture2D>,
    color: Color,
) {
    let params = DrawSphereParams {
        draw_mode: DrawMode::Lines,
        ..Default::default()
    };
    draw_sphere_ex(fc, center, radius, texture, color, params);
}

pub fn draw_sphere_ex(
    fc: &mut FrogCx,
    center: Vec3,
    radius: f32,
    texture: Option<&Texture2D>,
    color: Color,
    params: DrawSphereParams,
) {
    let rings = params.rings;
    let slices = params.slices;

    let scale = vec3(radius, radius, radius);

    fc.gl.texture(&fc.textures, texture);
    fc.gl.draw_mode(params.draw_mode);

    for i in 0..rings + 1 {
        for j in 0..slices {
            use std::f32::consts::PI;

            let pi34 = PI / 2. * 3.;
            let pi2 = PI * 2.;
            let i = i as f32;
            let j = j as f32;
            let rings: f32 = rings as _;
            let slices: f32 = slices as _;

            let v1 = vec3(
                (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * i).sin(),
                (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).cos(),
            );
            let uv1 = vec2(i / rings, j / slices);
            let v2 = vec3(
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).cos(),
            );
            let uv2 = vec2((i + 1.) / rings, (j + 1.) / slices);
            let v3 = vec3(
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * (j * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * (j * pi2 / slices).cos(),
            );
            let uv3 = vec2((i + 1.) / rings, j / slices);

            fc.gl.geometry(
                &[
                    Vertex::new2(v1 * scale + center, uv1, color),
                    Vertex::new2(v2 * scale + center, uv2, color),
                    Vertex::new2(v3 * scale + center, uv3, color),
                ],
                &[0, 1, 2],
            );

            let v1 = vec3(
                (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * i).sin(),
                (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).cos(),
            );
            let uv1 = vec2(i / rings, j / slices);
            let v2 = vec3(
                (pi34 + (PI / (rings + 1.)) * (i)).cos() * ((j + 1.) * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * (i)).sin(),
                (pi34 + (PI / (rings + 1.)) * (i)).cos() * ((j + 1.) * pi2 / slices).cos(),
            );
            let uv2 = vec2(i / rings, (j + 1.) / slices);
            let v3 = vec3(
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).cos(),
            );
            let uv3 = vec2((i + 1.) / rings, (j + 1.) / slices);

            fc.gl.geometry(
                &[
                    Vertex::new2(v1 * scale + center, uv1, color),
                    Vertex::new2(v2 * scale + center, uv2, color),
                    Vertex::new2(v3 * scale + center, uv3, color),
                ],
                &[0, 1, 2],
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawCylinderParams {
    pub sides: usize,
    pub draw_mode: DrawMode,
}

impl Default for DrawCylinderParams {
    fn default() -> DrawCylinderParams {
        DrawCylinderParams {
            sides: 16,
            draw_mode: DrawMode::Triangles,
        }
    }
}

pub fn draw_cylinder(
    fc: &mut FrogCx,
    position: Vec3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    texture: Option<&Texture2D>,
    color: Color,
) {
    draw_cylinder_ex(
        fc,
        position,
        radius_top,
        radius_bottom,
        height,
        texture,
        color,
        Default::default(),
    );
}

pub fn draw_cylinder_wires(
    fc: &mut FrogCx,
    position: Vec3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    texture: Option<&Texture2D>,
    color: Color,
) {
    let params = DrawCylinderParams {
        draw_mode: DrawMode::Lines,
        ..Default::default()
    };
    draw_cylinder_ex(
        fc,
        position,
        radius_top,
        radius_bottom,
        height,
        texture,
        color,
        params,
    );
}

//Note: can also be used to draw a cone by setting radius_top or radius_bottom to 0
pub fn draw_cylinder_ex(
    fc: &mut FrogCx,
    position: Vec3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    texture: Option<&Texture2D>,
    color: Color,
    params: DrawCylinderParams,
) {
    let sides = params.sides;

    fc.gl.texture(&fc.textures, texture);
    fc.gl.draw_mode(params.draw_mode);

    use std::f32::consts::PI;
    let angle_step = PI * 2.0 / sides as f32;
    //draw body
    for i in 0..sides + 1 {
        let i = i as f32;
        //bottom left
        let v1 = vec3(
            (i * angle_step).sin() * radius_bottom,
            0.0,
            (i * angle_step).cos() * radius_bottom,
        );
        //bottom right
        let v2 = vec3(
            ((i + 1.0) * angle_step).sin() * radius_bottom,
            0.0,
            ((i + 1.0) * angle_step).cos() * radius_bottom,
        );
        //top right
        let v3 = vec3(
            ((i + 1.0) * angle_step).sin() * radius_top,
            height,
            ((i + 1.0) * angle_step).cos() * radius_top,
        );

        fc.gl.geometry(
            &[
                Vertex::new2(v1 + position, vec2(0.0, 0.0), color),
                Vertex::new2(v2 + position, vec2(1.0, 0.0), color),
                Vertex::new2(v3 + position, vec2(1.0, 1.0), color),
            ],
            &[0, 1, 2],
        );

        //top left
        let v1 = vec3(
            (i * angle_step).sin() * radius_top,
            height,
            (i * angle_step).cos() * radius_top,
        );
        //bottom left
        let v2 = vec3(
            (i * angle_step).sin() * radius_bottom,
            0.0,
            (i * angle_step).cos() * radius_bottom,
        );
        //top right
        let v3 = vec3(
            ((i + 1.0) * angle_step).sin() * radius_top,
            height,
            ((i + 1.0) * angle_step).cos() * radius_top,
        );

        fc.gl.geometry(
            &[
                Vertex::new2(v1 + position, vec2(0.0, 0.0), color),
                Vertex::new2(v2 + position, vec2(1.0, 0.0), color),
                Vertex::new2(v3 + position, vec2(1.0, 1.0), color),
            ],
            &[0, 1, 2],
        );
    }

    //draw cap
    for i in 0..sides + 1 {
        let i = i as f32;
        let v1 = vec3(0.0, height, 0.0);
        let v2 = vec3(
            (i * angle_step).sin() * radius_top,
            height,
            (i * angle_step).cos() * radius_top,
        );
        let v3 = vec3(
            ((i + 1.0) * angle_step).sin() * radius_top,
            height,
            ((i + 1.0) * angle_step).cos() * radius_top,
        );

        fc.gl.geometry(
            &[
                Vertex::new2(v1 + position, vec2(0.0, 0.0), color),
                Vertex::new2(v2 + position, vec2(1.0, 0.0), color),
                Vertex::new2(v3 + position, vec2(1.0, 1.0), color),
            ],
            &[0, 1, 2],
        );
    }

    //draw base
    for i in 0..sides + 1 {
        let i = i as f32;
        let v1 = vec3(0.0, 0.0, 0.0);
        let v2 = vec3(
            (i * angle_step).sin() * radius_bottom,
            0.0,
            (i * angle_step).cos() * radius_bottom,
        );
        let v3 = vec3(
            ((i + 1.0) * angle_step).sin() * radius_bottom,
            0.0,
            ((i + 1.0) * angle_step).cos() * radius_bottom,
        );

        fc.gl.geometry(
            &[
                Vertex::new2(v1 + position, vec2(0.0, 0.0), color),
                Vertex::new2(v2 + position, vec2(1.0, 0.0), color),
                Vertex::new2(v3 + position, vec2(1.0, 1.0), color),
            ],
            &[0, 1, 2],
        );
    }
}

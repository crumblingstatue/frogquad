use {
    frogquad::{
        Color, FrogCx,
        camera::{Camera3D, set_camera, set_default_camera},
        input::show_mouse,
        models::{draw_cube_wires, draw_grid, draw_line_3d},
        time::get_frame_time,
    },
    glam::{Vec2, Vec3, vec3},
    miniquad::KeyCode,
};
// use glam::vec3;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

fn conf() -> miniquad::conf::Conf {
    miniquad::conf::Conf {
        window_title: String::from("frogquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

struct FirstPersonApp {
    x: f32,
    switch: bool,
    bounds: f32,
    pitch: f32,
    yaw: f32,
    front: Vec3,
    right: Vec3,
    up: Vec3,
    position: Vec3,
    last_mouse_position: Vec2,
    grabbed: bool,
    world_up: Vec3,
}

impl FirstPersonApp {
    fn new(fc: &mut FrogCx) -> Self {
        let x = 0.0;
        let switch = false;
        let bounds = 8.0;

        let world_up = vec3(0.0, 1.0, 0.0);
        let yaw: f32 = 1.18;
        let pitch: f32 = 0.0;

        let front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();
        let right = front.cross(world_up).normalize();
        let up = right.cross(front).normalize();

        let position = vec3(0.0, 1.0, 0.0);
        let last_mouse_position: Vec2 = fc.mouse_position().into();

        let grabbed = true;
        fc.set_cursor_grab(grabbed);
        show_mouse(false);
        Self {
            x,
            switch,
            bounds,
            yaw,
            pitch,
            front,
            right,
            up,
            position,
            last_mouse_position,
            grabbed,
            world_up,
        }
    }
}

impl frogquad::App for FirstPersonApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        let delta = get_frame_time(fc);

        if fc.is_key_pressed(KeyCode::Escape) {
            fc.request_quit();
            return;
        }
        if fc.is_key_pressed(KeyCode::Tab) {
            self.grabbed = !self.grabbed;
            fc.set_cursor_grab(self.grabbed);
            show_mouse(!self.grabbed);
        }

        if fc.is_key_down(KeyCode::Up) {
            self.position += self.front * MOVE_SPEED;
        }
        if fc.is_key_down(KeyCode::Down) {
            self.position -= self.front * MOVE_SPEED;
        }
        if fc.is_key_down(KeyCode::Left) {
            self.position -= self.right * MOVE_SPEED;
        }
        if fc.is_key_down(KeyCode::Right) {
            self.position += self.right * MOVE_SPEED;
        }

        let mouse_position: Vec2 = fc.mouse_position().into();
        let mouse_delta = mouse_position - self.last_mouse_position;

        self.last_mouse_position = mouse_position;

        if self.grabbed {
            self.yaw += mouse_delta.x * delta * LOOK_SPEED;
            self.pitch += mouse_delta.y * delta * -LOOK_SPEED;

            self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
            self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };

            self.front = vec3(
                self.yaw.cos() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.sin() * self.pitch.cos(),
            )
            .normalize();

            self.right = self.front.cross(self.world_up).normalize();
            self.up = self.right.cross(self.front).normalize();

            self.x += if self.switch { 0.04 } else { -0.04 };
            if self.x >= self.bounds || self.x <= -self.bounds {
                self.switch = !self.switch;
            }
        }

        fc.clear_background(Color::LIGHTGRAY);

        // Going 3d!

        set_camera(
            fc,
            &Camera3D {
                position: self.position,
                up: self.up,
                target: self.position + self.front,
                ..Default::default()
            },
        );

        draw_grid(fc, 20, 1., Color::BLACK, Color::GRAY);

        draw_line_3d(
            fc,
            vec3(self.x, 0.0, self.x),
            vec3(5.0, 5.0, 5.0),
            Color::new(1.0, 1.0, 0.0, 1.0),
        );

        draw_cube_wires(fc, vec3(0., 1., -6.), vec3(2., 2., 2.), Color::GREEN);
        draw_cube_wires(fc, vec3(0., 1., 6.), vec3(2., 2., 2.), Color::BLUE);
        draw_cube_wires(fc, vec3(2., 1., 2.), vec3(2., 2., 2.), Color::RED);

        // Back to screen space, render some text

        set_default_camera(fc);
        fc.draw_text("First Person Camera", 10.0, 20.0, 30.0, Color::BLACK);

        fc.draw_text(
            format!("X: {} Y: {}", mouse_position.x, mouse_position.y).as_str(),
            10.0,
            48.0 + 18.0,
            30.0,
            Color::BLACK,
        );
        fc.draw_text(
            format!("Press <TAB> to toggle mouse grab: {}", self.grabbed).as_str(),
            10.0,
            48.0 + 42.0,
            30.0,
            Color::BLACK,
        );
    }
}

fn main() {
    frogquad::start(conf().into(), |fc| Box::new(FirstPersonApp::new(fc)));
}

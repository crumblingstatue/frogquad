use {
    frogquad::{
        Color, FrogCx,
        conf::Conf,
        rand,
        shapes::{draw_circle, draw_poly_lines, draw_triangle_lines},
        text::{draw_text, measure_text},
        time::get_time,
    },
    glam::Vec2,
    miniquad::KeyCode,
};

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    collided: bool,
}

struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
}

fn wrap_around(v: &Vec2, fc: &FrogCx) -> Vec2 {
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > fc.screen_width() {
        vr.x = 0.;
    }
    if vr.x < 0. {
        vr.x = fc.screen_width()
    }
    if vr.y > fc.screen_height() {
        vr.y = 0.;
    }
    if vr.y < 0. {
        vr.y = fc.screen_height()
    }
    vr
}

struct AsteroidsApp {
    ship: Ship,
    bullets: Vec<Bullet>,
    last_shot: f64,
    asteroids: Vec<Asteroid>,
    gameover: bool,
}

impl AsteroidsApp {
    fn new(fc: &mut FrogCx) -> Self {
        let ship = Ship {
            pos: Vec2::new(fc.screen_width() / 2., fc.screen_height() / 2.),
            rot: 0.,
            vel: Vec2::new(0., 0.),
        };

        let bullets = Vec::new();
        let last_shot = get_time(fc);
        let asteroids = Vec::new();
        let gameover = false;

        Self {
            ship,
            bullets,
            last_shot,
            asteroids,
            gameover,
        }
    }
}

impl frogquad::App for AsteroidsApp {
    fn update(&mut self, fc: &mut FrogCx) {
        if self.gameover {
            fc.clear_background(Color::LIGHTGRAY);
            let mut text = "You Win!. Press [enter] to play again.";
            let font_size = 30.;

            if !self.asteroids.is_empty() {
                text = "Game Over. Press [enter] to play again.";
            }
            let text_size = measure_text(text, fc.default_font(), font_size as _, 1.0);
            draw_text(
                fc,
                text,
                fc.screen_width() / 2. - text_size.width / 2.,
                fc.screen_height() / 2. - text_size.height / 2.,
                font_size,
                Color::DARKGRAY,
            );
            if fc.is_key_down(KeyCode::Enter) {
                self.ship = Ship {
                    pos: Vec2::new(fc.screen_width() / 2., fc.screen_height() / 2.),
                    rot: 0.,
                    vel: Vec2::new(0., 0.),
                };
                self.bullets = Vec::new();
                self.asteroids = Vec::new();
                self.gameover = false;
                let screen_center = Vec2::new(fc.screen_width() / 2., fc.screen_height() / 2.);
                for _ in 0..10 {
                    self.asteroids.push(Asteroid {
                        pos: screen_center
                            + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                                .normalize()
                                * fc.screen_width().min(fc.screen_height())
                                / 2.,
                        vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
                        rot: 0.,
                        rot_speed: rand::gen_range(-2., 2.),
                        size: fc.screen_width().min(fc.screen_height()) / 10.,
                        sides: rand::gen_range(3, 8),
                        collided: false,
                    })
                }
            }
            return;
        }
        let frame_t = get_time(fc);
        let rotation = self.ship.rot.to_radians();

        let mut acc = -self.ship.vel / 100.; // Friction

        // Forward
        if fc.is_key_down(KeyCode::Up) {
            acc = Vec2::new(rotation.sin(), -rotation.cos()) / 3.;
        }

        // Shot
        if fc.is_key_down(KeyCode::Space) && frame_t - self.last_shot > 0.5 {
            let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());
            self.bullets.push(Bullet {
                pos: self.ship.pos + rot_vec * SHIP_HEIGHT / 2.,
                vel: rot_vec * 7.,
                shot_at: frame_t,
                collided: false,
            });
            self.last_shot = frame_t;
        }

        // Steer
        if fc.is_key_down(KeyCode::Right) {
            self.ship.rot += 5.;
        } else if fc.is_key_down(KeyCode::Left) {
            self.ship.rot -= 5.;
        }

        // Euler integration
        self.ship.vel += acc;
        if self.ship.vel.length() > 5. {
            self.ship.vel = self.ship.vel.normalize() * 5.;
        }
        self.ship.pos += self.ship.vel;
        self.ship.pos = wrap_around(&self.ship.pos, fc);

        // Move each bullet
        for bullet in self.bullets.iter_mut() {
            bullet.pos += bullet.vel;
        }

        // Move each asteroid
        for asteroid in self.asteroids.iter_mut() {
            asteroid.pos += asteroid.vel;
            asteroid.pos = wrap_around(&asteroid.pos, fc);
            asteroid.rot += asteroid.rot_speed;
        }

        // Bullet lifetime
        self.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in self.asteroids.iter_mut() {
            // Asteroid/ship collision
            if (asteroid.pos - self.ship.pos).length() < asteroid.size + SHIP_HEIGHT / 3. {
                self.gameover = true;
                break;
            }

            // Asteroid/bullet collision
            for bullet in self.bullets.iter_mut() {
                if (asteroid.pos - bullet.pos).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;

                    // Break the asteroid
                    if asteroid.sides > 3 {
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(bullet.vel.y, -bullet.vel.x).normalize()
                                * rand::gen_range(1., 3.),
                            rot: rand::gen_range(0., 360.),
                            rot_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        });
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(-bullet.vel.y, bullet.vel.x).normalize()
                                * rand::gen_range(1., 3.),
                            rot: rand::gen_range(0., 360.),
                            rot_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        })
                    }
                    break;
                }
            }
        }

        // Remove the collided objects
        self.bullets
            .retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
        self.asteroids.retain(|asteroid| !asteroid.collided);
        self.asteroids.append(&mut new_asteroids);

        // You win?
        if self.asteroids.is_empty() {
            self.gameover = true;
        }

        if self.gameover {
            return;
        }

        fc.clear_background(Color::LIGHTGRAY);

        for bullet in self.bullets.iter() {
            draw_circle(fc, bullet.pos.x, bullet.pos.y, 2., Color::BLACK);
        }

        for asteroid in self.asteroids.iter() {
            draw_poly_lines(
                fc,
                asteroid.pos.x,
                asteroid.pos.y,
                asteroid.sides,
                asteroid.size,
                asteroid.rot,
                2.,
                Color::BLACK,
            )
        }

        let v1 = Vec2::new(
            self.ship.pos.x + rotation.sin() * SHIP_HEIGHT / 2.,
            self.ship.pos.y - rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.ship.pos.x - rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.ship.pos.y - rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.ship.pos.x + rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.ship.pos.y + rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        draw_triangle_lines(fc, v1, v2, v3, 2., Color::BLACK);
    }
}

fn main() {
    frogquad::start(Conf::from_title("Asteroids"), |fc| {
        Box::new(AsteroidsApp::new(fc))
    });
}

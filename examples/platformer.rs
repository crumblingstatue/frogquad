use {
    frogquad::{
        FrogCx,
        camera::{Camera2D, set_camera},
        color,
        conf::Conf,
        file::load_string,
        input::{is_key_down, is_key_pressed},
        math::Rect,
        texture::load_texture,
        time::get_frame_time,
        window::clear_background,
    },
    glam::{Vec2, vec2},
    miniquad::{FilterMode, KeyCode},
};

use frogquad_tiled as tiled;

use frogquad_platformer::*;
use pollster::FutureExt;

struct Player {
    collider: Actor,
    speed: Vec2,
}

struct Platform {
    collider: Solid,
    speed: f32,
}

struct PlatformerApp {
    camera: Camera2D,
    platform: Platform,
    player: Player,
    tiled_map: frogquad_tiled::Map,
    world: World,
}

impl PlatformerApp {
    fn new(fc: &mut FrogCx) -> Self {
        let tileset = load_texture(fc, "examples/tileset.png").block_on().unwrap();
        tileset.set_filter(fc, FilterMode::Nearest);

        let tiled_map_json = load_string(fc, "examples/map.json").block_on().unwrap();
        let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
            static_colliders.push(if tile.is_some() {
                Tile::Solid
            } else {
                Tile::Empty
            });
        }

        let mut world = World::default();
        world.add_static_tiled_layer(static_colliders, 8., 8., 40, 1);

        let player = Player {
            collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
            speed: vec2(0., 0.),
        };

        let platform = Platform {
            collider: world.add_solid(vec2(170.0, 130.0), 32, 8),
            speed: 50.,
        };

        let camera = Camera2D::from_display_rect(Rect::new(0.0, 152.0, 320.0, -152.0));
        Self {
            player,
            platform,
            camera,
            tiled_map,
            world,
        }
    }
}

impl frogquad::App for PlatformerApp {
    fn update(&mut self, fc: &mut FrogCx) {
        clear_background(fc, color::BLACK);

        set_camera(fc, &self.camera);

        self.tiled_map
            .draw_tiles(fc, "main layer", Rect::new(0.0, 0.0, 320.0, 152.0), None);

        // draw platform
        {
            let pos = self.world.solid_pos(self.platform.collider);
            self.tiled_map.spr_ex(
                fc,
                "tileset",
                Rect::new(6.0 * 8.0, 0.0, 32.0, 8.0),
                Rect::new(pos.x, pos.y, 32.0, 8.0),
            )
        }

        // draw player
        {
            // sprite id from tiled
            const PLAYER_SPRITE: u32 = 120;

            let pos = self.world.actor_pos(self.player.collider);
            if self.player.speed.x >= 0.0 {
                self.tiled_map.spr(
                    fc,
                    "tileset",
                    PLAYER_SPRITE,
                    Rect::new(pos.x, pos.y, 8.0, 8.0),
                );
            } else {
                self.tiled_map.spr(
                    fc,
                    "tileset",
                    PLAYER_SPRITE,
                    Rect::new(pos.x + 8.0, pos.y, -8.0, 8.0),
                );
            }
        }

        // player movement control
        {
            let pos = self.world.actor_pos(self.player.collider);
            let on_ground = self
                .world
                .collide_check(self.player.collider, pos + vec2(0., 1.));

            if !on_ground {
                self.player.speed.y += 500. * get_frame_time(fc);
            }

            if is_key_down(fc, KeyCode::Right) {
                self.player.speed.x = 100.0;
            } else if is_key_down(fc, KeyCode::Left) {
                self.player.speed.x = -100.0;
            } else {
                self.player.speed.x = 0.;
            }

            if is_key_pressed(fc, KeyCode::Space) && on_ground {
                self.player.speed.y = -120.;
            }

            self.world.move_h(
                self.player.collider,
                self.player.speed.x * get_frame_time(fc),
            );
            self.world.move_v(
                self.player.collider,
                self.player.speed.y * get_frame_time(fc),
            );
        }

        // platform movement
        {
            self.world.solid_move(
                self.platform.collider,
                self.platform.speed * get_frame_time(fc),
                0.0,
            );
            let pos = self.world.solid_pos(self.platform.collider);
            if self.platform.speed > 1. && pos.x >= 220. {
                self.platform.speed *= -1.;
            }
            if self.platform.speed < -1. && pos.x <= 150. {
                self.platform.speed *= -1.;
            }
        }
    }
}

fn main() {
    frogquad::start(Conf::from_title("Platformer"), |fc| {
        Box::new(PlatformerApp::new(fc))
    });
}

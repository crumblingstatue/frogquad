use {
    frogquad::{Color, FrogCx, rand, text::measure_text},
    miniquad::KeyCode,
};

use std::collections::LinkedList;

const SQUARES: i16 = 16;

type Point = (i16, i16);

struct SnakeApp {
    snake: Snake,
    fruit: Point,
    score: i32,
    speed: f64,
    last_update: f64,
    navigation_lock: bool,
    game_over: bool,
}

impl SnakeApp {
    fn new(fc: &mut FrogCx) -> Self {
        Self {
            snake: Snake {
                head: (0, 0),
                dir: (1, 0),
                body: LinkedList::new(),
            },
            fruit: (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES)),
            score: 0,
            speed: 0.3,
            last_update: fc.time(),
            navigation_lock: false,
            game_over: false,
        }
    }
}

impl frogquad::App for SnakeApp {
    fn update(&mut self, fc: &mut FrogCx) {
        let up = (0, -1);
        let down = (0, 1);
        let right = (1, 0);
        let left = (-1, 0);
        if !self.game_over {
            if fc.is_key_down(KeyCode::Right) && self.snake.dir != left && !self.navigation_lock {
                self.snake.dir = right;
                self.navigation_lock = true;
            } else if fc.is_key_down(KeyCode::Left)
                && self.snake.dir != right
                && !self.navigation_lock
            {
                self.snake.dir = left;
                self.navigation_lock = true;
            } else if fc.is_key_down(KeyCode::Up) && self.snake.dir != down && !self.navigation_lock
            {
                self.snake.dir = up;
                self.navigation_lock = true;
            } else if fc.is_key_down(KeyCode::Down) && self.snake.dir != up && !self.navigation_lock
            {
                self.snake.dir = down;
                self.navigation_lock = true;
            }

            if fc.time() - self.last_update > self.speed {
                self.last_update = fc.time();
                self.snake.body.push_front(self.snake.head);
                self.snake.head = (
                    self.snake.head.0 + self.snake.dir.0,
                    self.snake.head.1 + self.snake.dir.1,
                );
                if self.snake.head == self.fruit {
                    self.fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                    self.score += 100;
                    self.speed *= 0.9;
                } else {
                    self.snake.body.pop_back();
                }
                if self.snake.head.0 < 0
                    || self.snake.head.1 < 0
                    || self.snake.head.0 >= SQUARES
                    || self.snake.head.1 >= SQUARES
                {
                    self.game_over = true;
                }
                for (x, y) in &self.snake.body {
                    if *x == self.snake.head.0 && *y == self.snake.head.1 {
                        self.game_over = true;
                    }
                }
                self.navigation_lock = false;
            }
        }
        if !self.game_over {
            fc.clear_background(Color::LIGHTGRAY);

            let game_size = fc.screen_width().min(fc.screen_height());
            let offset_x = (fc.screen_width() - game_size) / 2. + 10.;
            let offset_y = (fc.screen_height() - game_size) / 2. + 10.;
            let sq_size = (fc.screen_height() - offset_y * 2.) / SQUARES as f32;

            fc.draw_rectangle(
                offset_x,
                offset_y,
                game_size - 20.,
                game_size - 20.,
                Color::WHITE,
            );

            for i in 1..SQUARES {
                fc.draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    fc.screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    Color::LIGHTGRAY,
                );
            }

            for i in 1..SQUARES {
                fc.draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    fc.screen_height() - offset_y,
                    2.,
                    Color::LIGHTGRAY,
                );
            }

            fc.draw_rectangle(
                offset_x + self.snake.head.0 as f32 * sq_size,
                offset_y + self.snake.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                Color::DARKGREEN,
            );

            for (x, y) in &self.snake.body {
                fc.draw_rectangle(
                    offset_x + *x as f32 * sq_size,
                    offset_y + *y as f32 * sq_size,
                    sq_size,
                    sq_size,
                    Color::LIME,
                );
            }

            fc.draw_rectangle(
                offset_x + self.fruit.0 as f32 * sq_size,
                offset_y + self.fruit.1 as f32 * sq_size,
                sq_size,
                sq_size,
                Color::GOLD,
            );

            fc.draw_text(
                format!("SCORE: {}", self.score).as_str(),
                10.,
                20.,
                20.,
                Color::DARKGRAY,
            );
        } else {
            fc.clear_background(Color::WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, fc.default_font(), font_size as _, 1.0);

            fc.draw_text(
                text,
                fc.screen_width() / 2. - text_size.width / 2.,
                fc.screen_height() / 2. + text_size.height / 2.,
                font_size,
                Color::DARKGRAY,
            );

            if fc.is_key_down(KeyCode::Enter) {
                self.snake = Snake {
                    head: (0, 0),
                    dir: (1, 0),
                    body: LinkedList::new(),
                };
                self.fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                self.score = 0;
                self.speed = 0.3;
                self.last_update = fc.time();
                self.game_over = false;
            }
        }
    }
}

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

fn main() {
    frogquad::start("Snake", |fc| Box::new(SnakeApp::new(fc)));
}

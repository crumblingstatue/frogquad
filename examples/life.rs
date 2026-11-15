use frogquad::{
    Color, FrogCx,
    texture::{Image, Texture2D},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

struct LifeApp {
    image: Image,
    cells: Vec<CellState>,
    buffer: Vec<CellState>,
    texture: Texture2D,
}

impl LifeApp {
    fn new(fc: &mut FrogCx) -> Self {
        let w = fc.screen_width() as usize;
        let h = fc.screen_height() as usize;

        let mut cells = vec![CellState::Dead; w * h];
        let buffer = vec![CellState::Dead; w * h];

        let image = Image::gen_image_color(w as u16, h as u16, Color::WHITE);

        for cell in cells.iter_mut() {
            if quad_rand::gen_range(0, 5) == 0 {
                *cell = CellState::Alive;
            }
        }
        let texture = Texture2D::from_image(fc, &image);
        Self {
            image,
            cells,
            buffer,
            texture,
        }
    }
}

impl frogquad::App for LifeApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        fc.clear_background(Color::WHITE);

        let w = self.image.width();
        let h = self.image.height();

        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let mut neighbors_count = 0;

                for j in -1i32..=1 {
                    for i in -1i32..=1 {
                        // out of bounds
                        if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
                            continue;
                        }
                        // cell itself
                        if i == 0 && j == 0 {
                            continue;
                        }

                        let neighbor = self.cells[(y + j) as usize * w + (x + i) as usize];
                        if neighbor == CellState::Alive {
                            neighbors_count += 1;
                        }
                    }
                }

                let current_cell = self.cells[y as usize * w + x as usize];
                self.buffer[y as usize * w + x as usize] = match (current_cell, neighbors_count) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (CellState::Alive, x) if x < 2 => CellState::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (CellState::Alive, x) if x > 3 => CellState::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (CellState::Dead, 3) => CellState::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
            }
        }

        for i in 0..self.buffer.len() {
            self.cells[i] = self.buffer[i];

            self.image.set_pixel(
                (i % w) as u32,
                (i / w) as u32,
                match self.buffer[i] {
                    CellState::Alive => Color::BLACK,
                    CellState::Dead => Color::WHITE,
                },
            );
        }

        self.texture.update(fc, &self.image);

        fc.draw_texture(&self.texture, 0., 0., Color::WHITE);
    }
}

fn main() {
    frogquad::start("Life", LifeApp::new);
}

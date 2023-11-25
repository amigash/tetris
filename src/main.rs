use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;
use tetromino::Tetromino;

mod tetromino;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[macroquad::main("Tetris")]
async fn main() {
    let mut game = Game::new();
    let mut frame: u32 = 0;
    loop {
        while get_time() < f64::from(frame + 1) / 3.0 {
            game.update_projection();
            game.handle_input();
            game.render();
            next_frame().await;
        }
        game.update();
        frame += 1;
    }
}
#[derive(Copy, Clone)]
struct Block {
    x: usize,
    y: usize,
    color: u32,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Game {
    blocks: Vec<Block>,
    dx: isize,
    dy: usize,
    projection_dy: usize,
    tetromino: Tetromino,
    projection: Tetromino,
    canvas: Canvas2D,
}

impl Game {
    fn block(&self, y: usize, x: usize) -> Block {
        Block {
            x: x.saturating_add_signed(self.dx),
            y: y + self.dy,
            color: self.tetromino.color,
        }
    }
    fn new() -> Self {
        let tetromino = Tetromino::random();
        Self {
            blocks: Vec::new(),
            dx: (WIDTH / 2 - 1) as isize,
            dy: 0,
            projection_dy: 0,
            tetromino,
            projection: tetromino,
            canvas: Canvas2D::new(800.0, 600.0),
        }
    }

    fn next_tetromino(&mut self) {
        let tetromino = Tetromino::random();
        self.tetromino = tetromino;
        self.projection = tetromino;
        self.dy = 0;
        self.dx = (WIDTH / 2 - 1) as isize;
    }

    fn update(&mut self) {
        if self.dy == self.projection_dy {
            self.place();
            self.next_tetromino();
        } else {
            self.dy += 1;
        }
    }

    fn update_projection(&mut self) {
        let mut shift = 0;
        while !self.is_vertical_collision() {
            self.dy += 1;
            shift += 1;
        }
        self.projection_dy = self.dy - 1;
        self.dy -= shift;
    }

    fn drop(&mut self) {
        self.dy = self.projection_dy;
        self.update();
    }

    fn is_row_full(&self, row: usize) -> bool {
        self.blocks.iter().filter(|&&block| block.y == row).count() == WIDTH
    }

    fn remove_rows(&mut self, rows: Vec<usize>) {
        self.blocks.retain(|block| !rows.contains(&block.y));

        for row in rows {
            for block in &mut self.blocks {
                if block.y < row {
                    block.y += 1;
                }
            }
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            self.dx -= 1;
            if self.is_horizontal_collision() {
                self.dx += 1;
            }
        }
        if is_key_pressed(KeyCode::Right) {
            self.dx += 1;
            if self.is_horizontal_collision() {
                self.dx -= 1;
            }
        }
        if is_key_pressed(KeyCode::Up) {
            self.tetromino.rotate_clockwise();
            if self.is_horizontal_collision() {
                self.tetromino.rotate_counterclockwise();
            } else {
                self.projection.rotate_clockwise();
                self.update_projection();
            }
        }
        if is_key_pressed(KeyCode::Down) {
            self.tetromino.rotate_counterclockwise();
            if self.is_horizontal_collision() {
                self.tetromino.rotate_clockwise();
            } else {
                self.projection.rotate_counterclockwise();
                self.update_projection();
            }
        }
        if is_key_pressed(KeyCode::Space) {
            self.drop();
        }
    }

    fn is_horizontal_collision(&self) -> bool {
        self.tetromino.blocks().iter().any(|&[y, x]| {
            x.wrapping_add_signed(self.dx) >= WIDTH || self.blocks.contains(&self.block(y, x))
        })
    }

    fn is_vertical_collision(&self) -> bool {
        self.tetromino
            .blocks()
            .iter()
            .any(|&[y, x]| self.blocks.contains(&self.block(y, x)) || y + self.dy >= HEIGHT)
    }

    fn place(&mut self) {
        let mut rows = Vec::new();
        for [y, x] in self.tetromino.blocks() {
            self.blocks.push(self.block(y, x));
            rows.push(y + self.dy);
        }
        rows.sort_unstable();
        rows.dedup();
        rows.retain(|&row| self.is_row_full(row));
        self.remove_rows(rows);
    }

    fn render(&self) {
        fn draw_block(block: Block, color: Color) {
            draw_rectangle(
                block.x as f32 * 20.0,
                block.y as f32 * 20.0,
                20.0,
                20.0,
                color,
            );
        }

        set_camera(&self.canvas.camera);
        clear_background(WHITE);

        for &[y, x] in &self.projection.blocks() {
            let mut color = Color::from_hex(self.projection.color);
            color.a = 0.1;
            let block = Block {
                x: x.saturating_add_signed(self.dx),
                y: y + self.projection_dy,
                color: self.projection.color,
            };
            draw_block(block, color);
        }

        for &block in &self.blocks {
            draw_block(block, Color::from_hex(block.color));
        }

        for &[y, x] in &self.tetromino.blocks() {
            draw_block(self.block(y, x), Color::from_hex(self.tetromino.color));
        }

        draw_rectangle_lines(
            0.0,
            0.0,
            WIDTH as f32 * 20.0,
            HEIGHT as f32 * 20.0,
            2.0,
            BLACK,
        );

        set_default_camera();
        clear_background(BLACK);
        self.canvas.draw();
    }
}

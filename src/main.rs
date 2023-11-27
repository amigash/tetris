use graphics::render;
use std::collections::BTreeSet;
use macroquad::{
    input::{get_last_key_pressed, KeyCode},
    time::get_time,
    window::{Conf, next_frame}
};
use macroquad_canvas::Canvas2D;
use tetromino::Tetromino;

mod graphics;
mod tetromino;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const BLOCK_SIZE: f32 = 50.0;
const SCREEN_WIDTH: f32 = WIDTH as f32 * BLOCK_SIZE;
const SCREEN_HEIGHT: f32 = HEIGHT as f32 * BLOCK_SIZE;
const LINES_TO_LEVEL: usize = 10;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        high_dpi: true,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    let mut canvas = Canvas2D::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    loop {
        while game.seconds_since_last_update() < updates_per_second(game.lines_cleared) {
            game.update_projection();
            game.handle_input();
            render(&mut canvas, &game);
            next_frame().await;
        }
        game.update();
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
    lines_cleared: usize,
    last_frame_time: f64
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
            lines_cleared: 0,
            last_frame_time: 0.0
        }
    }

    fn next_tetromino(&mut self) {
        let tetromino = Tetromino::random();
        self.tetromino = tetromino;
        self.projection = tetromino;
        self.dy = 0;
        self.dx = (WIDTH / 2 - 1) as isize;
        if self.is_vertical_collision() {
            *self = Self::new();
        }
    }

    fn update(&mut self) {
        if self.dy == self.projection_dy {
            self.place();
            self.next_tetromino();
        } else {
            self.dy += 1;
        }
        self.last_frame_time = get_time();
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

    fn remove_rows(&mut self, rows: BTreeSet<usize>) {
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
        let Some(key) = get_last_key_pressed() else {
            return;
        };
        match key {
            KeyCode::Left => {
                self.dx -= 1;
                if self.is_horizontal_collision() {
                    self.dx += 1;
                }
            }
            KeyCode::Right => {
                self.dx += 1;
                if self.is_horizontal_collision() {
                    self.dx -= 1;
                }
            }
            KeyCode::Up => {
                self.tetromino.rotate_clockwise();
                if self.is_horizontal_collision() {
                    self.tetromino.rotate_counterclockwise();
                } else {
                    self.projection.rotate_clockwise();
                    self.update_projection();
                }
            }
            KeyCode::Down => {
                self.tetromino.rotate_counterclockwise();
                if self.is_horizontal_collision() {
                    self.tetromino.rotate_clockwise();
                } else {
                    self.projection.rotate_counterclockwise();
                    self.update_projection();
                }
            }
            KeyCode::Space => self.drop(),
            _ => {}
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
        let mut rows = BTreeSet::new();
        for [y, x] in self.tetromino.blocks() {
            self.blocks.push(self.block(y, x));
            rows.insert(y + self.dy);
        }
        rows.retain(|&row| self.is_row_full(row));
        self.lines_cleared += rows.len();
        self.remove_rows(rows);
    }

    fn seconds_since_last_update(&self) -> f64 {
        get_time() - self.last_frame_time
    }
}

fn updates_per_second(lines_cleared: usize) -> f64 {
    let level = (lines_cleared / LINES_TO_LEVEL) as f64;
    (0.8 - (level * 0.007)).powf(level)
}

use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;
use std::collections::HashSet;
use tetromino::Tetromino;

mod tetromino;

#[macroquad::main("Tetris")]
async fn main() {
    let mut game = Game::new();
    let mut frame: u8 = 0;
    loop {
        while get_time() < f64::from(frame + 1) / 5.0 {
            game.handle_input();
            game.render();
            next_frame().await;
        }
        game.update();
        frame += 1;
    }
}

struct Game {
    blocks: HashSet<[usize; 2]>,
    dx: isize,
    dy: usize,
    tetromino: Tetromino,
    canvas: Canvas2D,
}

impl Game {

    fn block(&self, y: &usize, x: &usize) -> [usize; 2] {
        [y + self.dy, x.saturating_add_signed(self.dx)]
    }
    fn new() -> Self {
        Self {
            blocks: HashSet::new(),
            dx: 0,
            dy: 0,
            tetromino: Tetromino::random(),
            canvas: Canvas2D::new(800.0, 600.0),
        }
    }

    fn update(&mut self) -> bool {
        if self.is_vertical_collision() {
            self.place_tetromino();
            self.tetromino = Tetromino::random();
            self.dy = 0;
            self.dx = 0;
            false
        } else {
            self.dy += 1;
            true
        }
    }

    fn drop(&mut self) {
        while self.update() {}
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            self.dx -= 1;
            if self.is_horizontal_collision() {
                self.dx += 1
            }
        }
        if is_key_pressed(KeyCode::Right) {
            self.dx += 1;
            if self.is_horizontal_collision() {
                self.dx -= 1
            }
        }
        if is_key_pressed(KeyCode::Up) {
            self.tetromino.rotate_clockwise();
            if self.is_horizontal_collision() {
                self.tetromino.rotate_counterclockwise();
            }
        }
        if is_key_pressed(KeyCode::Down) {
            self.tetromino.rotate_counterclockwise();
            if self.is_horizontal_collision() {
                self.tetromino.rotate_clockwise();
            }
        }
        if is_key_pressed(KeyCode::Space) {
            self.drop();
        }
    }

    fn is_horizontal_collision(&self) -> bool {
        self.tetromino.blocks().iter().any(|&[y, x]| {
            x.wrapping_add_signed(self.dx) > 10 || self.blocks.contains(&self.block(&y, &x))
        })
    }

    fn is_vertical_collision(&self) -> bool {
        self.tetromino.blocks().iter().any(|&[y, x]| {
            self.blocks.contains(&self.block(&y, &x))
                || y + self.dy > 20
        })
    }

    fn place_tetromino(&mut self) {
        for [y, x] in self.tetromino.blocks() {
            self.blocks.insert(self.block(&y, &x));
        }
    }

    fn render(&self) {
        fn draw_block(x: usize, y: usize) {
            draw_rectangle(x as f32 * 20.0, y as f32 * 20.0, 20.0, 20.0, RED);
        }

        set_camera(&self.canvas.camera);
        clear_background(WHITE);

        for &[y, x] in &self.blocks {
            draw_block(x, y);
        }

        for &[y, x] in &self.tetromino.blocks() {
            draw_block(x.saturating_add_signed(self.dx), y + self.dy);
        }

        set_default_camera();
        clear_background(BLACK);
        self.canvas.draw();
    }
}

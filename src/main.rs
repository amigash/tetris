use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;
use std::collections::HashSet;
use tetromino::Tetromino;

mod tetromino;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[macroquad::main("Tetris")]
async fn main() {
    let mut game = Game::new();
    let mut frame: u8 = 0;
    loop {
        while get_time() < f64::from(frame + 1) / 5.0 {
            game.update_projection();
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
    projection_dy: usize,
    tetromino: Tetromino,
    projection: Tetromino,
    canvas: Canvas2D,
}

impl Game {

    fn block(&self, y: usize, x: usize) -> [usize; 2] {
        [y + self.dy, x.saturating_add_signed(self.dx)]
    }
    fn new() -> Self {
        let tetromino = Tetromino::random();
        Self {
            blocks: HashSet::new(),
            dx: 0,
            dy: 0,
            projection_dy: 0,
            tetromino,
            projection: tetromino,
            canvas: Canvas2D::new(800.0, 600.0),
        }
    }

    fn update(&mut self) {
        self.dy += 1;
        if !self.is_vertical_collision() {
            return
        }
        self.dy -= 1;
        self.place();
        let tetromino = Tetromino::random();
        self.tetromino = tetromino;
        self.projection = tetromino;
        self.dy = 0;
        self.dx = 0;
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
        self.dy = self.projection_dy
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
            x.wrapping_add_signed(self.dx) > WIDTH || self.blocks.contains(&self.block(y, x))
        })
    }

    fn is_vertical_collision(&self) -> bool {
        self.tetromino.blocks().iter().any(|&[y, x]| {
            self.blocks.contains(&self.block(y, x))
                || y + self.dy > HEIGHT
        })
    }

    fn place(&mut self) {
        for [y, x] in self.tetromino.blocks() {
            self.blocks.insert(self.block(y, x));
        }
    }

    fn render(&self) {
        fn draw_block(block: [usize; 2]) {
            let [y, x] = block;
            draw_rectangle(x as f32 * 20.0, y as f32 * 20.0, 20.0, 20.0, RED);
        }

        set_camera(&self.canvas.camera);
        clear_background(WHITE);

        for &[y, x] in &self.projection.blocks() {
            let [y, x] = [y + self.projection_dy, x.saturating_add_signed(self.dx)];
            draw_rectangle(
                x as f32 * 20.0,
                y as f32 * 20.0,
                20.0,
                20.0,
                Color::from_rgba(255, 0, 0, 100)
            );
        }

        for &block in &self.blocks {
            draw_block(block);
        }

        for &[y, x] in &self.tetromino.blocks() {
            draw_block(self.block(y, x));
        }

        set_default_camera();
        clear_background(BLACK);
        self.canvas.draw();
    }
}

use crate::Game;
use crate::{Block, HEIGHT, WIDTH};
use macroquad::prelude::*;

pub fn render(game: &Game) {
    fn draw_block(block: Block, color: Color) {
        draw_rectangle(
            block.x as f32 * 20.0,
            block.y as f32 * 20.0,
            20.0,
            20.0,
            color,
        );
    }

    set_camera(&game.canvas.camera);
    clear_background(WHITE);

    for &[y, x] in &game.projection.blocks() {
        let mut color = Color::from_hex(game.projection.color);
        color.a = 0.1;
        let block = Block {
            x: x.saturating_add_signed(game.dx),
            y: y + game.projection_dy,
            color: game.projection.color,
        };
        draw_block(block, color);
    }

    for &block in &game.blocks {
        draw_block(block, Color::from_hex(block.color));
    }

    for &[y, x] in &game.tetromino.blocks() {
        draw_block(game.block(y, x), Color::from_hex(game.tetromino.color));
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
    game.canvas.draw();
}

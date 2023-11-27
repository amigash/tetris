use macroquad::{
    text::draw_text,
    shapes::{draw_rectangle, draw_rectangle_lines},
    color::{BLACK, Color, WHITE},
    camera::{set_camera, set_default_camera},
    window::clear_background
};
use crate::{
    Block,
    BLOCK_SIZE,
    HEIGHT,
    WIDTH,
    Game,
    SCREEN_HEIGHT,
    SCREEN_WIDTH,
    LINES_TO_LEVEL
};
use macroquad_canvas::Canvas2D;

const TRANSPARENCY: f32 = 0.25;
const PRIMARY_COLOR: Color = WHITE;
const SECONDARY_COLOR: Color = BLACK;

pub fn render(canvas: &mut Canvas2D, game: &Game) {
    fn draw_block(block: Block, color: Color) {
        draw_rectangle(
            block.x as f32 * BLOCK_SIZE,
            block.y as f32 * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
            color,
        );
    }

    set_camera(&canvas.camera);
    clear_background(SECONDARY_COLOR);

    let mut color = Color::from_hex(game.projection.color);
    color.a = TRANSPARENCY;
    for &[y, x] in &game.projection.blocks() {
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
        WIDTH as f32 * BLOCK_SIZE,
        HEIGHT as f32 * BLOCK_SIZE,
        5.0,
        PRIMARY_COLOR,
    );

    set_default_camera();

    draw_text(
        format!("Level: {}", game.lines_cleared / LINES_TO_LEVEL + 1).as_str(),
        SCREEN_WIDTH / 20.0,
        SCREEN_HEIGHT / 15.0,
        100.0,
        PRIMARY_COLOR,
    );

    draw_text(
        format!("Score: {}", game.lines_cleared).as_str(),
        SCREEN_WIDTH / 20.0,
        SCREEN_HEIGHT / 8.0,
        100.0,
        PRIMARY_COLOR,
    );
    canvas.draw();
}

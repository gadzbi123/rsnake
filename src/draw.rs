use crate::colors;
use crate::physics::{Direction, Position};
use piston_window::types::Color;
use piston_window::{ellipse, rectangle, Context, G2d};
use std::f64::consts::PI;
use std::path::Path;

pub const BLOCK_SIZE: f64 = 25.0;

pub fn draw_block(ctx: &Context, g: &mut G2d, c: Color, pos: &Position) {
    rectangle(
        c,
        [
            pos.x as f64 * BLOCK_SIZE,
            pos.y as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
        ],
        ctx.transform,
        g,
    );
}

pub fn draw_snake_head(ctx: &Context, g: &mut G2d, c: Color, pos: &Position, dir: &Direction) {
    draw_block(ctx, g, c, pos);

    fn draw_eye(ctx: &Context, g: &mut G2d, x: f64, y: f64) {
        rectangle(colors::BACKGROUND, [x, y, 5.0, 5.0], ctx.transform, g);
    }

    let (x, y) = (
        blocks_in_pixels(pos.x as u32) as f64,
        blocks_in_pixels(pos.y as u32) as f64,
    );

    let block = blocks_in_pixels(1) as f64;

    match dir {
        Direction::Up => {
            draw_eye(ctx, g, x + 5.0, y + 5.0);
            draw_eye(ctx, g, x + block - 10.0, y + 5.0);
        }
        Direction::Right => {
            draw_eye(ctx, g, x + block - 10.0, y + 5.0);
            draw_eye(ctx, g, x + block - 10.0, y + block - 10.0);
        }
        Direction::Down => {
            draw_eye(ctx, g, x + 5.0, y + block - 10.0);
            draw_eye(ctx, g, x + block - 10.0, y + block - 10.0);
        }
        Direction::Left => {
            draw_eye(ctx, g, x + 5.0, y + 5.0);
            draw_eye(ctx, g, x + 5.0, y + block - 10.0);
        }
    }
}

pub fn draw_fruit(ctx: &Context, g: &mut G2d, c: (Color, Color), pos: &Position) {
    ellipse(
        c.0,
        [
            pos.x as f64 * BLOCK_SIZE,
            pos.y as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
        ],
        ctx.transform,
        g,
    );
    // vertical tree part
    rectangle(
        c.1,
        [
            pos.x as f64 * BLOCK_SIZE + 11.0,
            pos.y as f64 * BLOCK_SIZE - 6.0,
            4.0,
            7.0,
        ],
        ctx.transform,
        g,
    );
    // horizontal tree part
    rectangle(
        c.1,
        [
            pos.x as f64 * BLOCK_SIZE + 11.0,
            pos.y as f64 * BLOCK_SIZE - 8.0,
            7.0,
            4.0,
        ],
        ctx.transform,
        g,
    );
    //    let image = Image::new().rect(square(0.0, 0.0, 200.0));
    //Texture::from_path(
    //g,
    //Path::new("Example.png"),
    //Flip::None,
    //&TextureSettings::new(),
    //)
    //.unwrap();
}

pub fn draw_overlay(ctx: &Context, g: &mut G2d, c: Color, size: (u32, u32)) {
    rectangle(
        c,
        [
            0.0,
            0.0,
            blocks_in_pixels(size.0) as f64,
            blocks_in_pixels(size.1) as f64,
        ],
        ctx.transform,
        g,
    );
}

pub fn blocks_in_pixels(n: u32) -> u32 {
    n * BLOCK_SIZE as u32
}

mod fractals;
mod visualcomputer;

use fractals::*;
use macroquad::prelude::*;

use crate::visualcomputer::compute_pixels;

#[macroquad::main("fractalviewer")]
async fn main() {
    let mb = Mandelbrot::new(200, 2.0);
    let image = compute_pixels(mb);
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);
        request_new_screen_size(image.width() as f32, image.height() as f32);
        draw_texture(&texture, 0.0, 0.0, WHITE);
        next_frame().await
    }
}

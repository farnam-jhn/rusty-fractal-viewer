use crate::fractals::Fractal;
use macroquad::prelude::*;
use num_complex::Complex;

pub fn compute_pixels(fractal: impl Fractal, width: u16, height: u16) -> Image {
    // Create a black image with pre-allocated buffer memory
    let mut image = Image::gen_image_color(width, height, BLACK);

    // Coordinate bounds for mapping pixels to the complex plane
    let re_min = fractal.get_centerx() - 1.5;
    let re_max = fractal.get_centerx() + 1.5;
    let im_min = -1.5;
    let im_max = 1.5;

    let max_iter = fractal.config().max_iterations as f32;

    for x in 0..width {
        for y in 0..height {
            let re = re_min + (x as f64 / width as f64) * (re_max - re_min);
            let im = im_min + (y as f64 / height as f64) * (im_max - im_min);

            let c = Complex::new(re, im);
            let iter = fractal.iterations_count(c);

            let color = if iter == fractal.config().max_iterations {
                BLACK
            } else {
                let t = iter as f32 / max_iter;
                Color::new(t, t, t, 1.0)
            };

            image.set_pixel(x as u32, y as u32, color);
        }
    }

    image
}

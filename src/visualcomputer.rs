use crate::fractals::Fractal;
use macroquad::prelude::*;
use num_complex::Complex;
use std::{cmp, vec};

/// Pixel computer: iterates through the point and evaluates the pixel's color
pub fn compute_pixels(fractal: impl Fractal, width: u16, height: u16) -> Image {
    // Create a black image with pre-allocated buffer memory
    let mut image = Image::gen_image_color(width, height, BLACK);

    // Coordinate bounds for mapping pixels to the complex plane
    let re_min = fractal.get_centerx() - 1.5;
    let re_max = fractal.get_centerx() + 1.5;
    let im_min = -1.5;
    let im_max = 1.5;

    let max_iter = fractal.config().max_iterations;

    for x in 0..width {
        for y in 0..height {
            let re = re_min + (x as f64 / width as f64) * (re_max - re_min);
            let im = im_min + (y as f64 / height as f64) * (im_max - im_min);

            let point = Complex::new(re, im);
            let iter = fractal.iterations_count(point);

            let color = smooth_color(COLOR_PALLETE, iter, max_iter);

            image.set_pixel(x as u32, y as u32, color);
        }
    }

    image
}

/// Linear interpolation between two colors.
fn lerp_color(color1: Color, color2: Color, t: f64) -> Color {
    let lerp = |x: f32, y: f32| ((x + (y - x)) as f64 * t) as f32;

    Color {
        r: lerp(color1.r, color2.r),
        g: lerp(color1.g, color2.g),
        b: lerp(color1.b, color2.b),
        a: lerp(color1.a, color2.a),
    }
}

/// Implementation of linear interpolation on colors
fn smooth_color(pallete: &[Color], iterations: i32, max_iterations: i32) -> Color {
    if pallete.is_empty() {
        return BLACK;
    }

    if iterations >= max_iterations {
        return *pallete.last().unwrap();
    }

    let t: f64 = iterations as f64 / max_iterations as f64;
    let scaled: f64 = t * (pallete.len() - 1) as f64;

    let i: usize = scaled as usize;
    let j: usize = cmp::min(i + 1, pallete.len() - 1);

    let fraction: f64 = scaled - i as f64;
    return lerp_color(pallete[i], pallete[j], fraction);
}

const COLOR_PALLETE: &[Color] = &[
    Color::from_rgba(8, 10, 25, 255),
    Color::from_rgba(15, 20, 45, 255),
    Color::from_rgba(25, 35, 70, 255),
    Color::from_rgba(35, 55, 100, 255),
    Color::from_rgba(50, 80, 130, 255),
    Color::from_rgba(70, 105, 160, 255),
    Color::from_rgba(95, 130, 185, 255),
    Color::from_rgba(120, 155, 205, 255),
    Color::from_rgba(145, 180, 220, 255),
    Color::from_rgba(170, 200, 230, 255),
    Color::from_rgba(190, 215, 235, 255),
    Color::from_rgba(210, 225, 230, 255),
    Color::from_rgba(235, 210, 180, 255),
    Color::from_rgba(245, 180, 130, 255),
    Color::from_rgba(250, 145, 90, 255),
    Color::from_rgba(250, 110, 60, 255),
    Color::from_rgba(245, 80, 45, 255),
    Color::from_rgba(235, 55, 40, 255),
    Color::from_rgba(210, 40, 40, 255),
    Color::from_rgba(170, 30, 45, 255),
    Color::from_rgba(120, 22, 42, 255),
    Color::from_rgba(70, 15, 35, 255),
    Color::from_rgba(30, 8, 25, 255),
    Color::from_rgba(0, 0, 0, 255),
];

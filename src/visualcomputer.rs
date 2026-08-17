use crate::fractals::Fractal;
use macroquad::prelude::*;
use num_complex::Complex;
use std::cmp;

/// Viewport is used to compute a specific region
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub center_re: f64,
    pub center_im: f64,
    pub zoom: f64,
}

impl Viewport {
    pub fn new(center_re: f64, center_im: f64, zoom: f64) -> Self {
        Self {
            center_re,
            center_im,
            zoom: zoom.max(1e-12),
        }
    }

    pub fn default_for_fractal(fractal: &impl Fractal) -> Self {
        Self {
            center_re: fractal.get_centerx(),
            center_im: 0.0,
            zoom: 1.0,
        }
    }

    pub fn pan(&mut self, delta_re: f64, delta_im: f64) {
        self.center_re += delta_re / self.zoom;
        self.center_im += delta_im / self.zoom;
    }

    pub fn zoom_by(&mut self, factor: f64) {
        self.zoom = (self.zoom * factor).max(1e-12);
    }

    /// Zoom in/out anchored at specific pixel coordinates (e.g. mouse cursor position)
    pub fn zoom_at(&mut self, factor: f64, x: u16, y: u16, width: u16, height: u16) {
        let point_before = self.pixel_to_complex(x, y, width, height);
        self.zoom = (self.zoom * factor).max(1e-12);
        let point_after = self.pixel_to_complex(x, y, width, height);
        self.center_re += point_before.re - point_after.re;
        self.center_im += point_before.im - point_after.im;
    }

    /// Map pixel coordinates (x, y) into complex plane (re, im) taking image aspect ratio into account
    pub fn pixel_to_complex(&self, x: u16, y: u16, width: u16, height: u16) -> Complex<f64> {
        let aspect_ratio = width as f64 / height as f64;
        let scale = 3.0 / self.zoom;

        let span_re = scale * aspect_ratio.max(1.0);
        let span_im = scale * (1.0 / aspect_ratio).max(1.0);

        let re = self.center_re + (x as f64 / width as f64 - 0.5) * span_re;
        let im = self.center_im + (y as f64 / height as f64 - 0.5) * span_im;

        Complex::new(re, im)
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            center_re: -0.5,
            center_im: 0.0,
            zoom: 1.0,
        }
    }
}

/// Dynamic color palette definition supporting custom colors, phase offsets, and cycling
#[derive(Debug, Clone)]
pub struct Palette {
    pub colors: Vec<Color>,
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    /// Classic fractal gradient matching MandelbrotExplorer colorPalette
    pub fn classic() -> Self {
        Self {
            colors: COLOR_PALETTE_CLASSIC.to_vec(),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::classic()
    }
}

/// Dynamic render configuration combining Viewport mapping and Palette styling
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub viewport: Viewport,
    pub palette: Palette,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            viewport: Viewport::default(),
            palette: Palette::default(),
        }
    }
}

/// Dynamic pixel computer supporting custom Viewport and Palette
pub fn compute_pixels_dynamic(
    fractal: &impl Fractal,
    width: u16,
    height: u16,
    config: &RenderConfig,
) -> Image {
    let mut image = Image::gen_image_color(width, height, BLACK);
    let max_iter = fractal.config().max_iterations;

    for y in 0..height {
        for x in 0..width {
            let point = config.viewport.pixel_to_complex(x, y, width, height);
            let iter = fractal.iterations_count(point);
            let color = smooth_color(&config.palette, iter, max_iter);
            image.set_pixel(x as u32, y as u32, color);
        }
    }

    image
}

/// Linear interpolation between two colors
pub fn lerp_color(color1: Color, color2: Color, t: f64) -> Color {
    let lerp = |x: f32, y: f32| -> f32 {
        let x_u8 = (x * 255.0) as f64;
        let y_u8 = (y * 255.0) as f64;
        let res_u8 = (x_u8 + (y_u8 - x_u8) * t) as u8;
        res_u8 as f32 / 255.0
    };

    Color {
        r: lerp(color1.r, color2.r),
        g: lerp(color1.g, color2.g),
        b: lerp(color1.b, color2.b),
        a: lerp(color1.a, color2.a),
    }
}

/// implementation of color interpolation
pub fn smooth_color(palette: &Palette, iteration: i32, max_iterations: i32) -> Color {
    if palette.colors.is_empty() {
        return BLACK;
    }

    if iteration >= max_iterations {
        return *palette.colors.last().unwrap_or(&BLACK);
    }

    let t = iteration as f64 / max_iterations as f64;
    let scaled = t * (palette.colors.len() - 1) as f64;

    let i = scaled as usize;
    let j = cmp::min(i + 1, palette.colors.len() - 1);
    let fraction = scaled - i as f64;

    lerp_color(palette.colors[i], palette.colors[j], fraction)
}

const COLOR_PALETTE_CLASSIC: &[Color] = &[
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

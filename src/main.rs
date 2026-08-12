mod fractals;
mod visualcomputer;

use crate::visualcomputer::*;
use fractals::*;
use macroquad::prelude::*;
use num_complex::Complex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FractalType {
    Mandelbrot,
    Julia,
    BurningShip,
    Tricorn,
}

fn get_fractal_name(fractal_type: FractalType) -> String {
    match fractal_type {
        FractalType::Mandelbrot => Mandelbrot::get_name(),
        FractalType::Julia => Julia::get_name(),
        FractalType::BurningShip => BurningShip::get_name(),
        FractalType::Tricorn => Tricorn::get_name(),
    }
}

fn render_fractal(
    fractal_type: FractalType,
    max_iterations: i32,
    width: u16,
    height: u16,
    config: &RenderConfig,
) -> Image {
    let max_mag = 2.0;
    match fractal_type {
        FractalType::Mandelbrot => {
            let f = Mandelbrot::new(max_iterations, max_mag);
            compute_pixels_dynamic(&f, width, height, config)
        }
        FractalType::Julia => {
            let f = Julia::new(max_iterations, max_mag, Complex { re: -0.7, im: 0.27 });
            compute_pixels_dynamic(&f, width, height, config)
        }
        FractalType::BurningShip => {
            let f = BurningShip::new(max_iterations, max_mag);
            compute_pixels_dynamic(&f, width, height, config)
        }
        FractalType::Tricorn => {
            let f = Tricorn::new(max_iterations, max_mag);
            compute_pixels_dynamic(&f, width, height, config)
        }
    }
}

fn default_viewport_for(fractal_type: FractalType) -> Viewport {
    match fractal_type {
        FractalType::Mandelbrot => Viewport::new(-0.5, 0.0, 1.0),
        FractalType::Julia => Viewport::new(0.0, 0.0, 1.0),
        FractalType::BurningShip => Viewport::new(-0.45, -0.5, 1.0),
        FractalType::Tricorn => Viewport::new(-0.5, 0.0, 1.0),
    }
}

#[macroquad::main("Rusty Fractal Viewer")]
async fn main() {
    let mut width: u16 = 1000;
    let mut height: u16 = 1000;

    let mut current_fractal = FractalType::Mandelbrot;
    let mut max_iterations: i32 = 200;

    let current_palette = Palette::classic();
    let mut viewport = default_viewport_for(current_fractal);

    let mut render_config = RenderConfig {
        viewport,
        palette: current_palette.clone(),
    };

    let mut image = render_fractal(
        current_fractal,
        max_iterations,
        width,
        height,
        &render_config,
    );
    let mut texture = Texture2D::from_image(&image);

    let mut last_mouse_pos: Option<(f32, f32)> = None;
    let mut show_hud = true;

    loop {
        let mut needs_rebuild = false;

        let screen_w = screen_width() as u16;
        let screen_h = screen_height() as u16;
        if screen_w > 0 && screen_h > 0 && (screen_w != width || screen_h != height) {
            width = screen_w;
            height = screen_h;
            needs_rebuild = true;
        }

        // Mouse zooming
        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            let zoom_factor = if wheel_y > 0.0 { 1.05 } else { 1.0 / 1.05 };
            let (mx, my) = mouse_position();
            let clamped_x = mx.clamp(0.0, width as f32) as u16;
            let clamped_y = my.clamp(0.0, height as f32) as u16;
            viewport.zoom_at(zoom_factor, clamped_x, clamped_y, width, height);
            needs_rebuild = true;
        }

        // Mouse dragging
        if is_mouse_button_pressed(MouseButton::Left) {
            last_mouse_pos = Some(mouse_position());
        }
        if is_mouse_button_down(MouseButton::Left) {
            if let Some((lx, ly)) = last_mouse_pos {
                let (cx, cy) = mouse_position();
                let dx = cx - lx;
                let dy = cy - ly;

                if dx != 0.0 || dy != 0.0 {
                    let aspect_ratio = width as f64 / height as f64;
                    let scale = 3.0 / viewport.zoom;
                    let span_re = scale * aspect_ratio.max(1.0);
                    let span_im = scale * (1.0 / aspect_ratio).max(1.0);

                    viewport.center_re -= (dx as f64 / width as f64) * span_re;
                    viewport.center_im -= (dy as f64 / height as f64) * span_im;
                    needs_rebuild = true;
                }
                last_mouse_pos = Some((cx, cy));
            }
        } else {
            last_mouse_pos = None;
        }

        // Keyboard zooming and navigation
        if is_key_down(KeyCode::Equal) || is_key_down(KeyCode::KpAdd) {
            let (mx, my) = (width as u16 / 2, height as u16 / 2);
            viewport.zoom_at(1.04, mx, my, width, height);
            needs_rebuild = true;
        }
        if is_key_down(KeyCode::Minus) || is_key_down(KeyCode::KpSubtract) {
            let (mx, my) = (width as u16 / 2, height as u16 / 2);
            viewport.zoom_at(1.0 / 1.04, mx, my, width, height);
            needs_rebuild = true;
        }

        let key_pan_speed = 0.09 / viewport.zoom;
        if is_key_down(KeyCode::A) {
            viewport.center_re -= key_pan_speed;
            needs_rebuild = true;
        }
        if is_key_down(KeyCode::D) {
            viewport.center_re += key_pan_speed;
            needs_rebuild = true;
        }
        if is_key_down(KeyCode::W) {
            viewport.center_im -= key_pan_speed;
            needs_rebuild = true;
        }
        if is_key_down(KeyCode::S) {
            viewport.center_im += key_pan_speed;
            needs_rebuild = true;
        }

        // Reset Viewport
        if is_key_pressed(KeyCode::R) {
            viewport = default_viewport_for(current_fractal);
            needs_rebuild = true;
        }

        // Fractal Selection
        if is_key_pressed(KeyCode::Key1) {
            current_fractal = FractalType::Mandelbrot;
            viewport = default_viewport_for(current_fractal);
            needs_rebuild = true;
        }
        if is_key_pressed(KeyCode::Key2) {
            current_fractal = FractalType::Julia;
            viewport = default_viewport_for(current_fractal);
            needs_rebuild = true;
        }
        if is_key_pressed(KeyCode::Key3) {
            current_fractal = FractalType::BurningShip;
            viewport = default_viewport_for(current_fractal);
            needs_rebuild = true;
        }
        if is_key_pressed(KeyCode::Key4) {
            current_fractal = FractalType::Tricorn;
            viewport = default_viewport_for(current_fractal);
            needs_rebuild = true;
        }

        if is_key_pressed(KeyCode::Right) {
            max_iterations = (max_iterations + 50).min(5000);
            needs_rebuild = true;
        }
        if is_key_pressed(KeyCode::Left) {
            max_iterations = (max_iterations - 50).max(10);
            needs_rebuild = true;
        }

        // Toggle head up display
        if is_key_pressed(KeyCode::H) {
            show_hud = !show_hud;
        }

        // Recompute image if settings or view moved
        if needs_rebuild {
            render_config.viewport = viewport;
            render_config.palette = current_palette.clone();

            image = render_fractal(
                current_fractal,
                max_iterations,
                width,
                height,
                &render_config,
            );
            if texture.width() as u16 != width || texture.height() as u16 != height {
                texture = Texture2D::from_image(&image);
            } else {
                texture.update(&image);
            }
        }

        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        if show_hud {
            // Head up display overlay
            draw_rectangle(15.0, 15.0, 360.0, 185.0, Color::new(0.0, 0.0, 0.0, 0.75));

            let font_size = 20.0;
            let text_color = WHITE;

            draw_text(
                &format!("Fractal: {}", get_fractal_name(current_fractal)),
                25.0,
                40.0,
                24.0,
                GOLD,
            );
            draw_text(
                &format!("Zoom: {:.2e}x", viewport.zoom),
                25.0,
                65.0,
                font_size,
                text_color,
            );
            draw_text(
                &format!(
                    "Center: ({:.6}, {:.6})",
                    viewport.center_re, viewport.center_im
                ),
                25.0,
                85.0,
                font_size,
                text_color,
            );
            draw_text(
                &format!("Max Iterations: {}", max_iterations),
                25.0,
                105.0,
                font_size,
                text_color,
            );

            draw_text(
                "Controls: Scroll/+- (Zoom), Drag/WASD",
                25.0,
                150.0,
                16.0,
                LIGHTGRAY,
            );
            draw_text(
                "1-4 (Fractal), <-/-> (Iter), R (Reset), H (HUD)",
                25.0,
                170.0,
                16.0,
                LIGHTGRAY,
            );
            draw_text(&format!("FPS: {}", get_fps()), 25.0, 190.0, 16.0, GREEN);
        }

        next_frame().await;
    }
}

use macroquad::prelude::*;

#[macroquad::main("fractalviewer")]
async fn main() {
    loop {
        request_new_screen_size(200.0, 100.0);
        clear_background(WHITE);
        draw_circle(100f32, 50f32, 20.0, BLACK);
        next_frame().await
    }
}

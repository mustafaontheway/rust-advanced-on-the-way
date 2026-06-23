use macroquad::prelude::*;

#[macroquad::main("Mini Game")]
async fn main() {
    
    let mut x = 100.0;

    loop {
      
        clear_background(BLACK);

        if is_key_down(KeyCode::Right) {
            x += 2.0;
        }

        draw_rectangle(x, 100.0, 10.0, 10.0, WHITE);

        next_frame().await;
    }
}

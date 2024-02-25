mod engine;

use macroquad::input::KeyCode::{Escape, Space};
use macroquad::prelude::*;
use crate::engine::Engine;

//fps for physics
const FPS: i32 = 20;
const TIME_PER_FRAME: f32 = 1f32 / FPS as f32;


fn get_conf() -> Conf {
    Conf {
        window_title: "Marching Squares".to_string(),
        window_width: 1200,
        window_height: 600,
        high_dpi: true,
        fullscreen: false,
        sample_count: 0,
        window_resizable: true,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(get_conf())]
async fn main() {
    let mut engine = Engine::new(IVec2::new(120, 60));
    let mut z = 0f32;
    engine.update_interpolation_values(z);


    let mut lag = 0f32;
    loop {
        if is_key_down(Escape) {break}
        if is_key_down(Space) {
            z += 0.4f32;
            engine.update_interpolation_values(z)
        }
        lag += get_frame_time();
        while lag >= TIME_PER_FRAME {
            lag -= TIME_PER_FRAME;

            z += 0.08f32;
            engine.update_interpolation_values(z)

        }
        clear_background(BLACK);
        //engine.draw_grid();
        engine.draw_marching_squares();


        next_frame().await;
    }
}

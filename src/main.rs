mod engine;

use macroquad::input::KeyCode::Escape;
use macroquad::prelude::*;
use crate::engine::Engine;

//fps for physics
const FPS: i32 = 30;
const TIME_PER_FRAME: f32 = 1f32 / FPS as f32;


fn get_conf() -> Conf {
    Conf {
        window_title: "Marching Squares".to_string(),
        window_width: 1200,
        window_height: 600,
        high_dpi: false,
        fullscreen: false,
        sample_count: 0,
        window_resizable: true,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(get_conf())]
async fn main() {
    let engine = Engine::new(IVec2::new(100, 50));


    let mut lag = 0f32;
    loop {
        if is_key_down(Escape) {break}
        lag += get_frame_time();
        while lag >= TIME_PER_FRAME {
            lag -= TIME_PER_FRAME;


        }
        clear_background(BLACK);
        engine.draw_grid();


        next_frame().await;
    }
}

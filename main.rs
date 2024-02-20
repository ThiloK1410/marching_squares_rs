use macroquad::prelude::*;

//fps for physics
const FPS: i32 = 30;
const TIME_PER_FRAME: f32 = 1f32 / FPS as f32;

//gets called once at program start
pub fn init() {

}

//fixed rate loop (physics and such)
pub fn on_loop() {

}

//variable speed loop, draw here
pub fn on_render() {
    clear_background(WHITE);
}

#[macroquad::main("WindowName")]
async fn main() {
    init();
    let mut lag = 0f32;
    loop {
        lag += get_frame_time();
        while lag >= TIME_PER_FRAME {
            on_loop();
            lag -= TIME_PER_FRAME;
        }
        on_render();

        next_frame().await;
    }
}

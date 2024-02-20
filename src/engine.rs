use macroquad::color::WHITE;
use macroquad::math::{IVec2, Vec2};
use macroquad::shapes::draw_line;
use macroquad::window::{screen_height, screen_width};
use opensimplex_noise_rs::OpenSimplexNoise;

pub struct Engine {
    grid_size: IVec2,
    noise_generator: OpenSimplexNoise,
}

impl Engine {
    pub fn new(size: IVec2) -> Self {
        Engine {
            grid_size: size,
            noise_generator: OpenSimplexNoise::new(Some(123)),
        }
    }
    pub fn draw_grid(&self) {
        let color = WHITE;
        let line_thickness = 1f32;

        let square_size: Vec2 = Vec2::new(screen_width()/self.grid_size.x as f32,
                                          screen_height()/self.grid_size.y as f32);
        for i in 1..self.grid_size.x {
            draw_line(i as f32*square_size.x, 0f32,
                      i as f32*square_size.x, screen_height(),
            line_thickness, color);
        }
        for i in 1..self.grid_size.y {
            draw_line(0f32, i as f32*square_size.y,
            screen_width(), i as f32*square_size.y,
            line_thickness, color);
        }
    }
}
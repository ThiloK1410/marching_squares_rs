use macroquad::color::{WHITE, YELLOW};
use macroquad::math::{IVec2, Vec2};
use macroquad::shapes::{draw_circle, draw_line};
use macroquad::window::{screen_height, screen_width};
use opensimplex_noise_rs::OpenSimplexNoise;

pub struct Engine {
    grid_size: IVec2,
    noise_size: IVec2,
    noise_generator: OpenSimplexNoise,
    interpolation_values_vertical: Vec<Vec<Option<(f32,f32)>>>,
    interpolation_values_horizontal: Vec<Vec<Option<(f32,f32)>>>,
    z_value: f32,
    scale: f64,
}

impl Engine {
    pub fn new(size: IVec2) -> Self {
        Engine {
            grid_size: size,
            noise_size: IVec2::new(size.x + 1, size.y + 1),
            noise_generator: OpenSimplexNoise::new(Some(123)),
            interpolation_values_vertical: vec![vec![Some((0f32,0f32)); (size.x + 1) as usize]; (size.y + 1) as usize],
            interpolation_values_horizontal: vec![vec![Some((0f32,0f32)); (size.x + 1) as usize]; (size.y + 1) as usize],
            z_value: 0f32,
            scale: 0.2_f64,
        }
    }
    pub fn draw_grid(&self) {
        let color = WHITE;
        let line_thickness = 1f32;

        let square_size: Vec2 = Vec2::new(screen_width() / self.grid_size.x as f32,
                                          screen_height() / self.grid_size.y as f32);
        for i in 1..self.grid_size.x {
            draw_line(i as f32 * square_size.x, 0f32,
                      i as f32 * square_size.x, screen_height(),
                      line_thickness, color);
        }
        for i in 1..self.grid_size.y {
            draw_line(0f32, i as f32 * square_size.y,
                      screen_width(), i as f32 * square_size.y,
                      line_thickness, color);
        }
    }
    pub fn update_interpolation_values(&mut self, z: f32) {
        self.z_value = z;
        let square_size: Vec2 = Vec2::new(screen_width() / self.grid_size.x as f32,
                                          screen_height() / self.grid_size.y as f32);
        for y in 0..=self.grid_size.y as usize {
            for x in 0..=self.grid_size.x as usize {
                let point1 = self.noise_generator.eval_3d(
                    x as f64 * self.scale,
                    y as f64 * self.scale,
                    z as f64 * self.scale) as f32;
                let point2 = self.noise_generator.eval_3d(
                    x as f64 * self.scale,
                    (y + 1) as f64 * self.scale,
                    z as f64 * self.scale) as f32;
                if point1 * point2 <= 0f32 {
                    self.interpolation_values_vertical[y][x] = Some(
                        (square_size.x*x as f32, square_size.y*y as f32+(point1.abs() / (point2 - point1).abs())*square_size.y))
                } else {
                    self.interpolation_values_vertical[y][x] = None
                }

                let point1 = self.noise_generator.eval_3d(
                    x as f64 * self.scale,
                    y as f64 * self.scale,
                    z as f64 * self.scale) as f32;
                let point2 = self.noise_generator.eval_3d(
                    (x + 1) as f64 * self.scale,
                    y as f64 * self.scale,
                    z as f64 * self.scale) as f32;
                if point1 * point2 <= 0f32 {
                    self.interpolation_values_horizontal[y][x] = Some(
                        (square_size.x*x as f32+(point1.abs() / (point2 - point1).abs())*square_size.x, square_size.y*y as f32))
                } else {
                    self.interpolation_values_horizontal[y][x] = None
                }
            }
        }
    }
    pub fn draw_interpolation_points(&self) {
        for y in 0..=self.grid_size.y as usize {
            for x in 0..=self.grid_size.x as usize {
                if let Some(val) = self.interpolation_values_vertical[y][x] {
                    draw_circle(val.0, val.1, 4f32, WHITE);
                }
                if let Some(val) = self.interpolation_values_horizontal[y][x] {
                    draw_circle(val.0, val.1, 4f32, WHITE);
                }
            }
        }
    }
    pub fn draw_marching_squares(&self) {
        let thickness = 1f32;
        let color = YELLOW;
        for y in 0..self.grid_size.y as usize {
            for x in 0..self.grid_size.x as usize {
                let points = vec!(
                    self.interpolation_values_horizontal[y][x],
                    self.interpolation_values_vertical[y][x + 1],
                    self.interpolation_values_horizontal[y + 1][x],
                    self.interpolation_values_vertical[y][x]);
                let mut point_vec: Vec<(f32, f32)> = Vec::new();
                for x in points.iter() {
                    if let Some(tuple) = x {
                        point_vec.push(*tuple);
                    }
                }
                match point_vec.len() {
                    2 => {
                        draw_line(point_vec[0].0, point_vec[0].1, point_vec[1].0, point_vec[1].1,
                                  thickness, color)
                    }
                    4 => {
                        let mid_value = self.noise_generator.eval_3d(
                            (x as f64+0.5f64) * self.scale,
                            (y as f64+0.5f64) * self.scale,
                            self.z_value as f64 * self.scale) * self.noise_generator.eval_3d(
                            x as f64 * self.scale,
                            y as f64 * self.scale,
                            self.z_value as f64 * self.scale);
                        if mid_value < 0f64 {
                            draw_line(point_vec[0].0, point_vec[0].1, point_vec[3].0, point_vec[3].1,
                                      thickness, color);
                            draw_line(point_vec[1].0, point_vec[1].1, point_vec[2].0, point_vec[2].1,
                                      thickness, color)
                        } else {
                            draw_line(point_vec[0].0, point_vec[0].1, point_vec[1].0, point_vec[1].1,
                                      thickness, color);
                            draw_line(point_vec[3].0, point_vec[3].1, point_vec[2].0, point_vec[2].1,
                                      thickness, color)
                        }
                    }
                    _ => ()
                }
            }
        }
    }
}
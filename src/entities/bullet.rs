use macroquad::prelude::*;

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub is_alive: bool,
    pub size: f32,
    pub speed: f32,
}
impl Bullet {
    pub fn new(x: f32, y: f32, color: Color, size: f32, speed: f32) -> Bullet {
        Bullet {
            x,
            y,
            color,
            is_alive: true,
            size,
            speed,
        }
    }

    pub fn draw(&mut self, texture: &Texture2D) {
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
    }
}

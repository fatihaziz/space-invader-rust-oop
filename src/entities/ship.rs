use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Ship {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub is_alive: bool,
    pub size: f32,
    pub speed: f32,
}

impl Ship {
    pub fn new(x: f32, y: f32, color: Color, is_alive: bool, size: f32, speed: f32) -> Ship {
        Ship {
            x,
            y,
            color,
            is_alive,
            size,
            speed,
        }
    }

    pub fn draw(&mut self, texture: &Texture2D) {
        draw_texture_ex(
            *texture,
            self.x,
            self.y,
            self.color,
            DrawTextureParams {
                dest_size: Some(vec2(self.size, self.size)),
                ..Default::default()
            },
        );
    }

    pub fn left(&mut self) {
        self.x -= self.speed;
    }
    pub fn right(&mut self) {
        self.x += self.speed;
    }
    pub fn up(&mut self) {
        self.y -= self.speed;
    }
    pub fn down(&mut self) {
        self.y += self.speed;
    }
}

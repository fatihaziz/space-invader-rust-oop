use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub is_alive: bool,
    pub size: f32,
    pub speed: f32,
    pub score: f32,
}
impl Enemy {
    pub fn new(x: f32, y: f32, color: Color, size: f32, speed: f32, score: f32) -> Enemy {
        Enemy {
            x,
            y,
            color,
            is_alive: true,
            size,
            speed,
            score,
        }
    }

    pub fn kill(&mut self) {
        self.is_alive = false;
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
}

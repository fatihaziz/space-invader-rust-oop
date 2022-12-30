use macroquad::prelude::*;

use super::bullet::Bullet;

#[derive(Clone, Copy)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub is_alive: bool,
    pub size: f32,
    pub speed: f32,
    pub health: f32,
    pub score: f32,
}
impl Enemy {
    pub fn new(
        x: f32,
        y: f32,
        color: Color,
        size: f32,
        speed: f32,
        health: f32,
        score: f32,
    ) -> Enemy {
        Enemy {
            x,
            y,
            color,
            is_alive: true,
            size,
            speed,
            health,
            score,
        }
    }

    pub fn kill(&mut self, bullet: Bullet) {
        self.health -= bullet.damage;
        if self.health <= 0.0 {
            self.is_alive = false;
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

    pub fn moving(&mut self, window_width: f32) {
        let enemy = self;
        enemy.x += enemy.speed;
        if enemy.x > window_width - enemy.size {
            enemy.x = window_width - enemy.size;
            enemy.y += enemy.size;
            enemy.speed *= -1.0;
        }
        if enemy.x < 0.0 {
            enemy.x = 0.0;
            enemy.y += enemy.size;
            enemy.speed *= -1.0;
        }
    }
}

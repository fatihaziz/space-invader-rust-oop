use macroquad::prelude::*;

use super::enemy::Enemy;

#[derive(Copy, Clone, PartialEq)]
pub enum BulletBy {
    PLAYER,
    ENEMY,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub is_alive: bool,
    pub size: f32,
    pub speed: f32,
    pub damage: f32,
    pub by: BulletBy,
}
impl Bullet {
    pub fn new(
        x: f32,
        y: f32,
        color: Color,
        size: f32,
        speed: f32,
        damage: f32,
        by: BulletBy,
    ) -> Bullet {
        Bullet {
            x,
            y,
            color,
            is_alive: true,
            size,
            speed,
            damage,
            by,
        }
    }

    pub fn hit(&mut self, enemy: Enemy) {
        if enemy.health >= self.damage {
            self.is_alive = false;
        }
    }

    pub fn is_hit(&mut self, enemy: Enemy) -> bool {
        let bullet = self;
        if bullet.x < enemy.x + enemy.size
            && bullet.x + bullet.size > enemy.x
            && bullet.y < enemy.y + enemy.size
            && bullet.y + bullet.size > enemy.y
        {
            return true;
        }
        return false;
    }

    pub fn draw(&mut self, _texture: &Texture2D) {
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
    }
}

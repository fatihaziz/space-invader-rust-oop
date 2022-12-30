use crate::logic::GameObject;

use macroquad::{prelude::*, rand::gen_range};

#[derive(Copy, Clone)]
pub struct Buff {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub is_alive: bool,
    pub size: f32,
    pub speed: f32,
    pub apply_effect: fn(&Buff, &mut GameObject),
}
impl Buff {
    pub fn choose_effect(&mut self, game: &mut GameObject) {
        let rand_num = gen_range(0, 10);
        match rand_num {
            1 => {
                fn speed_effect(buff: &Buff, game: &mut GameObject) {
                    game.ship.unwrap().speed += 1.0;
                }
                self.apply_effect = speed_effect;
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, _texture: &Texture2D) {}
}

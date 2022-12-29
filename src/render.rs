use macroquad::prelude::*;

use crate::logic::GameObject;

pub fn bg_render(txt2d: &Texture2D) {
    draw_texture(*txt2d, 0.0, 0.0, WHITE)
}

pub fn score_on_play_render(score: f32) {
    draw_text(&format!("Score: {}", score), 10.0, 30.0, 24.0, WHITE);
}

pub fn game_render(game: &mut GameObject) {
    bg_render(&game.texture.bg);

    //draw ship
    let ship = &mut game.ship.unwrap();
    {
        ship.draw(&game.texture.ship);
        game.ship = Some(ship.clone());
    }

    // draw bullet
    if let Some(bullet_vec) = &mut game.bullet_vec {
        for bullet in bullet_vec {
            bullet.draw(&game.texture.bullet);
        }
    }

    score_on_play_render(game.score);
}

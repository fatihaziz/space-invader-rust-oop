use macroquad::prelude::*;

use crate::{logic::GameObject, WINDOW_WIDTH};

pub fn bg_render(txt2d: &Texture2D) {
    draw_texture(*txt2d, 0.0, 0.0, WHITE)
}

pub fn score_on_play_render(score: f32) {
    draw_text(&format!("Score: {}", score), 10.0, 30.0, 24.0, WHITE);
}
pub fn render_start_screen() {
    draw_text(
        "Press Space to Start",
        (WINDOW_WIDTH / 2.0) - 200.0,
        250.0,
        50.0,
        WHITE,
    );
    draw_text(
        "Press Esc to Exit",
        (WINDOW_WIDTH / 2.0) - 100.0,
        300.0,
        24.0,
        WHITE,
    );
}

pub fn render_game_over_screen(game: &mut GameObject) {
    draw_text(
        "Game Over",
        (WINDOW_WIDTH / 2.0) - 100.0,
        250.0,
        50.0,
        WHITE,
    );
    draw_text("Score: ", (WINDOW_WIDTH / 2.0) - 100.0, 300.0, 24.0, WHITE);
    draw_text(
        game.score.to_string().as_str(),
        (WINDOW_WIDTH / 2.0) - 20.0,
        300.0,
        24.0,
        GREEN,
    );
    draw_text(
        "Press Space to Restart",
        (WINDOW_WIDTH / 2.0) - 100.0,
        350.0,
        24.0,
        WHITE,
    );
    draw_text(
        "Press Esc to Exit",
        (WINDOW_WIDTH / 2.0) - 100.0,
        500.0,
        24.0,
        WHITE,
    );
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
    if let Some(bullet_vec) = &mut game.bullets_vec {
        for bullet in bullet_vec {
            bullet.draw(&game.texture.bullet);
        }
    }

    // draw enemies
    if let Some(enemy_vec) = &mut game.enemies_vec {
        for enemy in enemy_vec {
            enemy.draw(&game.texture.enemy);
        }
    }

    score_on_play_render(game.score);
}

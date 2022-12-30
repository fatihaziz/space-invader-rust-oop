use macroquad::prelude::*;

use crate::{logic::GameObject, WINDOW_WIDTH};

pub struct GameTexture {
    pub bg: Texture2D,
    pub ship: Texture2D,
    pub enemy: Texture2D,
    pub bullet: Texture2D,
}
impl GameTexture {
    pub fn load() -> GameTexture {
        GameTexture {
            bg: (Texture2D::from_file_with_format(include_bytes!("../assets/bg.png"), None)),
            ship: (Texture2D::from_file_with_format(include_bytes!("../assets/ship.png"), None)),
            enemy: (Texture2D::from_file_with_format(include_bytes!("../assets/enemy.png"), None)),
            bullet: (Texture2D::from_file_with_format(
                include_bytes!("../assets/bullet.png"),
                None,
            )),
        }
    }
}

pub fn bg_render(txt2d: &Texture2D) {
    draw_texture(*txt2d, 0.0, 0.0, WHITE)
}

pub fn gample_overlay_render(game: &mut GameObject) {
    draw_text(&format!("Score: {}", game.score), 10.0, 30.0, 24.0, WHITE);
    draw_text(&format!("Waves: {}", game.waves), 10.0, 60.0, 24.0, WHITE);
}
pub fn start_screen_render() {
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

pub fn game_over_screen_render(game: &mut GameObject) {
    draw_text(
        "Game Over",
        (WINDOW_WIDTH / 2.0) - 100.0,
        250.0,
        50.0,
        WHITE,
    );
    draw_text(
        &format!("Score: {}", game.score),
        (WINDOW_WIDTH / 2.0) - 100.0,
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

    gample_overlay_render(game);
}

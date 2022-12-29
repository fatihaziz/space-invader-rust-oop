use logic::{game_logic, new_game, GameState};
// also add 'tokio-js-set-interval = "<latest-version>"' to your Cargo.toml!
use macroquad::prelude::*;
use render::{game_render, render_game_over_screen, render_start_screen};

mod entities;
mod logic;
mod render;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

fn windows_conf() -> Conf {
    Conf {
        window_title: "Space Invader".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

fn limit_fps(max_fps: f64) {
    let frame_time = 1.0 / max_fps;
    let start_time = macroquad::time::get_time();
    let end_time = start_time + frame_time;
    while macroquad::time::get_time() < end_time {
        // do nothing
    }
}

#[macroquad::main(windows_conf)]
async fn main() {
    let mut game = new_game().await;

    let mut time_exit: Option<f32> = None;
    loop {
        limit_fps(500.0);

        if is_key_pressed(KeyCode::Space) {
            if game.state == GameState::CREATED {
                game.state = GameState::PLAYING;
            } else if game.state == GameState::GAMEOVER {
                game = new_game().await;
                game.state = GameState::PLAYING;
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            if game.state == GameState::PLAYING {
                game.state = GameState::GAMEOVER;
            } else if game.state == GameState::GAMEOVER {
                game.state = GameState::EXIT;
            }
        }
        match game.state {
            GameState::CREATED => render_start_screen(),

            GameState::PLAYING => {
                game_logic(&mut game);
                game_render(&mut game);
            }

            GameState::GAMEOVER => render_game_over_screen(&mut game),

            GameState::EXIT => {
                clear_background(BLACK);
                draw_text(
                    "Thanks for Playing!",
                    (WINDOW_WIDTH / 2.0) - 200.0,
                    250.0,
                    50.0,
                    WHITE,
                );

                if time_exit.is_none() {
                    time_exit = Some(macroquad::time::get_time() as f32);
                } else {
                    if (macroquad::time::get_time() as f32 - time_exit.unwrap()) > 1.0 {
                        break;
                    }
                }
            }
        }

        // for debugging
        {
            // show fps on top left corner, size 12
            draw_text(&format!("FPS: {}", get_fps()), 10.0, 10.0, 12.0, WHITE);
        }

        next_frame().await;
    }
}

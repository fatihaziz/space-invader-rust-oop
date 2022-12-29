// also add 'tokio-js-set-interval = "<latest-version>"' to your Cargo.toml!
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

#[derive(Debug, Eq, PartialEq)]
pub enum GameState {
    CREATED,
    PLAYING,
    PAUSED,
    GAMEOVER,
}

#[derive(Clone, Copy)]
pub struct Ship {
    pub is_alive: bool,
    pub size: f32,
    pub x: f32,
    pub y: f32,
    pub velocity: f32,
}
pub struct GameObject {
    pub state: GameState,
    pub ship_vec: Arc<Mutex<Vec<Ship>>>,
}

// create rust to detect keypress

fn keypress() -> String {
    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(pressed),
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
            kind: KeyEventKind::Press,
        }) = read().unwrap()
        {
            return pressed.to_string();
        }
    }
}

pub const INIT_WIDTH: f32 = 800.0;
pub const INIT_HEIGHT: f32 = 600.0;

async fn new_game(ship_velocity: f32) -> Arc<Mutex<GameObject>> {
    let game_object = Arc::new(Mutex::new(GameObject {
        state: GameState::CREATED,
        ship_vec: Arc::new(Mutex::new(Vec::new())),
    }));

    // create thread to detect keypress
    let ship_vec = game_object.lock().unwrap().ship_vec.clone();
    if ship_vec.lock().unwrap().last().is_none() {
        // init ship vec
        ship_vec.lock().unwrap().push(Ship {
            is_alive: true,
            size: 24.0,
            x: 0.0,
            y: 0.0,
            velocity: ship_velocity,
        });
    }
    return game_object;
}

async fn game_logic(game_object: &mut Arc<Mutex<GameObject>>) {
    loop {
        let ship_vec = game_object.lock().unwrap().ship_vec.clone();
        let mut ship = *ship_vec.lock().unwrap().last().unwrap();
        let keypress = keypress();
        if (keypress.is_empty()) {
            continue;
        }
        match keypress.as_str() {
            "w" => {
                ship.y += ship.velocity;
            }
            "a" => {
                ship.x -= ship.velocity;
            }
            "s" => {
                ship.y -= ship.velocity;
            }
            "d" => {
                ship.x += ship.velocity;
            }
            _ => {
                println!("other key pressed!");
            }
        }

        // print ship position
        ship_vec.lock().unwrap().push(Ship {
            is_alive: ship.is_alive,
            size: ship.size,
            x: ship.x,
            y: ship.y,
            velocity: ship.velocity,
        });
        ship_vec.lock().unwrap().remove(0);
        let game_object_arc = game_object.clone();
        game_object_arc.lock().unwrap().state = GameState::PLAYING;
        println!(
            "print pos: x: {}, y: {}, game state: {:?}",
            ship.x,
            ship.y,
            game_object_arc.lock().unwrap().state
        );
    }
}

async fn render_with_macroquad(game_object: &mut Arc<Mutex<GameObject>>) {
    loop {
        clear_background(BLACK);

        // draw ship in window center
        // let ship = *ship_vec.lock().unwrap().last().unwrap();

        // draw_rectangle(
        //     INIT_WIDTH / 2.0 + ship.x,
        //     INIT_HEIGHT / 2.0 + ship.y,
        //     ship.size,
        //     ship.size,
        //     GREEN,
        // );

        next_frame().await;
    }
}

#[macroquad::main("Space Invader")]
async fn main() {
    loop {
        let mut game_object = new_game(1.0).await;
        let mut game_object_arc = game_object.clone();
        let game_logic_handle = tokio::spawn(async move {
            game_logic(&mut game_object_arc).await;
        });
        let mut game_object_arc = game_object.clone();
        let render_handle = tokio::spawn(async move {
            render_with_macroquad(&mut game_object_arc).await;
        });
        game_logic_handle.await.unwrap();
        render_handle.await.unwrap();
    }
}

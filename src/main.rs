// also add 'tokio-js-set-interval = "<latest-version>"' to your Cargo.toml!

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

#[derive(Debug)]
pub enum GameState {
    CREATED,
    PLAYING,
    PAUSED,
}

#[derive(Clone, Copy)]
pub struct Ship {
    pub state: i32,
    pub size: i32,
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
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

#[tokio::main]
async fn main() {
    let game_object = Arc::new(Mutex::new(GameObject {
        state: GameState::CREATED,
        ship_vec: Arc::new(Mutex::new(Vec::new())),
    }));
    let ship_vec = game_object.lock().unwrap().ship_vec.clone();

    let mut handles = vec![];

    let game_object_arc = game_object.clone();
    handles.push(thread::spawn(move || loop {
        if ship_vec.lock().unwrap().last().is_none() {
            // init ship vec
            ship_vec.lock().unwrap().push(Ship {
                state: 1,
                size: 24,
                x1: 0,
                x2: 24,
                y1: 0,
                y2: 24,
            });
            continue;
        }
        // get last ship
        let mut ship = *ship_vec.lock().unwrap().last().unwrap();
        match keypress().as_str() {
            "w" => {
                ship.y1 += 1;
                ship.y2 += 1;
            }
            "a" => {
                ship.x1 -= 1;
                ship.x2 -= 1;
            }
            "s" => {
                ship.y1 -= 1;
                ship.y2 -= 1;
            }
            "d" => {
                ship.x1 += 1;
                ship.x2 += 1;
            }
            _ => {
                println!("other key pressed!");
            }
        }

        // print ship position
        ship_vec.lock().unwrap().push(Ship {
            state: ship.state,
            size: ship.size,
            x1: ship.x1,
            x2: ship.x2,
            y1: ship.y1,
            y2: ship.y2,
        });
        ship_vec.lock().unwrap().remove(0);
        game_object_arc.lock().unwrap().state = GameState::PLAYING;

        println!(
            "print pos realtime: x1: {}, x2: {}, y1: {}, y2: {}",
            ship.x1, ship.x2, ship.y1, ship.y2
        );
    }));

    let ship_vec = game_object.lock().unwrap().ship_vec.clone();
    handles.push(thread::spawn(move || loop {
        if ship_vec.lock().unwrap().last().is_none() {
            continue;
        }
        let ship = *ship_vec.lock().unwrap().last().unwrap();
        println!(
            "tick print pos delay 1s: x1: {}, x2: {}, y1: {}, y2: {}, game state: {:?}",
            ship.x1,
            ship.x2,
            ship.y1,
            ship.y2,
            game_object.lock().unwrap().state
        );
        thread::sleep(Duration::from_millis(1000));
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}

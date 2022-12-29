use macroquad::prelude::*;
use macroquad::texture::Texture2D;

use crate::{
    entities::{bullet::Bullet, enemy::Enemy, ship::Ship},
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

#[derive(Debug, Eq, PartialEq)]
pub enum GameState {
    CREATED,
    PLAYING,
    GAMEOVER,
    EXIT,
}
pub struct GameTexture {
    pub bg: Texture2D,
    pub ship: Texture2D,
    pub enemy: Texture2D,
    pub bullet: Texture2D,
}

pub struct GameObject {
    pub state: GameState,
    pub ship: Option<Ship>,
    pub enemies_vec: Option<Vec<Enemy>>,
    pub bullets_vec: Option<Vec<Bullet>>,
    pub score: f32,
    pub high_score: f32,
    pub texture: GameTexture,
}

pub async fn new_game() -> GameObject {
    let mut game_object = GameObject {
        state: GameState::CREATED,
        ship: None,
        bullets_vec: None,
        enemies_vec: None,
        score: 0.0,
        high_score: 0.0,
        texture: {
            GameTexture {
                bg: (Texture2D::from_file_with_format(include_bytes!("../assets/bg.png"), None)),
                ship: (Texture2D::from_file_with_format(
                    include_bytes!("../assets/ship.png"),
                    None,
                )),
                enemy: (Texture2D::from_file_with_format(
                    include_bytes!("../assets/enemy.png"),
                    None,
                )),
                bullet: (Texture2D::from_file_with_format(
                    include_bytes!("../assets/bullet.png"),
                    None,
                )),
            }
        },
    };

    game_object.ship = Some(Ship::new(
        WINDOW_WIDTH / 2.0,
        WINDOW_HEIGHT - 60.0,
        WHITE,
        true,
        50.0,
        1.0,
    ));

    return game_object;
}

pub fn game_logic(game: &mut GameObject) {
    let ship = &mut game.ship.unwrap();
    // user input
    {
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            ship.left();
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            ship.right();
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            ship.up();
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            ship.down();
        }

        // limiting ship position
        if ship.x < 0.0 {
            ship.x = 0.0;
        }
        if ship.x > WINDOW_WIDTH - ship.size {
            ship.x = WINDOW_WIDTH - ship.size;
        }
        if ship.y < 0.0 {
            ship.y = 0.0;
        }
        if ship.y > WINDOW_HEIGHT - ship.size {
            ship.y = WINDOW_HEIGHT - ship.size;
        }
    }

    // fire a bullet when hit space
    {
        let bullet = Bullet::new((ship.x + ship.size / 2.0) - 5.0, ship.y, WHITE, 10.0, 0.5);
        if is_key_pressed(KeyCode::Space) {
            if let Some(bullet_vec) = &mut game.bullets_vec {
                bullet_vec.push(bullet);
            } else {
                game.bullets_vec = Some(vec![bullet]);
            }
        }
    }

    // move the bullet towards up
    {
        if let Some(bullet_vec) = &mut game.bullets_vec {
            for bullet in bullet_vec {
                bullet.y -= bullet.speed;
            }
        }
        // if outside screen, remove it
        if let Some(bullet_vec) = &mut game.bullets_vec {
            bullet_vec.iter_mut().for_each(|bullet| {
                if bullet.y < 0.0 {
                    bullet.is_alive = false;
                }
            });
            bullet_vec.retain(|bullet| bullet.is_alive);
        }
    }

    // generate enemies, 6 enemies each row, move to right and down
    {
        let enemy_size = 50.0;
        let enemy_speed = 0.25;
        if let Some(enemies_vec) = &mut game.enemies_vec {
            enemies_vec.retain(|enemy| enemy.is_alive);

            for enemy in enemies_vec {
                // check if enemy is hit by bullet
                // not generated by copilot :(
                if let Some(bullet_vec) = &mut game.bullets_vec {
                    for bullet in bullet_vec {
                        if bullet.x < enemy.x + enemy.size
                            && bullet.x + bullet.size > enemy.x
                            && bullet.y < enemy.y + enemy.size
                            && bullet.y + bullet.size > enemy.y
                        {
                            enemy.kill();
                            bullet.hit();
                            game.score += enemy.score;
                        }
                    }
                }
                enemy.x += enemy.speed;
                if enemy.x > WINDOW_WIDTH - enemy.size {
                    enemy.x = WINDOW_WIDTH - enemy.size;
                    enemy.y += enemy.size;
                    enemy.speed *= -1.0;
                }
                if enemy.x < 0.0 {
                    enemy.x = 0.0;
                    enemy.y += enemy.size;
                    enemy.speed *= -1.0;
                }
            }
        } else {
            let mut enemies_vec = Vec::new();
            for i in 0..20 {
                for j in 0..4 {
                    enemies_vec.push(Enemy::new(
                        (i as f32 * enemy_size * 1.0) + enemy_size,
                        (j as f32 * enemy_size * 1.0) + enemy_size,
                        WHITE,
                        enemy_size,
                        enemy_speed,
                        1.0,
                    ));
                }
            }
            game.enemies_vec = Some(enemies_vec);
        }
    }

    // gameover, when no enemies left, or when an enemy hit ship, or when enemy goes window
    {
        if let Some(enemies_vec) = &mut game.enemies_vec {
            if enemies_vec.len() == 0 {
                game.state = GameState::GAMEOVER;
            }
            for enemy in enemies_vec {
                if enemy.y > WINDOW_HEIGHT - enemy.size {
                    game.state = GameState::GAMEOVER;
                }
                if enemy.x < ship.x + ship.size
                    && enemy.x + enemy.size > ship.x
                    && enemy.y < ship.y + ship.size
                    && enemy.y + enemy.size > ship.y
                {
                    game.state = GameState::GAMEOVER;
                }
            }
        }
    }

    game.ship = Some(ship.clone());
}

pub mod bricks;
pub mod config;
pub mod game;
pub mod record;

use crate::abstractions::{Keycode, Screen};

use super::game::{Game, GameContext};
use config::{TETRIS_HEIGHT, TETRIS_WIDTH};
use game::{Tetris as TetrisGame, Unit};

pub struct Tetris {
    tetris_game: TetrisGame,
}

impl Game for Tetris {
    fn create() -> Self {
        let mut tetris_game = TetrisGame::new();
        tetris_game.start();
        Self { tetris_game }
    }

    fn logic_tick(&mut self, _ctx: &mut GameContext) {
        self.tetris_game.update();
    }

    fn render_tick(&mut self, ctx: &mut GameContext) {
        for key in ctx.key_buffer {
            match key {
                Keycode::KC_G => {
                    self.tetris_game.event_left();
                }
                Keycode::KC_D => {
                    self.tetris_game.event_right();
                }
                Keycode::KC_F => {
                    self.tetris_game.event_rotate();
                }
                Keycode::KC_ENTER => {
                    self.tetris_game.event_sink();
                }
                _ => {}
            }
        }

        let pixel_size = 4;

        let shadow = self.tetris_game.get_shadow();
        let abs = self.tetris_game.get_absolute();
        for y in 0..TETRIS_HEIGHT {
            for x in 0..TETRIS_WIDTH {
                let Unit(on) = &self.tetris_game.board.datas[y][x];
                // if y == 0 && *on {
                //     reset = true;
                // }
                for dy in 0..pixel_size {
                    for dx in 0..pixel_size {
                        Screen::set_pixel(
                            (x * pixel_size + dx) as u8,
                            (y * pixel_size + dy) as u8,
                            *on,
                        );
                    }
                }
            }
        }

        for &(x, y) in &abs {
            if y >= 0 {
                for dy in 0..pixel_size {
                    for dx in 0..pixel_size {
                        Screen::set_pixel(
                            (x as usize * pixel_size + dx) as u8,
                            (y as usize * pixel_size + dy) as u8,
                            true,
                        );
                    }
                }
            }
        }

        for &(x, y) in &shadow {
            let coords = (x as isize, y as isize);
            if abs.contains(&coords) {
                continue;
            }
            if y >= 0 {
                for dy in 0..pixel_size {
                    for dx in 0..pixel_size {
                        let checkerboard = (dx + dy) % 2 == 0; // alternate between true/false
                        Screen::set_pixel(
                            (x as usize * pixel_size + dx) as u8,
                            (y as usize * pixel_size + dy) as u8,
                            checkerboard,
                        );
                    }
                }
            }
        }
    }

    fn logic_delay(&self) -> u8 {
        6
    }

    fn destroy(&mut self) {}
}

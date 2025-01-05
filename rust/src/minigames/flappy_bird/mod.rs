mod bird;
mod pipe;

use alloc::{format, string::ToString};
use bird::Bird;
use pipe::Pipe;

use super::game::{Game, GameContext};
use crate::abstractions::{Keycode, Screen};

pub struct FlappyBird {
    bird: Bird,
    pipes: [Pipe; 3],
    tick: u32,
    score: u16,
    deaths: u16,
    started: bool,
}

impl FlappyBird {
    const BIRD_SIZE: u32 = 4;
    const PIPE_WIDTH: u8 = 8;
    const PIPE_HEIGHT: u8 = 128;
    const PIPE_GAP: u8 = 28;

    fn reset(&mut self) {
        self.bird = Bird::new();
        self.pipes = [Pipe::new(), Pipe::new(), Pipe::new()];
        self.tick = (Self::PIPE_WIDTH / 2) as u32 + 4;
        self.score = 0;
        self.deaths += 1;
        self.deaths = self.deaths.min(99);
        self.started = false;
    }
}

impl Game for FlappyBird {
    fn create() -> Self {
        Self {
            bird: Bird::new(),
            pipes: [Pipe::new(), Pipe::new(), Pipe::new()],
            tick: (Self::PIPE_WIDTH / 2) as u32 + 4,
            score: 0,
            deaths: 0,
            started: false,
        }
    }

    fn logic_tick(&mut self, ctx: &mut GameContext) {
        for key in ctx.key_buffer {
            match key {
                Keycode::KC_ENTER => {
                    if self.started {
                        self.bird.flap();
                    }
                    self.started = true;
                }

                _ => {}
            }
        }

        if !self.started {
            return;
        };

        self.tick += 1;

        self.bird.gravity();

        self.bird.tick();

        let wrapped = self.tick % Self::PIPE_GAP as u32 == 0;

        let should_score = self.tick % Self::PIPE_GAP as u32 == (Self::PIPE_GAP as u32 / 2) + 8;

        if should_score {
            self.score += 1;
            self.score = self.score.min(999);
        }

        if wrapped {
            self.pipes.rotate_left(1);
            self.pipes[2] = Pipe::new();
        }

        if self.bird.game_over(&self.pipes, self.tick) {
            self.reset();
        }
    }

    fn render_tick(&mut self, _: &mut GameContext) {
        Screen::clear(false);

        let screen_bird_y = self.bird.get_pos() as u32;
        let screen_bird_x = 4;

        for (i, pipe) in self.pipes.iter().enumerate() {
            for x in 0..Self::PIPE_WIDTH {
                for y in 0..Self::PIPE_HEIGHT {
                    let x =
                        (x + i as u8 * Self::PIPE_GAP) - (self.tick % Self::PIPE_GAP as u32) as u8;
                    let on = pipe.is_dead(y);

                    Screen::set_pixel(x, y, on);
                }
            }
        }

        for dx in 0..Self::BIRD_SIZE {
            for dy in 0..Self::BIRD_SIZE {
                Screen::set_pixel((dx + screen_bird_x) as u8, (dy + screen_bird_y) as u8, true);
            }
        }

        let string1 = self.deaths.to_string();
        let string2 = self.score.to_string();
        let spaces_needed = 5 - (string1.len() + string2.len());
        let result = format!("{}{}{}", string1, " ".repeat(spaces_needed.max(0)), string2);

        Screen::draw_text_inverted(&result, false);
    }
}

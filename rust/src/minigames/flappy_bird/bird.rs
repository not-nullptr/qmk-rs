use crate::abstractions::Screen;

use super::{pipe::Pipe, FlappyBird};

pub struct Bird {
    y: i32,
    vel: i32,
}

impl Bird {
    pub fn new() -> Self {
        Self { y: 620, vel: 0 }
    }

    pub fn flap(&mut self) {
        self.vel = -35;
    }

    pub fn gravity(&mut self) {
        self.vel += 5;
    }

    pub fn tick(&mut self) {
        self.y += self.vel;
        self.y = self.y.max(0);
    }

    pub fn game_over(&mut self, pipes: &[Pipe; 3], tick: u32) -> bool {
        let bird_x = 4;
        let bird_y = self.get_pos();
        let bird_width = FlappyBird::BIRD_SIZE;
        let bird_height = FlappyBird::BIRD_SIZE;

        if self.y >= (Screen::SCREEN_HEIGHT as i32) * 10 || bird_y < 0 {
            return true;
        }

        for (i, pipe) in pipes.iter().enumerate() {
            let pipe_offset =
                (i as u8 * FlappyBird::PIPE_GAP) - (tick % FlappyBird::PIPE_GAP as u32) as u8;
            let pipe_start_x = pipe_offset;
            let pipe_end_x = FlappyBird::PIPE_WIDTH + pipe_offset;

            if pipe_start_x <= (bird_x + bird_width as u8) && bird_x <= pipe_end_x {
                if pipe.is_dead(bird_y as u8) || pipe.is_dead((bird_y + bird_height as i32) as u8) {
                    return true;
                }
            }
        }

        false
    }
    pub fn get_pos(&mut self) -> i32 {
        self.y / 10
    }
}

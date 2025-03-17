use crate::{
    animation::{AngularFrequency, DampingRatio, DeltaTime, Spring, fps},
    page::{Page, RenderInfo},
    state::InputEvent,
};
use alloc::boxed::Box;
use qmk::{rgb::RGBLight, screen::Screen};

use super::HomePage;

pub struct HelloWorldPage {
    spring: Spring,
    pos: (f32, f32),
    vel: (f32, f32),
    cursor: (u8, u8),
    target: (f32, f32),
}

impl Default for HelloWorldPage {
    fn default() -> Self {
        Self {
            spring: Spring::new(DeltaTime(fps(15)), AngularFrequency(6.0), DampingRatio(0.7)),
            pos: (0.0, 0.0),
            vel: (0.0, 0.0),
            cursor: (0, 0),
            target: (0.0, 0.0),
        }
    }
}

impl Page for HelloWorldPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        while let Some(event) = renderer.input.poll() {
            match event {
                InputEvent::EncoderClick(i) => {
                    if i == 0 {
                        self.target = (self.cursor.0 as f32, self.cursor.1 as f32);
                    } else {
                        return Some(Box::new(HomePage::default()));
                    }
                }

                InputEvent::EncoderScroll(index, clockwise) => {
                    if clockwise {
                        if index == 0 {
                            self.cursor.0 += 1;
                        } else {
                            self.cursor.1 += 1;
                        }
                    } else {
                        if index == 0 {
                            self.cursor.0 -= 1;
                        } else {
                            self.cursor.1 -= 1;
                        }
                    }
                }
            }
        }

        let (pos_x, vel_x) = self.spring.update(self.pos.0, self.vel.0, self.target.0);
        let (pos_y, vel_y) = self.spring.update(self.pos.1, self.vel.1, self.target.1);
        self.pos = (pos_x, pos_y);
        self.vel = (vel_x, vel_y);
        renderer
            .framebuffer
            .fill_rect(self.pos.0 - 16.0, self.pos.1 - 16.0, 32, 32);
        renderer
            .framebuffer
            .fill_rect(0, self.cursor.1, Screen::OLED_DISPLAY_WIDTH as u8, 2);
        renderer
            .framebuffer
            .fill_rect(self.cursor.0, 0, 2, Screen::OLED_DISPLAY_HEIGHT as u8);

        None
    }
}

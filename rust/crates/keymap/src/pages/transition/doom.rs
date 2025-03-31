use core::cell::RefCell;

use alloc::boxed::Box;
use critical_section::{Mutex, with};
use qmk::{framebuffer::Framebuffer, screen::Screen};

use crate::{
    page::Page,
    state::{InputHandler, PAGE},
};

use super::TransitionHandler;

#[derive(Clone, Copy)]
struct Column {
    until: u8,
    y: u8,
}

impl Column {
    fn from_prev(last: &Column, rand: u8) -> Self {
        let rand = (rand % 3) as i32 - 1;
        let until = (last.until as i32 + rand).clamp(0, u8::MAX as i32) as u8;
        Self { until, y: 0 }
    }

    fn first() -> Self {
        Self { until: 0, y: 0 }
    }

    fn tick(&mut self) {
        self.until = self.until.saturating_sub(1);
        if self.until == 0 {
            self.y = self.y.saturating_add(6);
        }
    }
}

const COLUMN_WIDTH: usize = 1;

pub struct DoomTransition {
    to: Box<dyn Page>,
    columns: [Column; Screen::OLED_DISPLAY_WIDTH / COLUMN_WIDTH],
}

struct DoomRNG {
    rng_index: usize,
}

impl DoomRNG {
    const RNG_TABLE: [u8; 256] = [
        0, 8, 109, 220, 222, 241, 149, 107, 75, 248, 254, 140, 16, 66, 74, 21, 211, 47, 80, 242,
        154, 27, 205, 128, 161, 89, 77, 36, 95, 110, 85, 48, 212, 140, 211, 249, 22, 79, 200, 50,
        28, 188, 52, 140, 202, 120, 68, 145, 62, 70, 184, 190, 91, 197, 152, 224, 149, 104, 25,
        178, 252, 182, 202, 182, 141, 197, 4, 81, 181, 242, 145, 42, 39, 227, 156, 198, 225, 193,
        219, 93, 122, 175, 249, 0, 175, 143, 70, 239, 46, 246, 163, 53, 163, 109, 168, 135, 2, 235,
        25, 92, 20, 145, 138, 77, 69, 166, 78, 176, 173, 212, 166, 113, 94, 161, 41, 50, 239, 49,
        111, 164, 70, 60, 2, 37, 171, 75, 136, 156, 11, 56, 42, 146, 138, 229, 73, 146, 77, 61, 98,
        196, 135, 106, 63, 197, 195, 86, 96, 203, 113, 101, 170, 247, 181, 113, 80, 250, 108, 7,
        255, 237, 129, 226, 79, 107, 112, 166, 103, 241, 24, 223, 239, 120, 198, 58, 60, 82, 128,
        3, 184, 66, 143, 224, 145, 224, 81, 206, 163, 45, 63, 90, 168, 114, 59, 33, 159, 95, 28,
        139, 123, 98, 125, 196, 15, 70, 194, 253, 54, 14, 109, 226, 71, 17, 161, 93, 186, 87, 244,
        138, 20, 52, 123, 251, 26, 36, 17, 46, 52, 231, 232, 76, 31, 221, 84, 37, 216, 165, 212,
        106, 197, 242, 98, 43, 39, 175, 254, 145, 190, 84, 118, 222, 187, 136, 120, 163, 236, 249,
    ];

    const fn new() -> Self {
        Self { rng_index: 0 }
    }

    fn random(&mut self) -> u8 {
        self.rng_index = (self.rng_index + 1) & 0xFF;
        Self::RNG_TABLE[self.rng_index]
    }
}

static DOOM_RNG: Mutex<RefCell<DoomRNG>> = Mutex::new(RefCell::new(DoomRNG::new()));

impl TransitionHandler for DoomTransition {
    fn new(to: Box<dyn Page>) -> Self
    where
        Self: Sized,
    {
        let columns = with(|cs| {
            let mut rng = DOOM_RNG.borrow_ref_mut(cs);
            let mut columns = [Column::first(); Screen::OLED_DISPLAY_WIDTH / COLUMN_WIDTH];
            for i in 0..Screen::OLED_DISPLAY_WIDTH / COLUMN_WIDTH {
                if i == 0 {
                    continue;
                }

                columns[i] = Column::from_prev(&columns[i - 1], rng.random());
            }

            for _ in 0..rng.random() {
                rng.random();
            }

            columns
        });

        Self { to, columns }
    }

    fn render(&mut self, renderer: &mut crate::page::RenderInfo) -> bool {
        let mut from = PAGE.borrow_ref_mut(renderer.cs);
        let mut from_framebuffer = Framebuffer::default();
        let mut from_renderer = crate::page::RenderInfo {
            framebuffer: &mut from_framebuffer,
            cs: renderer.cs,
            tick: renderer.tick,
            input: &mut InputHandler::new(),
            actions: renderer.actions,
        };
        from.render(&mut from_renderer);
        drop(from);

        if self
            .columns
            .iter()
            .any(|c| (c.y as usize) < Screen::OLED_DISPLAY_HEIGHT / 4)
        {
            while renderer.input.poll().is_some() {}
        }

        self.to.render(renderer);

        for column in self.columns.iter_mut() {
            column.tick();
        }

        for (i, column) in self.columns.iter().enumerate() {
            let x = i * COLUMN_WIDTH;
            let y = column.y;
            let width = COLUMN_WIDTH;
            let height = Screen::OLED_DISPLAY_HEIGHT;

            let slice = from_framebuffer.get_framebuffer_at(x, 0, width, height);
            renderer
                .framebuffer
                .draw_framebuffer_at(x, y, width, height, &slice);
        }

        self.columns
            .iter()
            .all(|column| column.y as usize >= Screen::OLED_DISPLAY_HEIGHT)
    }

    fn take_page(self: Box<Self>) -> Box<dyn Page> {
        self.to
    }
}

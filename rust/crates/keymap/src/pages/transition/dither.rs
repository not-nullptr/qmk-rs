use super::TransitionHandler;
use crate::{
    page::{Page, RenderInfo},
    state::PAGE,
};
use alloc::boxed::Box;
use qmk::{framebuffer::Framebuffer, screen::Screen};

const BAYER_MATRIX: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

pub struct DitherTransition {
    to: Box<dyn Page>,
    progress: u8,
}

impl TransitionHandler for DitherTransition {
    fn new(to: Box<dyn Page>) -> Self
    where
        Self: Sized,
    {
        Self { to, progress: 0 }
    }

    fn render(&mut self, renderer: &mut RenderInfo) -> bool {
        if self.progress == 7 {
            return true;
        }

        while renderer.input.poll().is_some() {}

        let mut from = PAGE.borrow_ref_mut(renderer.cs);
        from.render(renderer);
        drop(from);

        let mut to_framebuffer = Framebuffer::default();
        let mut to_renderer = RenderInfo {
            cs: renderer.cs,
            framebuffer: &mut to_framebuffer,
            input: renderer.input,
            tick: renderer.tick,
            actions: renderer.actions,
        };

        self.to.render(&mut to_renderer);

        for y in 0..Screen::OLED_DISPLAY_HEIGHT {
            for x in 0..Screen::OLED_DISPLAY_WIDTH {
                let bayer_x = x % 4;
                let bayer_y = y % 4;
                let bayer_value = BAYER_MATRIX[bayer_y][bayer_x];
                let threshold = self.progress * 2;

                if bayer_value > threshold {
                    continue;
                }

                let pixel = to_framebuffer.get_pixel(x, y);

                if pixel {
                    renderer.framebuffer.draw_pixel(x, y);
                } else {
                    renderer.framebuffer.clear_pixel(x, y);
                }
            }
        }

        self.progress += 1;
        false
    }

    fn take_page(self: Box<Self>) -> Box<dyn Page> {
        self.to
    }
}

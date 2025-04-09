use super::TransitionHandler;
use crate::{
    page::{Page, RenderInfo},
    state::PAGE,
};
use alloc::boxed::Box;
#[cfg(not(test))]
#[cfg(not(target_arch = "wasm32"))]
use micromath::F32Ext;
use qmk::{framebuffer::Framebuffer, screen::Screen};

const BAYER_MATRIX: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

fn ease_out_expo_extreme(x: f32) -> f32 {
    if x == 1.0 {
        return 1.0;
    }
    1.0 - (2.0f32).powf(-5.0 * x)
}

pub struct ScaleTransition {
    to: Box<dyn Page>,
    progress: u8,
}

impl TransitionHandler for ScaleTransition {
    fn new(to: Box<dyn Page>) -> Self
    where
        Self: Sized,
    {
        Self { to, progress: 0 }
    }

    fn render(&mut self, renderer: &mut RenderInfo) -> bool {
        if self.progress == 9 {
            return true;
        }

        while renderer.input.poll().is_some() {}

        if self.progress < 7 {
            let mut from = PAGE.borrow_ref_mut(renderer.cs);
            from.render(renderer);
            drop(from);

            let width = Screen::OLED_DISPLAY_WIDTH as f32
                * ease_out_expo_extreme((7 - self.progress) as f32 / 9.0);
            let height = Screen::OLED_DISPLAY_HEIGHT as f32
                * ease_out_expo_extreme((7 - self.progress) as f32 / 9.0);
            renderer.framebuffer.scale_around(32, 64, width, height);
        }

        if self.progress > 0 {
            let mut to_framebuffer = Framebuffer::default();
            let mut to_renderer = RenderInfo {
                cs: renderer.cs,
                framebuffer: &mut to_framebuffer,
                input: renderer.input,
                tick: renderer.tick,
                actions: renderer.actions,
            };

            self.to.render(&mut to_renderer);

            if self.progress < 7 {
                let width = Screen::OLED_DISPLAY_WIDTH as f32
                    * ease_out_expo_extreme(self.progress as f32 / 7.0);
                let height = Screen::OLED_DISPLAY_HEIGHT as f32
                    * ease_out_expo_extreme(self.progress as f32 / 7.0);
                to_framebuffer.scale_around(32, 64, width, height);
            }

            for y in 0..Screen::OLED_DISPLAY_HEIGHT {
                for x in 0..Screen::OLED_DISPLAY_WIDTH {
                    let bayer_x = x % 4;
                    let bayer_y = y % 4;
                    let bayer_value = BAYER_MATRIX[bayer_y][bayer_x];
                    let threshold = (self.progress * 2) + 1;

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
        }

        self.progress += 1;
        false
    }

    fn take_page(self: Box<Self>) -> Box<dyn Page> {
        self.to
    }

    fn page(&mut self) -> &mut Box<dyn Page> {
        &mut self.to
    }
}

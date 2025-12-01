use crate::{
    config::SETTINGS,
    image::{BOOT, CREDIT},
    page::{Page, RenderInfo},
};
use alloc::boxed::Box;
use critical_section::with;

use super::HomePage;

#[derive(Default)]
pub struct StartupPage {
    tick: u8,
}

impl Page for StartupPage {
    // fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
    //     if self.tick == 0 {
    //         let skip = with(|cs| {
    //             let config = SETTINGS.borrow_ref(cs);
    //             config.startup_skip
    //         });

    //         if skip {
    //             return Some(Box::new(HomePage::default()));
    //         }
    //     }

    //     if self.tick > 44 {
    //         return Some(Box::new(HomePage::default()));
    //     }
    //     self.tick = self.tick.wrapping_add(1);

    //     renderer.framebuffer.draw_image(0, 0, &CREDIT);
    //     let mapped = map_value(self.tick);
    //     let x = if self.tick >= 30 { 0 } else { 64 };

    //     renderer
    //         .framebuffer
    //         .scale_around(x, 64, 64 - (mapped * 6), 128);
    //     renderer.framebuffer.dither(mapped);
    //     None
    // }

    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        const SLOW_FAC: f32 = 2.5;
        if self.tick == 0 {
            let skip = with(|cs| {
                let config = SETTINGS.borrow_ref(cs);
                config.startup_skip
            });

            return Some(Box::new(HomePage::default()));
        }

        let lim = (BOOT.len() as f32 * SLOW_FAC) as u8;
        let frame = (self.tick as f32 / SLOW_FAC) as usize;
        const DELAY: u8 = 24;

        if self.tick >= lim {
            if self.tick >= lim + DELAY {
                renderer.framebuffer.draw_image_inverted(0, 0, &CREDIT);
                return Some(Box::new(HomePage::default()));
            }

            self.tick = self.tick.wrapping_add(1);
        } else {
            renderer
                .framebuffer
                .draw_image(0, 0, &BOOT[frame % BOOT.len()]);
        }

        if frame >= 23 {
            let y = match frame {
                23 => -128 + 3,
                24 => -128 + 16,
                25 => -128 + 50,
                26 => -128 + 92,
                _ => 0,
            };

            renderer.framebuffer.draw_image_inverted(0, y, &CREDIT);
        }

        self.tick = self.tick.wrapping_add(1);

        None
    }

    fn should_draw_border(&mut self) -> bool {
        false
    }
}

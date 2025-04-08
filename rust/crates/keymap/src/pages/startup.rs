use crate::{
    config::SETTINGS,
    image::CREDIT,
    page::{Page, RenderInfo},
};
use alloc::boxed::Box;
use critical_section::with;

use super::HomePage;

#[derive(Default)]
pub struct StartupPage {
    tick: u8,
}

fn map_value(n: u8) -> u8 {
    match n {
        0..=15 => 15 - n,
        16..=29 => 0,
        30..=44 => n - 30,
        _ => 15,
    }
}

impl Page for StartupPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        if self.tick == 0 {
            let skip = with(|cs| {
                let config = SETTINGS.borrow_ref(cs);
                config.startup_skip
            });

            if skip {
                return Some(Box::new(HomePage::default()));
            }
        }

        if self.tick > 44 {
            return Some(Box::new(HomePage::default()));
        }
        self.tick = self.tick.wrapping_add(1);
        renderer.framebuffer.draw_image(0, 0, &CREDIT);
        renderer.framebuffer.dither(map_value(self.tick));
        None
    }
}

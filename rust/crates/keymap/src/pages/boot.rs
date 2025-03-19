use core::sync::atomic::Ordering;

use crate::{
    image::WHY,
    page::{Page, RenderInfo},
    screen::{IS_TRANSITIONING, TRANSITION},
};
use alloc::boxed::Box;
use rp2040_hal::rom_data::reset_to_usb_boot;

#[derive(Default)]
pub struct BootPage {
    boot: bool,
}

impl Page for BootPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        renderer.framebuffer.draw_image(0, 0, &WHY);

        if self.boot {
            return None;
        }

        let is_transitioning = IS_TRANSITIONING.load(Ordering::SeqCst);
        if is_transitioning {
            return None;
        }

        self.boot = true;
        renderer.actions.push(Box::new(|| {
            reset_to_usb_boot(0, 0);
        }));

        None
    }
}

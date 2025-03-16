use alloc::boxed::Box;

use crate::{
    page::{Page, RenderInfo},
    state::InputEvent,
};

use super::HomePage;

#[derive(Default)]
pub struct RgbPage;

impl Page for RgbPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        while let Some(event) = renderer.input.poll() {
            match event {
                InputEvent::EncoderClick(i) => {
                    if i == 0 {
                        return Some(Box::new(HomePage::default()));
                    }
                }

                _ => {}
            }
        }

        renderer.framebuffer.draw_text_centered(8, "Colour", false);
        None
    }
}

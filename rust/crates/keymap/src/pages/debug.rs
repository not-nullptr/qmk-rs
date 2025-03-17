use super::HomePage;
use crate::{
    page::{Page, RenderInfo},
    state::InputEvent,
};
use alloc::{boxed::Box, format, string::ToString};

pub struct DebugPage {
    width: u8,
    height: u8,
}

impl Default for DebugPage {
    fn default() -> Self {
        Self {
            width: 64,
            height: 128,
        }
    }
}

impl Page for DebugPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        while let Some(event) = renderer.input.poll() {
            match event {
                InputEvent::EncoderClick(i) => {
                    if i == 0 {
                        return Some(Box::new(HomePage::default()));
                    }
                }

                InputEvent::EncoderScroll(i, clockwise) => {
                    if i != 0 {
                        continue;
                    }

                    if renderer.input.left_encoder_down() {
                        if clockwise {
                            self.width = self.width.saturating_add(4);
                        } else {
                            self.width = self.width.saturating_sub(4);
                        }
                    } else {
                        if clockwise {
                            self.height = self.height.saturating_add(4);
                        } else {
                            self.height = self.height.saturating_sub(4);
                        }
                    }
                }

                _ => {}
            }
        }

        renderer
            .framebuffer
            .draw_text_centered(32, 60, renderer.tick.to_string(), false);

        None
    }
}

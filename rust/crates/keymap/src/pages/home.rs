use super::{HelloWorldPage, RgbPage};
use crate::{
    page::{Page, RenderInfo},
    state::InputEvent,
};
use alloc::boxed::Box;

fn new_hello_world_page() -> Box<dyn Page> {
    Box::new(HelloWorldPage::default())
}

fn new_rgb_page() -> Box<dyn Page> {
    Box::new(RgbPage::default())
}

const PAGES: usize = 2;

type CreatePage = fn() -> Box<dyn Page>;
const OPTION_TEXT: [&str; PAGES] = ["Spring", "RGB"];
const OPTION_CONSTRUCTORS: [CreatePage; PAGES] = [new_hello_world_page, new_rgb_page];

#[derive(Default)]
pub struct HomePage {
    hovered: u8,
}

impl Page for HomePage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        while let Some(event) = renderer.input.poll() {
            match event {
                InputEvent::EncoderScroll(index, clockwise) => {
                    if index != 0 {
                        continue;
                    }

                    if clockwise {
                        self.hovered = (self.hovered + 1) % OPTION_TEXT.len() as u8;
                    } else {
                        self.hovered =
                            (self.hovered + OPTION_TEXT.len() as u8 - 1) % OPTION_TEXT.len() as u8;
                    }
                }

                InputEvent::EncoderClick(i) => {
                    if i == 0 {
                        return Some(OPTION_CONSTRUCTORS[self.hovered as usize]());
                    }
                }
            }
        }

        for (i, option) in OPTION_TEXT.iter().enumerate() {
            let y = (i as u8 * 11) + 4;
            let hovered = i == self.hovered as usize;
            if hovered {
                renderer.framebuffer.fill_rect(0, y, 64, 11);
            }

            renderer.framebuffer.draw_text(4, y + 2, *option, hovered);
        }

        None
    }
}

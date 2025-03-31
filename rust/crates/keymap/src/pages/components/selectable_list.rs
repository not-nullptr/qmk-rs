use crate::{page::RenderInfo, state::InputEvent};
use alloc::vec::Vec;
use qmk::screen::Screen;

pub struct ListConfig {
    pub x: u8,
    pub y: u8,
    pub item_width: u8,
    pub item_height: u8,
    pub gap: u8,
}

impl Default for ListConfig {
    fn default() -> Self {
        ListConfig {
            x: 0,
            y: 20,
            item_width: Screen::OLED_DISPLAY_WIDTH as u8,
            item_height: 11,
            gap: 0,
        }
    }
}

pub struct SelectableList {
    pub selected: usize,
    config: ListConfig,
}

impl SelectableList {
    pub fn new(config: ListConfig) -> Self {
        Self {
            config,
            selected: 0,
        }
    }

    pub fn render(
        &mut self,
        renderer: &mut RenderInfo,
        options: &[&str],
        events: &Vec<InputEvent>,
    ) -> Option<usize> {
        let mut should_return = false;
        for event in events {
            match event {
                InputEvent::EncoderScroll(index, clockwise) => {
                    if *index != 0 {
                        continue;
                    }

                    if *clockwise {
                        self.selected = (self.selected + 1) % options.len();
                    } else {
                        self.selected = (self.selected + options.len() - 1) % options.len();
                    }
                }

                InputEvent::EncoderClick(index) => {
                    if *index == 0 {
                        should_return = true;
                    }
                }
            }
        }
        for (i, option) in options.iter().enumerate() {
            let gap = self.config.item_height + self.config.gap;
            let y = self.config.y + (i as u8 * gap);
            let hovered = i == self.selected;
            if hovered {
                renderer.framebuffer.fill_rect(
                    self.config.x,
                    y,
                    self.config.item_width,
                    self.config.item_height,
                );
            }

            renderer
                .framebuffer
                .draw_text(self.config.x + 4, y + 2, *option, hovered);
        }

        if should_return {
            Some(self.selected)
        } else {
            None
        }
    }
}

use super::SettingsPage;
use crate::{
    page::{Page, RenderInfo},
    pages::{TRANSITION_TYPE, components::SelectableList},
};
use alloc::{boxed::Box, format, vec::Vec};
use core::sync::atomic::Ordering;

pub struct TransitionSettingsPage {
    list: SelectableList,
    list_items: &'static [&'static str],
}

impl Default for TransitionSettingsPage {
    fn default() -> Self {
        Self {
            list: SelectableList::new(Default::default()),
            list_items: &["Back", "Dither", "Scale", "Slide"],
        }
    }
}

impl Page for TransitionSettingsPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        let events = renderer.input.collect();
        if let Some(index) = self.list.render(renderer, &self.list_items, &events) {
            if index == 0 {
                return Some(Box::new(SettingsPage::default()));
            }

            TRANSITION_TYPE.store(index as u8 - 1, Ordering::SeqCst);
        }

        renderer.framebuffer.draw_text_centered(
            32,
            8,
            self.list_items[TRANSITION_TYPE.load(Ordering::SeqCst) as usize + 1],
            false,
        );

        None
    }
}

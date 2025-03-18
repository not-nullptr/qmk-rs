use super::SettingsPage;
use crate::{
    call_option, define_options,
    page::{Page, RenderInfo},
    pages::{TRANSITION_TYPE, components::SelectableList},
};
use alloc::boxed::Box;
use core::sync::atomic::Ordering;

pub struct TransitionSettingsPage {
    list: SelectableList,
}

define_options! {
    self => TransitionSettingsPage,
    "Back" => |_| Some(Box::new(SettingsPage::default())),
    "Dither" => |_| {
        TRANSITION_TYPE.store(0, Ordering::SeqCst);
        None
    },
    "Scale" => |_| {
        TRANSITION_TYPE.store(1, Ordering::SeqCst);
        None
    },
    "Slide" => |_| {
        TRANSITION_TYPE.store(2, Ordering::SeqCst);
        None
    },
}

impl Default for TransitionSettingsPage {
    fn default() -> Self {
        Self {
            list: SelectableList::new(Default::default()),
        }
    }
}

impl Page for TransitionSettingsPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        let events = renderer.input.collect();
        if let Some(index) = self.list.render(renderer, LIST_STRINGS, &events) {
            call_option!(index, self, LIST_CONSTRUCTORS);
        }

        renderer.framebuffer.draw_text_centered(
            32,
            8,
            LIST_STRINGS[TRANSITION_TYPE.load(Ordering::SeqCst) as usize + 1],
            false,
        );

        None
    }
}

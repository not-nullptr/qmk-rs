use super::SettingsPage;
use crate::{
    call_option,
    config::{PageTransition, SETTINGS},
    define_options,
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
    "Back", back => |_| Some(Box::new(SettingsPage::default())),
    "Dither", dither => |_| {
        TRANSITION_TYPE.store(PageTransition::Dither as u8, Ordering::SeqCst);
        None
    },
    "Scale", scale => |_| {
        TRANSITION_TYPE.store(PageTransition::Scale as u8, Ordering::SeqCst);
        None
    },
    "Slide", slide => |_| {
        TRANSITION_TYPE.store(PageTransition::Slide as u8, Ordering::SeqCst);
        None
    },
    "Doom", doom => |_| {
        TRANSITION_TYPE.store(PageTransition::Doom as u8, Ordering::SeqCst);
        None
    },
    "None", none => |_| {
        TRANSITION_TYPE.store(PageTransition::None as u8, Ordering::SeqCst);
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
            let mut settings = SETTINGS.borrow_ref_mut(renderer.cs);
            settings.transition = PageTransition::from_u8(TRANSITION_TYPE.load(Ordering::SeqCst));
            settings.save();
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

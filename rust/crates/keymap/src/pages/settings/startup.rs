use super::SettingsPage;
use crate::{
    call_option,
    config::SETTINGS,
    define_options,
    page::{Page, RenderInfo},
    pages::components::SelectableList,
};
use alloc::boxed::Box;
use critical_section::with;

pub struct StartupSettingsPage {
    list: SelectableList,
}

define_options! {
    self => StartupSettingsPage,
    "Back", back => |_| Some(Box::new(SettingsPage::default())),
    "Anim On", anim_on => |_| {
        with(|cs| {
            let mut settings = SETTINGS.borrow_ref_mut(cs);
            settings.startup_skip = false;
            settings.save();
        });
        None
    },
    "Anim Off", anim_off => |_| {
        with(|cs| {
            let mut settings = SETTINGS.borrow_ref_mut(cs);
            settings.startup_skip = true;
            settings.save();
        });
        None
    },
}

impl Default for StartupSettingsPage {
    fn default() -> Self {
        Self {
            list: SelectableList::new(Default::default()),
        }
    }
}

impl Page for StartupSettingsPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        let events = renderer.input.collect();
        if let Some(index) = self.list.render(renderer, LIST_STRINGS, &events) {
            call_option!(index, self, LIST_CONSTRUCTORS);
        }

        renderer
            .framebuffer
            .draw_text_centered(32, 8, "Startup", false);

        None
    }
}

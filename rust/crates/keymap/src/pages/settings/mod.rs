mod startup;
mod transition;

use super::{
    HomePage,
    components::{ListConfig, SelectableList},
};
use crate::{
    call_option, define_options,
    page::{Page, RenderInfo},
};
use alloc::boxed::Box;
use startup::StartupSettingsPage;
use transition::TransitionSettingsPage;

define_options! {
    "Back", back => |_| Some(HomePage::default()),
    "Anims", anims => |_| Some(TransitionSettingsPage::default()),
    "Startup", startup => |_| Some(StartupSettingsPage::default()),
}

pub struct SettingsPage {
    list: SelectableList,
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self {
            list: SelectableList::new(ListConfig::default()),
        }
    }
}

impl Page for SettingsPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        let events = renderer.input.collect();

        if let Some(index) = self.list.render(renderer, LIST_STRINGS, &events) {
            call_option!(index, renderer.actions, LIST_CONSTRUCTORS);
        }

        renderer
            .framebuffer
            .draw_text_centered(32, 8, "Settings", false);

        None
    }
}

mod transition;

use super::{
    HomePage,
    components::{ListConfig, SelectableList},
};
use crate::{
    define_options,
    page::{Page, RenderInfo},
    state::InputEvent,
};
use alloc::boxed::Box;
use qmk::screen::Screen;
use transition::TransitionSettingsPage;

define_options! {
    "Back" => HomePage::default; home,
    "Anims" => TransitionSettingsPage::default; transition
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

        if let Some(index) = self.list.render(renderer, OPTION_TEXT, &events) {
            return Some(OPTION_CONSTRUCTORS[index]());
        }

        renderer
            .framebuffer
            .draw_text_centered(8, "Settings", false);

        None
    }
}

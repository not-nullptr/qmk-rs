use super::{
    ColourPage, DebugPage, HelloWorldPage, SettingsPage,
    components::{ListConfig, SelectableList},
};
use crate::{
    define_options,
    page::{Page, RenderInfo},
};
use alloc::boxed::Box;

define_options! {
    "Spring" => HelloWorldPage::default; hello_world,
    "Colour" => ColourPage::default; colour,
    "Settings" => SettingsPage::default; settings,
    "Debug" => DebugPage::default; debug,
}

pub struct HomePage {
    list: SelectableList,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            list: SelectableList::new(ListConfig::default()),
        }
    }
}

impl Page for HomePage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        let events = renderer.input.collect();
        if let Some(index) = self.list.render(renderer, OPTION_TEXT, &events) {
            return Some(OPTION_CONSTRUCTORS[index]());
        }
        renderer
            .framebuffer
            .draw_text_centered(32, 8, "Home", false);
        None
    }
}

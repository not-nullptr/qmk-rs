use super::{
    HelloWorldPage, RgbPage, SettingsPage,
    components::{ListConfig, SelectableList},
};
use crate::{
    define_options,
    page::{Page, RenderInfo},
};
use alloc::boxed::Box;

define_options! {
    "Spring" => HelloWorldPage::default; hello_world,
    "Colour" => RgbPage::default; colour,
    "Settings" => SettingsPage::default; settings
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
        renderer.framebuffer.draw_text_centered(8, "Home", false);
        None
    }
}

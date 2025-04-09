use super::{
    ColourPage, DebugPage, HelloWorldPage, InfoPage, SettingsPage,
    components::{ListConfig, SelectableList},
};
use crate::{
    call_option, define_options,
    page::{Page, RenderInfo},
};
use alloc::boxed::Box;

define_options! {
    "Spring", spring => |_| Some(HelloWorldPage::default()),
    "Colour", colour => |_| Some(ColourPage::default()),
    "Settings", settings => |_| Some(SettingsPage::default()),
    "Info", info => |_| Some(InfoPage::default()),
    "Debug", debug => |_| Some(DebugPage::default()),
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
        if let Some(index) = self.list.render(renderer, LIST_STRINGS, &events) {
            call_option!(index, renderer.actions, LIST_CONSTRUCTORS);
        }
        renderer
            .framebuffer
            .draw_text_centered(32, 8, "Home", false);
        None
    }
}

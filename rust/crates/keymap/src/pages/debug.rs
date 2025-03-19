use super::{Actions, BootPage, HomePage, components::SelectableList};
use crate::{
    call_option, define_options,
    page::{Page, RenderInfo},
};
use alloc::boxed::Box;

define_options! {
    "Back", back => |_| Some(HomePage::default()),
    "USB Boot", boot => |_| Some(BootPage::default()),
}

pub struct DebugPage {
    list: SelectableList,
}

impl Default for DebugPage {
    fn default() -> Self {
        Self {
            list: SelectableList::new(Default::default()),
        }
    }
}

impl Page for DebugPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        let events = renderer.input.collect();
        if let Some(index) = self.list.render(renderer, LIST_STRINGS, &events) {
            call_option!(index, renderer.actions, LIST_CONSTRUCTORS);
        }

        renderer
            .framebuffer
            .draw_text_centered(32, 8, "Debug", false);

        None
    }
}

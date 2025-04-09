use alloc::{boxed::Box, vec::Vec};
use critical_section::CriticalSection;
use qmk::framebuffer::Framebuffer;

use crate::state::InputHandler;

pub trait Page: Send + Sync {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>>;
    #[allow(unused_variables)]
    fn init(&mut self, renderer: &mut RenderInfo) {}
    fn should_draw_border(&mut self) -> bool {
        true
    }
}

pub struct RenderInfo<'a> {
    pub framebuffer: &'a mut Framebuffer,
    pub cs: CriticalSection<'a>,
    pub tick: u32,
    pub input: &'a mut InputHandler,
    pub actions: &'a mut Vec<Box<dyn FnOnce()>>,
}

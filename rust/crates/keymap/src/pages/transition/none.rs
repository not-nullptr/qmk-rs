use super::TransitionHandler;
use crate::page::{Page, RenderInfo};
use alloc::boxed::Box;

pub struct NoneTransition {
    to: Box<dyn Page>,
}

impl TransitionHandler for NoneTransition {
    fn new(to: Box<dyn Page>) -> Self
    where
        Self: Sized,
    {
        Self { to }
    }

    fn render(&mut self, renderer: &mut RenderInfo) -> bool {
        renderer.framebuffer.clear();
        self.to.render(renderer);
        true
    }

    fn take_page(self: Box<Self>) -> Box<dyn Page> {
        self.to
    }
}

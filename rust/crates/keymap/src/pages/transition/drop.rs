use super::TransitionHandler;
use crate::{
    page::{Page, RenderInfo},
    state::PAGE,
};
use alloc::boxed::Box;
#[cfg(not(test))]
#[cfg(not(target_arch = "wasm32"))]
use micromath::F32Ext;
use qmk::framebuffer::Framebuffer;

pub struct SlideTransition {
    to: Box<dyn Page>,
    progress: u8,
}

impl TransitionHandler for SlideTransition {
    fn new(to: Box<dyn Page>) -> Self {
        Self { to, progress: 5 }
    }

    fn render(&mut self, renderer: &mut RenderInfo) -> bool {
        let mut from_framebuffer = Framebuffer::default();
        if self.progress >= 15 {
            return true;
        }
        // consume all input events while transitioning
        while renderer.input.poll().is_some() {}
        self.to.render(renderer);
        let mut from_renderer = RenderInfo {
            framebuffer: &mut from_framebuffer,
            cs: renderer.cs,
            tick: renderer.tick,
            input: renderer.input,
            actions: renderer.actions,
        };
        let mut from = PAGE.borrow_ref_mut(renderer.cs);
        from.render(&mut from_renderer);
        drop(from);
        renderer.framebuffer.draw_framebuffer(
            0,
            ease_in_out_expo(self.progress as f32 / 20.0) * 128.0,
            from_framebuffer.take_framebuffer(),
        );
        self.progress += 1;
        false
    }

    fn take_page(self: Box<Self>) -> Box<dyn Page> {
        self.to
    }

    fn page(&mut self) -> &mut Box<dyn Page> {
        &mut self.to
    }
}

fn ease_in_out_expo(x: f32) -> f32 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        (2.0f32.powf(20.0 * x - 10.0)) / 2.0
    } else {
        (2.0 - 2.0f32.powf(-20.0 * x + 10.0)) / 2.0
    }
}

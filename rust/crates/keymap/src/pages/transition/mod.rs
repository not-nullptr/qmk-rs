mod dither;
mod drop;
mod scale;

use crate::page::{Page, RenderInfo};
use alloc::boxed::Box;
use core::{cell::RefCell, sync::atomic::AtomicU8};

pub use dither::*;
pub use drop::*;
pub use scale::*;

pub static TRANSITION_TYPE: AtomicU8 = AtomicU8::new(0);

pub trait TransitionHandler: Send + Sync {
    fn new(to: Box<dyn Page>) -> Self
    where
        Self: Sized;
    fn render(&mut self, renderer: &mut RenderInfo) -> bool;
    fn take_page(self: Box<Self>) -> Box<dyn Page>;
}

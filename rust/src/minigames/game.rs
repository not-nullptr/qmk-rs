use core::any::type_name;

use crate::abstractions::Keycode;
use alloc::vec::Vec;

pub struct GameContext<'a> {
    pub tick_num: u32,
    pub key_buffer: &'a Vec<Keycode>,
}

pub trait Game: Send {
    fn logic_tick(&mut self, ctx: &mut GameContext) {
        let _ = ctx;
    }
    fn render_tick(&mut self, ctx: &mut GameContext) {
        let _ = ctx;
    }
    fn logic_delay(&self) -> u8 {
        1
    }
    fn id() -> &'static str
    where
        Self: Sized,
    {
        type_name::<Self>()
    }
    fn idv(&self) -> &'static str {
        type_name::<Self>()
    }
    fn create() -> Self
    where
        Self: Sized;
    fn destroy(&mut self) {}
}

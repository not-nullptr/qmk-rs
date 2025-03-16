use crate::page::Page;
use crate::pages::HomePage;
use alloc::{boxed::Box, vec::Vec};
use core::cell::RefCell;
use core::prelude::rust_2024::*;
use critical_section::Mutex;
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct InputHandler {
    events: Vec<InputEvent>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_event(&mut self, event: InputEvent) {
        self.events.push(event);
    }

    pub fn poll(&mut self) -> Option<InputEvent> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }
}

pub enum InputEvent {
    EncoderScroll(u8, bool),
    EncoderClick(u8),
}

pub static INPUT_HANDLER: Lazy<Mutex<RefCell<InputHandler>>> =
    Lazy::new(|| Mutex::new(RefCell::new(InputHandler::new())));
pub static PAGE: Lazy<Mutex<RefCell<Box<dyn Page>>>> =
    Lazy::new(|| Mutex::new(RefCell::new(Box::new(HomePage::default()))));

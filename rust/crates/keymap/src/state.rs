#![allow(dead_code)]

use crate::page::Page;
use crate::pages::HomePage;
use alloc::vec;
use alloc::{boxed::Box, vec::Vec};
use core::cell::RefCell;
use core::prelude::rust_2024::*;
use critical_section::Mutex;
use once_cell::sync::Lazy;
use qmk::qk_keycode_defines;

pub struct InputHandler {
    events: Vec<InputEvent>,
    keys: Vec<u32>,
    left_encoder_down: bool,
    right_encoder_down: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            events: vec![],
            keys: vec![],
            left_encoder_down: false,
            right_encoder_down: false,
        }
    }

    pub fn left_encoder_down(&self) -> bool {
        self.left_encoder_down
    }

    pub fn right_encoder_down(&self) -> bool {
        self.right_encoder_down
    }

    pub fn is_key_down(&self, key: u32) -> bool {
        self.keys.contains(&key)
    }

    pub fn is_key_up(&self, key: u32) -> bool {
        !self.is_key_down(key)
    }

    pub fn keys(&self) -> &[u32] {
        &self.keys
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

    pub fn collect(&mut self) -> Vec<InputEvent> {
        core::mem::take(&mut self.events)
    }

    pub fn down(&mut self, key: u32) {
        if key == qk_keycode_defines::KC_F20 {
            self.left_encoder_down = true;
        } else if key == qk_keycode_defines::KC_F21 {
            self.right_encoder_down = true;
        } else if !self.keys.contains(&key) {
            self.keys.push(key);
        }
    }

    pub fn up(&mut self, key: u32) {
        if key == qk_keycode_defines::KC_F20 {
            self.left_encoder_down = false;
        } else if key == qk_keycode_defines::KC_F21 {
            self.right_encoder_down = false;
        } else if let Some(index) = self.keys.iter().position(|&k| k == key) {
            self.keys.remove(index);
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

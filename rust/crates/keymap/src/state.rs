#![allow(dead_code)]

use crate::keymap::CS_RESET;
use crate::page::Page;
use crate::pages::StartupPage;
use alloc::vec;
use alloc::{boxed::Box, vec::Vec};
use core::cell::RefCell;
use core::hint::black_box;
use core::prelude::rust_2024::*;
use critical_section::Mutex;
use once_cell::sync::Lazy;
use qmk::keyboard::Keyboard;
use qmk::keys::{KC_C, KC_DOWN, KC_ENTER, KC_F20, KC_F21};

#[derive(Clone)]
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

    #[inline(always)]
    pub fn left_encoder_down(&self) -> bool {
        self.left_encoder_down
    }

    #[inline(always)]
    pub fn right_encoder_down(&self) -> bool {
        self.right_encoder_down
    }

    #[inline(always)]
    pub fn is_key_down(&self, key: u32) -> bool {
        self.keys.contains(&key)
    }

    #[inline(always)]
    pub fn is_key_up(&self, key: u32) -> bool {
        !self.is_key_down(key)
    }

    #[inline(always)]
    pub fn keys(&self) -> &[u32] {
        &self.keys
    }

    #[inline(always)]
    pub fn handle_event(&mut self, event: InputEvent) {
        let is_game_mode = Keyboard::layer_state_is(2);
        if !is_game_mode {
            self.events.push(event);
        }
    }

    pub fn poll(&mut self) -> Option<InputEvent> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }

    #[inline(always)]
    pub fn collect(&mut self) -> Vec<InputEvent> {
        core::mem::take(&mut self.events)
    }

    pub fn down(&mut self, key: u32) {
        if key == KC_F20 {
            self.left_encoder_down = true;
        } else if key == KC_F21 {
            self.right_encoder_down = true;
        } else if !self.keys.contains(&key) && !Keyboard::layer_state_is(2) {
            // self.keys.push(key);
        }
    }

    pub fn up(&mut self, key: u32) {
        if key == KC_F20 {
            self.left_encoder_down = false;
        } else if key == KC_F21 {
            self.right_encoder_down = false;
        } else if let Some(index) = self.keys.iter().position(|&k| k == key) {
            self.keys.remove(index);
        }
    }
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    EncoderScroll(u8, bool),
    EncoderClick(u8),
    KeyDown(u32),
}

pub static INPUT_HANDLER: Lazy<Mutex<RefCell<InputHandler>>> =
    Lazy::new(|| Mutex::new(RefCell::new(InputHandler::new())));
pub static PAGE: Lazy<Mutex<RefCell<Box<dyn Page>>>> =
    Lazy::new(|| Mutex::new(RefCell::new(Box::new(StartupPage::default()))));

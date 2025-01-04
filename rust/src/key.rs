use alloc::vec::Vec;
use critical_section::with;

use crate::{abstractions::Keycode, raw_c::get_current_wpm, state::APP_STATE};

pub struct Keyboard {
    key_buffer: Vec<Keycode>,
}

impl Keyboard {
    pub const fn new() -> Self {
        Self {
            key_buffer: Vec::new(),
        }
    }
    pub fn key_press(&mut self, keycode: Keycode) {
        self.key_buffer.insert(0, keycode);
    }
    pub fn key_release(&mut self, keycode: Keycode) {}
    pub fn get_wpm() -> u8 {
        unsafe { get_current_wpm() }
    }

    pub fn read_keys(&mut self) -> Vec<Keycode> {
        let buf = self.key_buffer.clone(); // i love embedded!
        self.key_buffer.clear();
        buf
    }
}

#[no_mangle]
pub extern "C" fn key_press_user_rs(keycode: Keycode) {
    with(|cs| {
        let mut state = APP_STATE.borrow(cs).borrow_mut();
        state.keyboard.key_press(keycode);
    });
}

#[no_mangle]
pub extern "C" fn key_release_user_rs(keycode: Keycode) {
    with(|cs| {
        let mut state = APP_STATE.borrow(cs).borrow_mut();
        state.keyboard.key_release(keycode);
    });
}

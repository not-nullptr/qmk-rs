use alloc::vec::Vec;

use crate::{
    abstractions::{Keycode, Screen},
    raw_c::get_current_wpm,
};

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
        self.write_key(keycode);
        match keycode {
            Keycode::KC_F20 | Keycode::KC_F21 => {
                Screen::change_page();
            }
            _ => {}
        }
    }
    pub fn key_release(&mut self, _keycode: Keycode) {}
    pub fn get_wpm() -> u8 {
        unsafe { get_current_wpm() }
    }

    pub fn read_keys(&mut self) -> Vec<Keycode> {
        let buf = self.key_buffer.clone(); // i love embedded!
        self.key_buffer.clear();
        buf
    }

    fn write_key(&mut self, key: Keycode) {
        self.key_buffer.insert(0, key);
    }
}

use crate::{abstractions::Keycode, raw_c::get_current_wpm};

pub struct Keyboard;

impl Keyboard {
    pub fn key_press(keycode: Keycode) {}
    pub fn key_release(keycode: Keycode) {}
    pub fn get_wpm() -> u8 {
        unsafe { get_current_wpm() }
    }
}

#[no_mangle]
pub extern "C" fn key_press_user_rs(keycode: Keycode) {
    Keyboard::key_press(keycode);
}

#[no_mangle]
pub extern "C" fn key_release_user_rs(keycode: Keycode) {
    Keyboard::key_release(keycode);
}

use crate::{
    abstractions::{Keycode, Screen},
    raw_c::get_current_wpm,
    state::AppState,
};

pub struct Keyboard;

impl Keyboard {
    pub fn key_press(keycode: Keycode, state: &mut AppState) {
        state.write_key(keycode);
        match keycode {
            Keycode::KC_F20 | Keycode::KC_F21 => {
                Screen::change_page(state);
            }
            _ => {}
        }
    }
    pub fn key_release(_keycode: Keycode, _state: &mut AppState) {}
    pub fn get_wpm() -> u8 {
        unsafe { get_current_wpm() }
    }
}

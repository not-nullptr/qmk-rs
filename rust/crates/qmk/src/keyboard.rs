use qmk_sys::is_keyboard_left;

pub struct Keyboard;

impl Keyboard {
    pub fn is_left() -> bool {
        unsafe { is_keyboard_left() }
    }

    pub fn is_right() -> bool {
        !Self::is_left()
    }
}

use qmk_sys::is_keyboard_left;

pub struct Keyboard;

impl Keyboard {
    pub fn is_left() -> bool {
        unsafe { is_keyboard_left() }
    }

    pub fn is_right() -> bool {
        !Self::is_left()
    }

    pub fn send_slave(data: *const u8, len: u8) -> bool {
        send_to_slave_wrapper(data, len)
    }
}

fn send_to_slave_wrapper(data: *const u8, len: u8) -> bool {
    unsafe extern "C" {
        fn send_to_slave(data: *const u8, len: u8) -> bool;
    }

    unsafe { send_to_slave(data, len) }
}

#[cfg(not(target_arch = "wasm32"))]
use qmk_sys::is_keyboard_left;

#[cfg(target_arch = "wasm32")]
unsafe fn is_keyboard_left() -> bool {
    true
}

pub struct Keyboard;

impl Keyboard {
    pub fn is_left() -> bool {
        unsafe { is_keyboard_left() }
    }

    pub fn is_right() -> bool {
        !Self::is_left()
    }

    pub fn send_slave(_data: *const u8, _len: u8) -> bool {
        // send_to_slave_wrapper(data, len)
        #[cfg(not(target_arch = "wasm32"))]
        return send_to_slave_wrapper(_data, _len);

        #[cfg(target_arch = "wasm32")]
        {
            true
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn send_to_slave_wrapper(data: *const u8, len: u8) -> bool {
    unsafe extern "C" {
        fn send_to_slave(data: *const u8, len: u8) -> bool;
    }

    unsafe { send_to_slave(data, len) }
}

#[cfg(target_arch = "wasm32")]
mod bindings {
    pub unsafe fn is_keyboard_left() -> bool {
        true
    }

    pub unsafe fn tap_code16(_key: u16) {}
    pub unsafe fn tap_code16_delay(_key: u16, _delay: u16) {}
}

#[cfg(not(target_arch = "wasm32"))]
mod bindings {
    pub use qmk_sys::{is_keyboard_left, tap_code16, tap_code16_delay};
}

pub struct Keyboard;

impl Keyboard {
    pub fn is_left() -> bool {
        unsafe { bindings::is_keyboard_left() }
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

    pub fn layer_state_is(layer: u8) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe extern "C" {
            fn layer_state_is(layer: u8) -> bool;
        }

        #[cfg(target_arch = "wasm32")]
        unsafe fn layer_state_is(_layer: u8) -> bool {
            false
        }

        unsafe { layer_state_is(layer) }
    }

    pub fn send_key(key: u16) {
        unsafe {
            bindings::tap_code16(key);
        }
    }

    pub fn send_key_delay(key: u16, delay: u16) {
        unsafe {
            bindings::tap_code16_delay(key, delay);
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

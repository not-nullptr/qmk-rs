#![no_std]

extern crate alloc;

pub const EEPROM_BYTES: usize = 5;

#[cfg(target_arch = "wasm32")]
use core::ffi::c_void;

pub use qmk_macro::*;
pub mod eeconfig;
pub mod framebuffer;
pub mod keyboard;
pub mod keys;
pub mod logging;
pub mod rect;
pub mod rgb;
pub mod screen;
pub mod sys;

use qmk_sys::keyrecord_t;
pub type KeyRecord = keyrecord_t;
#[cfg(not(target_arch = "wasm32"))]
pub use qmk_sys::defer_exec;
#[cfg(target_arch = "wasm32")]
pub unsafe fn defer_exec(
    _: u32,
    _: Option<unsafe extern "C" fn(u32, *mut c_void) -> u32>,
    _: *mut c_void,
) {
    // No-op in WASM
}
pub use qmk_sys::keyevent_type_t;
pub use qmk_sys::oled_rotation_t as OledRotation;

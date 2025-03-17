#![no_std]

extern crate alloc;

pub use qmk_macro::*;
pub mod framebuffer;
pub mod keyboard;
pub mod logging;
pub mod screen;

use qmk_sys::keyrecord_t;
pub type KeyRecord = keyrecord_t;
pub use qmk_sys::keyevent_type_t;
pub use qmk_sys::qk_keycode_defines;

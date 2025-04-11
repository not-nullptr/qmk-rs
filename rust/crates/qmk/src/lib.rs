#![no_std]

extern crate alloc;

pub const EEPROM_BYTES: usize = 5;

pub use qmk_macro::*;
pub mod eeconfig;
pub mod framebuffer;
pub mod keyboard;
pub mod keys;
pub mod logging;
pub mod rgb;
pub mod screen;
pub mod sys;

use qmk_sys::keyrecord_t;
pub type KeyRecord = keyrecord_t;
pub use qmk_sys::keyevent_type_t;
pub use qmk_sys::oled_rotation_t as OledRotation;

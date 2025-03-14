#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]

extern crate alloc;
extern crate core;

mod heap;
mod oled;

use panic_halt as _;
use qmk_macro::save;

save!("../keyboards/sofle/keymaps/nulls_keymap/rust_bindings.c");

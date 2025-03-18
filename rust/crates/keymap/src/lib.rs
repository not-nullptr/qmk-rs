#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]

use qmk::{keyboard::Keyboard, qmk_callback, rgb::RGBLight};

extern crate alloc;
extern crate core;

mod animation;
#[cfg(not(test))]
mod heap;
mod image;
mod input;
mod page;
mod pages;
mod random;
mod screen;
mod state;

pub const HID_SYNC: i8 = 0x00;

#[qmk_callback(() -> void)]
fn keyboard_post_init_user() {
    RGBLight::set_hsv(0, 0, 0);
}

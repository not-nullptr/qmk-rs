#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]

extern crate alloc;
extern crate core;

mod abstractions;
mod animate;
mod debug;
mod heap;
mod hid;
mod image;
mod keyboard;
mod keymap;
mod macros;
mod minigames;
mod random;
mod raw_c;
mod rotary_encoder;
mod screen;
mod state;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

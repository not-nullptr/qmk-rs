#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]
#![allow(warnings)]

extern crate core;

mod abstractions;
mod image;
mod raw_c;
mod screen;
mod scroll;
mod state;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

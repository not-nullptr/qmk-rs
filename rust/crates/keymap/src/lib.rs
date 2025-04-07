#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]

extern crate alloc;
extern crate core;
#[cfg(target_arch = "wasm32")]
extern crate std;

mod animation;
mod config;
mod heap;
mod image;
mod init;
mod input;
mod page;
mod pages;
mod random;
mod screen;
mod state;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn start() {
    use console_error_panic_hook::set_once;
    set_once();
}

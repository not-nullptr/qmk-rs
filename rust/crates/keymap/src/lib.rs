#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]

extern crate alloc;
extern crate core;

#[cfg(not(test))]
mod heap;
mod image;
mod screen;

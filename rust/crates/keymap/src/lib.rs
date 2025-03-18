#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]

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

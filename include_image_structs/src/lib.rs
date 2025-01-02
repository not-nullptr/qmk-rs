#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]
#![allow(warnings)]

#[derive(Debug)]
pub struct Chunk {
    pub x: u8,
    pub y: u8,
    pub bytes: [u8; 8],
}

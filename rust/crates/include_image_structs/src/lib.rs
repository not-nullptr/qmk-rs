#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]
#![allow(warnings)]

#[derive(Debug, Copy, Clone)]
pub struct QmkImage<const N: usize> {
    pub width: u8,
    pub height: u8,
    pub bytes: [u8; N],
}

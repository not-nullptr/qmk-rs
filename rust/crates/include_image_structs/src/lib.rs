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

impl<const N: usize> QmkImage<N> {
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.width as usize || y >= self.height as usize {
            return None;
        }
        let byte_index = (y / 8) * self.width as usize + x;
        let bit_index = y % 8;
        Some((self.bytes[byte_index] >> bit_index) & 1 == 1)
    }
}

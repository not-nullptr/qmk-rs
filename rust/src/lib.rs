#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]
#![allow(warnings)]

extern crate alloc;
extern crate core;

mod abstractions;
mod image;
mod key;
mod macros;
mod raw_c;
mod rotary_encoder;
mod screen;
mod state;

use abstractions::Screen;
use embedded_alloc::LlffHeap as Heap;
use rp2040_hal as _;

pub const HEAP_SIZE: usize = 8096;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    Screen::draw_text("oops!");
    loop {}
}

#[no_mangle]
pub extern "C" fn init_allocator_rs() {
    use core::mem::MaybeUninit;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

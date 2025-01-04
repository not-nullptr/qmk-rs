use critical_section::with;
use embedded_alloc::LlffHeap as Heap;
use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    timer::{Alarm0, Timer},
    watchdog::Watchdog,
};

use crate::{raw_c::timer_read, state::APP_STATE};

pub const HEAP_SIZE: usize = 64000; // 25% of memory or so

#[global_allocator]
pub static HEAP: Heap = Heap::empty();

#[no_mangle]
pub extern "C" fn init_allocator_rs() {
    use core::mem::MaybeUninit;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

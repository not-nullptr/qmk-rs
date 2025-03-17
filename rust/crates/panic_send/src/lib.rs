#![deny(warnings)]
#![no_std]
#![allow(unused_imports)]

use core::fmt::{self, Write};
use core::mem::MaybeUninit;
use core::panic::PanicInfo;

#[unsafe(link_section = ".crash_info")]
#[cfg(not(test))]
static mut CRASH_INFO: [MaybeUninit<u8>; 1024] = [MaybeUninit::uninit(); 1024];

#[inline(never)]
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use core::slice;

    #[allow(static_mut_refs)]
    let slice =
        unsafe { slice::from_raw_parts_mut(CRASH_INFO.as_mut_ptr() as *mut u8, CRASH_INFO.len()) };

    slice[0] = 0xCC;

    loop {}
}

#[cfg(not(target_arch = "wasm32"))]
use embedded_alloc::LlffHeap;
use qmk::qmk_callback;
#[cfg(not(target_arch = "wasm32"))]
use rp2040_hal as _;
#[cfg(not(target_arch = "wasm32"))]
use rp2040_panic_usb_boot as _;

#[cfg(not(target_arch = "wasm32"))]
const HEAP_SIZE: usize = 64000;

#[cfg(not(target_arch = "wasm32"))]
#[global_allocator]
pub static HEAP: LlffHeap = LlffHeap::empty();

#[qmk_callback(() -> void)]
fn keyboard_pre_init_user() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use core::mem::MaybeUninit;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe {
            #[allow(static_mut_refs)]
            HEAP.init(HEAP_MEM.as_mut_ptr() as usize, HEAP_SIZE);
        };
    }
}

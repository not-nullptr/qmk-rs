use embedded_alloc::LlffHeap;
use panic_halt as _;
use qmk::qmk_callback;
use rp2040_hal as _;

const HEAP_SIZE: usize = 64000;

#[global_allocator]
pub static HEAP: LlffHeap = LlffHeap::empty();

#[qmk_callback(() -> void)]
fn keyboard_pre_init_user() {
    use core::mem::MaybeUninit;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe {
        #[allow(static_mut_refs)]
        HEAP.init(HEAP_MEM.as_mut_ptr() as usize, HEAP_SIZE);
    };
}

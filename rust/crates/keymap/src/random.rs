use core::sync::atomic::{AtomicU32, Ordering};

static RANDOM: AtomicU32 = AtomicU32::new(0xDEADBEEF);

pub fn rand() -> u32 {
    let mut state = RANDOM.load(Ordering::Relaxed);
    state = (state.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7FFFFFFF;
    RANDOM.store(state, Ordering::Relaxed);
    state
}

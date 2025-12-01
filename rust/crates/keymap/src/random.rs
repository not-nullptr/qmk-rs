#![allow(dead_code)]

use core::{
    ops::Range,
    sync::atomic::{AtomicU32, Ordering},
};

static RANDOM: AtomicU32 = AtomicU32::new(0xDEADBEEF);

pub fn rand() -> u32 {
    let mut state = RANDOM.load(Ordering::Relaxed);
    state = (state.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7FFFFFFF;
    RANDOM.store(state, Ordering::Relaxed);
    state
}

pub fn rand_between(range: Range<u32>) -> u32 {
    let diff = range.end - range.start;
    range.start + (rand() % diff)
}

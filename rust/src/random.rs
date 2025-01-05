static mut SEED: u32 = 532;

pub fn seed(seed: u32) {
    unsafe {
        SEED = seed;
    }
}

pub fn rnd() -> u32 {
    unsafe {
        let mut z: u32 = SEED + 0x9e3779b9;
        z ^= z >> 15;
        z *= 0x85ebca6b;
        z ^= z >> 13;
        z *= 0xc2b2ae35;
        z ^= z >> 16;
        SEED = z;
        z
    }
}

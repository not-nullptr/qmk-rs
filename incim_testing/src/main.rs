use include_image::generate_sin_table;

generate_sin_table!(1024);

fn fpsin(i: i16) -> i16 {
    let mut i = i;
    // convert (signed) input to a value between 0 and 8192 (pi/2 region of the curve fit)
    i <<= 1;
    let c = i < 0; // set carry for output pos/neg

    if i == (i | 0x4000) {
        // flip input value to corresponding value in range [0..8192)
        i = (1 << 15) - i;
    }
    i = (i & 0x7FFF) >> 1;

    // constants for the formula
    const A1: u32 = 3_370_945_099;
    const B1: u32 = 2_746_362_156;
    const C1: u32 = 292_421;
    const N: u32 = 13;
    const P: u32 = 32;
    const Q: u32 = 31;
    const R: u32 = 3;
    const A: u32 = 12;

    let mut y = (C1 * (i as u32)) >> N;
    y = B1 - ((i as u32 * y) >> R);
    y = (i as u32) * (y >> N);
    y = (i as u32) * (y >> N);
    y = A1 - (y >> (P - Q));
    y = (i as u32) * (y >> N);
    y = (y + (1 << (Q - A - 1))) >> (Q - A); // rounding

    if c {
        -(y as i16)
    } else {
        y as i16
    }
}

// cos(x) = sin(x + pi/2)
fn fpcos(i: i16) -> i16 {
    fpsin(i.wrapping_add(8192))
}
fn main() {
    println!("{:?}", fpsin(500));
}

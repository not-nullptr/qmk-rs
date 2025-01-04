use critical_section::with;
use include_image_structs::QmkImage;

use crate::state::APP_STATE;

pub fn animate_frames<const N: usize>(
    slowness: u32,
    frames: &[QmkImage<N>],
    counter: u32,
) -> QmkImage<N> {
    let len = frames.len();
    let counter = counter.div_ceil(slowness) % len as u32;
    return frames[counter as usize];
}

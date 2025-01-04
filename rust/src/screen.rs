use alloc::{borrow::ToOwned, format, string::ToString};
use critical_section::with;

use crate::{
    abstractions::{Keycode, Screen},
    animate::animate_frames,
    heap::{HEAP, HEAP_SIZE},
    image::CREDITS,
    keyboard::Keyboard,
    raw_c::{get_u8_str, oled_write},
    state::{AppPage, APP_STATE},
};

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn remap(value: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    new_min + (value - old_min) * (new_max - new_min) / (old_max - old_min)
}

#[no_mangle]
pub extern "C" fn oled_task_user_rs() -> bool {
    Screen::render();
    false
}

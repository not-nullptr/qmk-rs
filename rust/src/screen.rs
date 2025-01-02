use crate::{
    abstractions::Screen,
    image::RUST_LOGO,
    state::{app_state, AppStateLock},
};
use heapless::String;

#[no_mangle]
pub extern "C" fn oled_task_user_rs() -> bool {
    Screen::draw_chunks(&RUST_LOGO);
    false
}

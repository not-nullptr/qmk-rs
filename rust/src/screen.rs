use alloc::{format, string::ToString};
use critical_section::with;

use crate::{
    abstractions::Screen,
    image::RUST_LOGO,
    key::Keyboard,
    raw_c::{get_u8_str, oled_write},
    state::APP_STATE,
};

#[no_mangle]
pub extern "C" fn oled_task_user_rs() -> bool {
    // let wpm = Keyboard::get_wpm();
    // Screen::draw_text("  WPM");
    // Screen::draw_text(" ");
    // unsafe {
    // oled_write(get_u8_str(wpm, '0' as i8), false);
    // }
    // let height = RUST_LOGO.height;
    // Screen::draw_image(&RUST_LOGO, 0, Screen::SCREEN_HEIGHT - height);

    with(|cs| {
        let state = APP_STATE.borrow(cs).borrow();
        Screen::draw_text("COUNT", true);
        Screen::draw_text(&state.count.to_string(), true);
    });

    false
}

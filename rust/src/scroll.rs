use crate::image::RUST_LOGO;
use crate::{
    abstractions::{press_key, Keycode, Screen},
    state::{app_state, AppState, AppStateLock},
};
fn perform_scroll(up: bool) {
    let key = if up {
        Keycode::KC_PAGE_UP
    } else {
        Keycode::KC_PAGE_DOWN
    };
    press_key(key);

    let (reader, mut writer) = app_state.unwrap_rw();
    writer.write(AppState {
        count: reader.read().count + if up { 1 } else { -1 },
        ..reader.read()
    });
    AppStateLock::wrap_rw(reader, writer);
    Screen::clear(true);
}

#[no_mangle]
pub extern "C" fn encoder_update_user_rs(index: u8, clockwise: bool) -> bool {
    perform_scroll(!clockwise);
    false
}

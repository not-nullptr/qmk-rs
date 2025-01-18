use crate::abstractions::Keycode;

fn on_scroll(up: bool) {
    let _key = if up {
        Keycode::KC_PAGE_UP
    } else {
        Keycode::KC_PAGE_DOWN
    };
}

#[no_mangle]
pub extern "C" fn encoder_update_user(_index: u8, clockwise: bool) -> bool {
    on_scroll(!clockwise);
    false
}

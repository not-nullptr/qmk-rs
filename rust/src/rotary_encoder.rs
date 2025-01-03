use core::cell::RefCell;

use alloc::string::String;
use alloc::vec::Vec;
use critical_section::{with, Mutex};
use enum_iterator::{first, next};

use crate::abstractions::{press_key, Keycode, Screen};
use crate::heap::HEAP;
use crate::image::RUST_LOGO;
use crate::raw_c::qp_sh1106_make_i2c_device;
use crate::segfault;
use crate::state::{AppPage, APP_STATE};

fn on_scroll(up: bool) {
    let key = if up {
        Keycode::KC_PAGE_UP
    } else {
        Keycode::KC_PAGE_DOWN
    };

    // with(|cs| {
    //     let mut binding = APP_STATE.borrow(cs).borrow_mut();
    //     binding.count += if up { -1 } else { 1 };
    // });
}

static a: Mutex<RefCell<Vec<u8>>> = Mutex::new(RefCell::new(Vec::new()));

fn on_press(index: u8) {
    with(|cs| {
        let mut state = APP_STATE.borrow(cs).borrow_mut();
        let Some(next) = next(&state.page) else {
            let Some(first) = first::<AppPage>() else {
                return;
            };
            state.page = first;
            Screen::clear(false);
            return;
        };
        state.page = next;
        Screen::clear(false);
    });
}

#[no_mangle]
pub extern "C" fn encoder_update_user_rs(index: u8, clockwise: bool) -> bool {
    on_scroll(!clockwise);
    false
}

#[no_mangle]
pub extern "C" fn encoder_press_user_rs(index: u8) {
    on_press(index);
}

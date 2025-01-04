use core::cell::RefCell;

use alloc::string::String;
use alloc::vec::Vec;
use critical_section::{with, Mutex};
use enum_iterator::{first, next};

use crate::abstractions::{press_key, Keycode, Screen};
use crate::debug;
use crate::heap::HEAP;
use crate::raw_c::qp_sh1106_make_i2c_device;
use crate::segfault;
use crate::state::{AppPage, APP_STATE};

fn on_scroll(up: bool) {
    let key = if up {
        Keycode::KC_PAGE_UP
    } else {
        Keycode::KC_PAGE_DOWN
    };
}

#[no_mangle]
pub extern "C" fn encoder_update_user_rs(index: u8, clockwise: bool) -> bool {
    on_scroll(!clockwise);
    false
}

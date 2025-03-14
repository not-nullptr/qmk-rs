#![allow(non_upper_case_globals)]

use crate::{
    abstractions::Keycode, keyboard::Keyboard, random::seed, raw_c::KeyRecord, state::APP_STATE,
};
use critical_section::with;
use qmk_macro::qmk_callback;

#[qmk_callback((uint16_t, keyrecord_t*) -> bool)]
fn process_record_user(keycode: Keycode, record: *mut KeyRecord) -> bool {
    with(|cs| {
        let record = unsafe { *record };
        seed(record.event.time as u32);
        let mut state = APP_STATE.borrow(cs).borrow_mut();
        if record.event.pressed {
            Keyboard::key_press(keycode, &mut state);
        } else {
            Keyboard::key_release(keycode, &mut state);
        }
    });
    true
}

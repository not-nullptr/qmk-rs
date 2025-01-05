use critical_section::with;

use crate::{
    abstractions::Keycode, keyboard::Keyboard, random::seed, raw_c::KeyRecord, state::APP_STATE,
};

#[no_mangle]
pub extern "C" fn process_record_user_rs(keycode: Keycode, record: *mut KeyRecord) -> bool {
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
    false
}

use crate::debug;
use critical_section::with;

use crate::{abstractions::Keycode, raw_c::KeyRecord, state::APP_STATE};

#[no_mangle]
pub extern "C" fn process_record_user_rs(keycode: Keycode, record: *mut KeyRecord) -> bool {
    let record = unsafe { *record };
    debug!("{:?}, {:?}", keycode, record);
    with(|cs| {
        let mut state = APP_STATE.borrow(cs).borrow_mut();
        state.debug_count = keycode as i32;
        if record.event.pressed {
            state.keyboard.key_press(keycode);
        } else {
            state.keyboard.key_release(keycode);
        }
    });
    false
}

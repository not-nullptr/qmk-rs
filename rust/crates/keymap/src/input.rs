use crate::state::{INPUT_HANDLER, InputEvent};
use critical_section::with;
use qmk::{KeyRecord, qk_keycode_defines, qmk_callback};

#[qmk_callback((uint8_t, bool) -> bool)]
fn encoder_update_user(index: u8, clockwise: bool) -> bool {
    with(|cs| {
        let mut input_handler = INPUT_HANDLER.borrow_ref_mut(cs);
        input_handler.handle_event(InputEvent::EncoderScroll(index, clockwise));
    });
    false
}

#[qmk_callback((uint16_t, keyrecord_t*) -> bool)]
pub fn process_record_user(keycode: u16, record: *const KeyRecord) -> bool {
    let record = unsafe { *record };
    if record.event.type_ != 257 {
        return false;
    }

    let event = match keycode as u32 {
        qk_keycode_defines::KC_F20 => InputEvent::EncoderClick(0),
        qk_keycode_defines::KC_F21 => InputEvent::EncoderClick(1),
        _ => return false,
    };

    with(|cs| {
        let mut input_handler = INPUT_HANDLER.borrow_ref_mut(cs);
        input_handler.handle_event(event);
    });

    false
}

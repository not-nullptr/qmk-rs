use crate::state::{INPUT_HANDLER, InputEvent};
use critical_section::with;
use qmk::{
    KeyRecord,
    keys::{KC_F20, KC_F21},
    qmk_callback,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

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
    with(|cs| {
        let mut input_handler = INPUT_HANDLER.borrow_ref_mut(cs);
        let keycode = keycode as u32;
        let record = unsafe { *record };
        if record.event.type_ != 257 {
            input_handler.up(keycode);
            return true;
        }

        input_handler.down(keycode);

        let event = match keycode {
            KC_F20 => InputEvent::EncoderClick(0),
            KC_F21 => InputEvent::EncoderClick(1),
            _ => InputEvent::KeyDown(keycode),
        };

        if matches!(&event, InputEvent::EncoderClick(_)) {
            input_handler.handle_event(event);
            return false;
        }

        input_handler.handle_event(event);
        true
    })
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn process_record_wasm(keycode: u16, time: u16, pressed: bool) -> bool {
    use qmk::sys::{keyevent_t, keypos_t, tap_t};

    let record = KeyRecord {
        tap: tap_t {
            _bitfield_1: Default::default(),
            _bitfield_align_1: [],
        },
        event: keyevent_t {
            key: keypos_t { row: 0, col: 0 },
            pressed,
            time,
            type_: if pressed { 257 } else { 0 },
        },
    };

    process_record_user(keycode, &record as *const KeyRecord)
}

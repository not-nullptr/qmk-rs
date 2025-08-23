use core::ffi::c_void;

use crate::{
    keymap::CS_RESET,
    state::{INPUT_HANDLER, InputEvent},
};
use critical_section::with;
use qmk::{
    KeyRecord, defer_exec,
    keyboard::Keyboard,
    keys::{KC_C, KC_DOWN, KC_ENTER, KC_F20, KC_F21},
    qmk_callback, qmk_log,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[qmk_callback((uint8_t, bool) -> bool)]
fn encoder_update_user(index: u8, clockwise: bool) -> bool {
    with(|cs| {
        if let Ok(mut input_handler) = INPUT_HANDLER.borrow(cs).try_borrow_mut() {
            input_handler.handle_event(InputEvent::EncoderScroll(index, clockwise));
        }
    });
    false
}

#[unsafe(no_mangle)]
unsafe extern "C" fn macro_callback(_: u32, cb_arg: *mut c_void) -> u32 {
    let key = cb_arg as u32;
    let key = key as u16;
    Keyboard::send_key_delay(key, 50);
    0
}

#[qmk_callback((uint16_t, keyrecord_t*) -> bool)]
pub fn process_record_user(keycode: u16, record: *const KeyRecord) -> bool {
    let keycode = keycode as u32;
    let record = unsafe { *record };
    if record.event.type_ == 257 {
        let inputs = match keycode as u16 {
            CS_RESET => Some(&[KC_ENTER, KC_DOWN, KC_C]),
            _ => None,
        };

        if let Some(inputs) = inputs {
            for (i, key) in inputs.iter().enumerate() {
                let key = *key;

                unsafe {
                    defer_exec((i as u32 * 5) + 1, Some(macro_callback), key as *mut c_void);
                }
            }
        };
    }

    with(|cs| {
        let Ok(mut input_handler) = INPUT_HANDLER.borrow(cs).try_borrow_mut() else {
            return false;
        };

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

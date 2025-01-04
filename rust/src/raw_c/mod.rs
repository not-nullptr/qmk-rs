#![allow(warnings)]

use crate::abstractions::Keycode;

pub struct PainterDevice;

extern "C" {
    pub fn tap_code16(keycode: Keycode);
    pub fn oled_write(s: *const core::ffi::c_char, i: bool);
    pub fn oled_clear();
    pub fn oled_render_dirty(all: bool);
    pub fn oled_write_raw(data: *const u8, size: u16);
    pub fn oled_set_cursor(col: u8, row: u8);
    pub fn oled_write_pixel(x: u8, y: u8, on: bool);
    pub fn qp_sh1106_make_i2c_device(width: u16, height: u16, i2c_address: u8) -> PainterDevice;
    pub fn get_current_wpm() -> u8;
    pub fn get_u8_str(curr_num: u8, curr_pad: core::ffi::c_char) -> *const core::ffi::c_char;
    pub fn putchar_(character: core::ffi::c_char);
    pub fn timer_read() -> u16;
    pub fn timer_elapsed(timer: u16) -> u16;
    pub static mut layer_state: u16;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Tap {
    pub interrupted: bool,
    pub reserved_2: bool,
    pub reserved_1: bool,
    pub reserved_0: bool,
    pub count: u8,
}

#[derive(Clone, Copy, Debug)]
pub enum KeyEventType {
    TICK_EVENT = 0,
    KEY_EVENT = 1,
    ENCODER_CW_EVENT = 2,
    ENCODER_CCW_EVENT = 3,
    COMBO_EVENT = 4,
    DIP_SWITCH_ON_EVENT = 5,
    DIP_SWITCH_OFF_EVENT = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KeyPos {
    pub col: u8,
    pub row: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KeyEvent {
    pub key: KeyPos,
    pub time: u16,
    pub event_type: KeyEventType,
    pub pressed: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KeyRecord {
    pub event: KeyEvent,
    pub tap: Tap,
}

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

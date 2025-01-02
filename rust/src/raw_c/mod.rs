use crate::abstractions::Keycode;

extern "C" {
    pub fn tap_code16(keycode: Keycode);
    pub fn oled_write(s: &str);
    pub fn oled_clear();
    pub fn oled_render_dirty(all: bool);
    pub fn oled_write_raw(data: *const u8, size: u16);
    pub fn oled_set_cursor(col: u8, row: u8);
    pub fn oled_write_pixel(x: u8, y: u8, on: bool);
}

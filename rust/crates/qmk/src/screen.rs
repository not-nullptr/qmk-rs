#[cfg(not(target_arch = "wasm32"))]
use core::borrow::Borrow;

#[cfg(not(target_arch = "wasm32"))]
use alloc::{ffi::CString, string::String};
#[cfg(not(target_arch = "wasm32"))]
use include_image::QmkImage;
#[cfg(not(target_arch = "wasm32"))]
use num_traits::{Num, ToPrimitive};

pub struct Screen;

impl Screen {
    pub const OLED_DISPLAY_WIDTH: usize = 64;
    pub const OLED_DISPLAY_HEIGHT: usize = 128;
    pub const OLED_DISPLAY_SIZE: usize =
        ((Self::OLED_DISPLAY_WIDTH * Self::OLED_DISPLAY_HEIGHT) / 8);

    #[cfg(not(target_arch = "wasm32"))]
    pub fn draw_pixel<T, U>(x: T, y: U)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let x = x.to_u8().unwrap_or(255);
        let y = y.to_u8().unwrap_or(255);

        unsafe {
            qmk_sys::oled_write_pixel(x, y, true);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn clear_pixel<T, U>(x: T, y: U)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let x = x.to_u8().unwrap_or(255);
        let y = y.to_u8().unwrap_or(255);

        unsafe {
            qmk_sys::oled_write_pixel(x, y, false);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn draw_image<T, U, B, I, const N: usize>(x: T, y: U, image: &I)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
        I: QmkImage,
    {
        let image: &I = image.borrow();

        let width = image.width() as usize;
        let height = image.height() as usize;

        let offset_x = x.to_u8().unwrap_or(255);
        let offset_y = y.to_u8().unwrap_or(255);

        let columns = u32::div_ceil(height as u32, 8);
        for y_block in 0..columns {
            let y_offset = y_block * 8;
            for x in 0..width {
                let byte = image.as_bytes()[x + y_block as usize * width];
                for bit in 0..8 {
                    let is_on = (byte & (1 << bit)) != 0;
                    let x = x + offset_x as usize;
                    let y_offset = y_offset as u8 + offset_y;
                    if is_on {
                        Screen::draw_pixel(x, y_offset + bit);
                    } else {
                        Screen::clear_pixel(x, y_offset + bit);
                    }
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn draw_text<T, U>(row: T, col: U, text: impl Into<String>)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let Ok(text) = CString::new(text.into()) else {
            return;
        };
        let col = col.to_u8().unwrap_or(255);
        let row = row.to_u8().unwrap_or(255);

        unsafe {
            qmk_sys::oled_set_cursor(col, row);
            qmk_sys::oled_write(text.as_ptr(), false);
        }
    }
}

use alloc::{ffi::CString, string::String};
use include_image::QmkImage;
use num_traits::{Num, ToPrimitive};

pub struct Screen;

impl Screen {
    pub const OLED_DISPLAY_WIDTH: usize = 64;
    pub const OLED_DISPLAY_HEIGHT: usize = 128;
    pub const OLED_DISPLAY_SIZE: usize =
        ((Self::OLED_DISPLAY_WIDTH * Self::OLED_DISPLAY_HEIGHT) / 8);

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

    pub fn draw_image<T, U, const N: usize>(x: T, y: U, image: QmkImage<N>)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let offset_x = x.to_u8().unwrap_or(255);
        let offset_y = y.to_u8().unwrap_or(255);

        let columns = u32::div_ceil(image.height as u32, 8);
        for y_block in 0..columns {
            let y_offset = y_block * 8;
            for x in 0..image.width {
                let byte = image.bytes[x as usize + y_block as usize * image.width as usize];
                for bit in 0..8 {
                    let is_on = (byte & (1 << bit)) != 0;
                    let x = x + offset_x;
                    let y_offset = y_offset as u8 + offset_y;
                    if is_on {
                        Screen::draw_pixel(x, (y_offset + bit) as u8);
                    } else {
                        Screen::clear_pixel(x, (y_offset + bit) as u8);
                    }
                }
            }
        }
    }

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

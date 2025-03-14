use include_image::QmkImage;
use num_traits::{Num, ToPrimitive};

pub struct Screen;

impl Screen {
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
}

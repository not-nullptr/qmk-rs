use num_traits::{Num, ToPrimitive};

pub struct RGBLight;

impl RGBLight {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_hsv<H, S, V>(hue: H, sat: S, val: V)
    where
        H: Num + ToPrimitive,
        S: Num + ToPrimitive,
        V: Num + ToPrimitive,
    {
        let hue = hue.to_u8().unwrap_or(0);
        let sat = sat.to_u8().unwrap_or(0);
        let val = val.to_u8().unwrap_or(0);
        unsafe {
            qmk_sys::rgblight_sethsv(hue, sat, val);
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_hsv<H, S, V>(_: H, _: S, _: V)
    where
        H: Num + ToPrimitive,
        S: Num + ToPrimitive,
        V: Num + ToPrimitive,
    {
        // websites do not have rgb matrices, believe it or not
    }
}

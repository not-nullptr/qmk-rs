use alloc::{format, string::String};

#[macro_export]
macro_rules! qmk_log {
    ($($arg:tt)*) => {
        $crate::logging::printf(::alloc::format!($($arg)*));
    };
}

pub fn printf(s: impl Into<String>) {
    let s = s.into();
    for char in s.chars() {
        unsafe {
            qmk_sys::sendchar(char as u8);
        }
    }

    unsafe {
        qmk_sys::sendchar(b'\n');
    }
}

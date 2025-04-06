use alloc::string::String;

#[macro_export]
macro_rules! qmk_log {
    ($($arg:tt)*) => {
        $crate::logging::printf(::alloc::format!($($arg)*));
    };
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
pub fn printf(s: impl Into<String>) {
    use web_sys::console;
    let s = s.into();
    console::log_1(&s.into());
}

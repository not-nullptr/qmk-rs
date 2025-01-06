use crate::raw_c::putchar_;

pub struct Debug;

impl Debug {
    pub fn write(text: &str) {
        for byte in text.bytes() {
            unsafe {
                putchar_(byte);
            }
        }

        unsafe {
            putchar_(b'\n');
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            crate::debug::Debug::write(
                &alloc::format!($($arg)*)
            );
        }
    };
}

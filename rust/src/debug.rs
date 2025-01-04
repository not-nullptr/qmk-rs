use crate::raw_c::putchar_;

pub struct Debug;

impl Debug {
    pub fn write(text: &str) {
        for byte in text.bytes() {
            unsafe {
                putchar_(byte as i8);
            }
        }

        unsafe {
            putchar_(b'\n' as i8);
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

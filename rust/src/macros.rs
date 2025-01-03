#[macro_export]
macro_rules! segfault {
    () => {
        unsafe {
            *(0 as *mut u8) = 0;
        }
    };
}

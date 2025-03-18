use qmk_sys::is_keyboard_left;

pub struct Keyboard;

impl Keyboard {
    pub fn is_left() -> bool {
        unsafe { is_keyboard_left() }
    }

    pub fn is_right() -> bool {
        !Self::is_left()
    }

    pub fn send_slave(data: *const u8, len: u8) -> bool {
        unsafe extern "C" {
            fn send_to_slave(data: *const u8, len: u8) -> bool;
        }
        // unsafe {
        //     qmk_sys::transaction_rpc_exec(id, len, data as *mut c_void, 0, core::ptr::null_mut())
        // }

        unsafe { send_to_slave(data, len) }
    }
}

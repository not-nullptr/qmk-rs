use alloc::boxed::Box;
use core::ffi::c_void;
use qmk_sys::{is_keyboard_left, transaction_register_rpc, transaction_rpc_exec};

pub struct Keyboard;

fn closure_to_ffi<F: Fn(u8, *const c_void, u8, *mut c_void) + Send>(
    f: F,
) -> (
    *mut F,
    unsafe extern "C" fn(*mut F, u8, *const c_void, u8, *mut c_void),
) {
    unsafe extern "C" fn call_closure_from_ffi<F: Fn(u8, *const c_void, u8, *mut c_void) + Send>(
        f: *mut F,
        in_buflen: u8,
        in_data: *const c_void,
        out_buflen: u8,
        out_data: *mut c_void,
    ) {
        unsafe { (*f)(in_buflen, in_data, out_buflen, out_data) }
    }
    (Box::into_raw(Box::new(f)), call_closure_from_ffi::<F>)
}

impl Keyboard {
    pub fn is_left() -> bool {
        unsafe { is_keyboard_left() }
    }

    pub fn is_right() -> bool {
        !Self::is_left()
    }

    pub fn register_transaction(
        id: i8,
        callback: unsafe extern "C" fn(u8, *const c_void, u8, *mut c_void),
    ) {
        unsafe { transaction_register_rpc(id, Some(callback)) }
    }

    pub fn call_on_slave(id: i8, data: &[u8]) -> bool {
        unsafe {
            let mut arr: [u8; 0] = [];
            let arr_ptr = arr.as_mut_ptr() as *mut c_void;
            let arr_len = arr.len() as u8;

            let data_ptr = data.as_ptr() as *const c_void;
            let data_len = data.len() as u8;

            transaction_rpc_exec(id, data_len, data_ptr, arr_len, arr_ptr)
        }
    }
}

use qmk_sys::{
    eeconfig_init_user_datablock, eeconfig_is_user_datablock_valid, eeconfig_read_user_datablock,
    eeconfig_update_user_datablock,
};

use crate::EEPROM_BYTES;
use core::ffi::c_void;

struct Helper<T>(T);
impl<T> Helper<T> {
    const fn get_size() -> usize {
        core::mem::size_of::<T>()
    }
}

pub struct EEConfig;

impl EEConfig {
    pub fn save<T: Sized>(object: &T) {
        let size = Helper::<T>::get_size();
        if size > EEPROM_BYTES {
            panic!("Size of T is greater than EEPROM_BYTES");
        }

        let object_ptr = object as *const T as *const c_void;
        unsafe {
            eeconfig_update_user_datablock(object_ptr);
        }
    }

    pub fn load<T: Sized>() -> T {
        let size = Helper::<T>::get_size();
        if size > EEPROM_BYTES {
            panic!("Size of T is greater than EEPROM_BYTES");
        }

        let mut object: T = unsafe { core::mem::zeroed() };
        let object_ptr = &mut object as *mut T as *mut c_void;
        unsafe {
            eeconfig_read_user_datablock(object_ptr);
        }
        object
    }

    pub fn is_valid() -> bool {
        unsafe { eeconfig_is_user_datablock_valid() }
    }

    pub fn init() {
        unsafe { eeconfig_init_user_datablock() }
    }
}

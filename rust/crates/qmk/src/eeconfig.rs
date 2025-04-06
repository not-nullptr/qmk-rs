#[cfg(target_arch = "wasm32")]
use alloc::string::ToString as _;
#[cfg(not(target_arch = "wasm32"))]
use qmk_sys::{
    eeconfig_init_user_datablock, eeconfig_is_user_datablock_valid, eeconfig_read_user_datablock,
    eeconfig_update_user_datablock,
};
#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(not(target_arch = "wasm32"))]
use crate::EEPROM_BYTES;

#[cfg(not(target_arch = "wasm32"))]
use core::ffi::c_void;
use core::marker::PhantomData;

pub struct Unchecked;
pub struct Checked;
pub trait EEConfigState {}
impl EEConfigState for Unchecked {}
impl EEConfigState for Checked {}

/// A struct to handle EEPROM configuration data.
///
/// It asserts at compile time that the size of the data type does not exceed the EEPROM size.
pub struct EEConfig<T: Sized, State: EEConfigState = Unchecked> {
    _state: PhantomData<State>,
    _data: PhantomData<T>,
}

// its okay to allow this here because new HAS to be const for the compile-time assert
#[allow(clippy::new_without_default)]
impl<T: Sized> EEConfig<T, Unchecked> {
    /// Create a new checked instance of `EEConfig`.
    /// This is a runtime no-op, equivalent of caling EEConfig::save/load directly.
    pub const fn new() -> EEConfig<T, Checked> {
        #[cfg(not(target_arch = "wasm32"))]
        assert!(
            core::mem::size_of::<T>() <= EEPROM_BYTES,
            "Size of T exceeds EEPROM size"
        );

        EEConfig::<T, Checked> {
            _state: PhantomData,
            _data: PhantomData,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Sized> EEConfig<T, Checked> {
    // at this point we can guarantee that the data size <= EEPROM_BYTES

    pub fn save(&self, object: &T) {
        let object_ptr = object as *const T as *const c_void;
        unsafe {
            eeconfig_update_user_datablock(object_ptr);
        }
    }

    pub fn load(&self) -> T {
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

#[cfg(target_arch = "wasm32")]
impl<T: Sized + Serialize + for<'a> Deserialize<'a> + Default> EEConfig<T, Checked> {
    pub fn save(&self, obj: &T) {
        let json = serde_json::to_string(obj).unwrap_or_else(|_| "{}".to_string());
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        local_storage.set_item("qmk_eeconfig", &json).unwrap();
    }

    pub fn load(&self) -> T {
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        let json = local_storage
            .get_item("qmk_eeconfig")
            .unwrap()
            .unwrap_or_else(|| "{}".to_string());
        serde_json::from_str(&json).unwrap_or_else(|_| T::default())
    }

    pub fn is_valid() -> bool {
        true
    }

    pub fn init() {}
}

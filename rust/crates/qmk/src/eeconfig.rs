#[cfg(not(target_arch = "wasm32"))]
use qmk_sys::{
    eeconfig_init_user_datablock, eeconfig_is_user_datablock_valid, eeconfig_read_user_datablock,
    eeconfig_update_user_datablock,
};
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(not(target_arch = "wasm32"))]
use crate::EEPROM_BYTES;
#[cfg(target_arch = "wasm32")]
use crate::qmk_log;
#[cfg(target_arch = "wasm32")]
use alloc::format;
#[cfg(target_arch = "wasm32")]
use alloc::vec::Vec;
#[cfg(not(target_arch = "wasm32"))]
use core::ffi::c_void;
#[cfg(target_arch = "wasm32")]
use core::fmt::Debug;
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
impl<T: Sized + Default + Debug> EEConfig<T, Checked> {
    pub fn save(&self, obj: &T) {
        let bytes = unsafe {
            core::slice::from_raw_parts(obj as *const T as *const u8, core::mem::size_of::<T>())
        };

        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();

        let hex_string = bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<_>>()
            .join("");

        local_storage
            .set_item("qmk_eeconfig", &hex_string)
            .unwrap_or_else(|_| {
                qmk_log!("Failed to save eeconfig to local storage");
            });
    }

    pub fn load(&self) -> T {
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        let hex_string = local_storage
            .get_item("qmk_eeconfig")
            .unwrap_or_else(|_| {
                qmk_log!("Failed to load eeconfig from local storage");
                None
            })
            .unwrap_or_default();

        if hex_string.is_empty() {
            return T::default();
        }

        let bytes = hex_string
            .as_bytes()
            .chunks(2)
            .map(|chunk| {
                let hex_str = core::str::from_utf8(chunk).unwrap_or("00");
                u8::from_str_radix(hex_str, 16).unwrap_or(0)
            })
            .collect::<Vec<_>>();
        let mut object: T = unsafe { core::mem::zeroed() };
        let object_ptr = &mut object as *mut T as *mut u8;
        let object_slice =
            unsafe { core::slice::from_raw_parts_mut(object_ptr, core::mem::size_of::<T>()) };

        if object_slice.len() != bytes.len() {
            qmk_log!(
                "EEConfig: Length mismatch, expected {} but got {}",
                object_slice.len(),
                bytes.len()
            );
            return T::default();
        }

        object_slice.copy_from_slice(&bytes);
        qmk_log!("EEConfig: Loaded object: {:?}", object);
        object
    }

    pub fn is_valid() -> bool {
        true
    }

    pub fn init() {}
}

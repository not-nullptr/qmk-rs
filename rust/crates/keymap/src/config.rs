use core::cell::RefCell;

use critical_section::Mutex;
use qmk::eeconfig::EEConfig;

pub static SETTINGS: Mutex<RefCell<UserConfig>> = Mutex::new(RefCell::new(UserConfig::new()));

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(target_arch = "wasm32", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PageTransition {
    #[default]
    Dither,
    Scale,
    Slide,
    Doom,
}

impl PageTransition {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => PageTransition::Dither,
            1 => PageTransition::Scale,
            2 => PageTransition::Slide,
            3 => PageTransition::Doom,
            _ => PageTransition::Dither,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(target_arch = "wasm32", derive(serde::Serialize, serde::Deserialize))]
pub struct UserConfig {
    pub transition: PageTransition,
    pub hsv: [u8; 3],
}

impl UserConfig {
    pub const fn new() -> Self {
        let _ = EEConfig::<UserConfig>::new();
        Self {
            transition: PageTransition::Dither,
            hsv: [0, 0, 0],
        }
    }

    pub fn load(&mut self) {
        let eeconfig = EEConfig::new();
        *self = eeconfig.load();
    }

    pub fn save(&self) {
        let eeconfig = EEConfig::new();
        eeconfig.save(self);
    }
}

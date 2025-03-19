use core::cell::RefCell;

use critical_section::Mutex;
use qmk::eeconfig::EEConfig;

pub static SETTINGS: Mutex<RefCell<UserConfig>> = Mutex::new(RefCell::new(UserConfig::new()));

#[derive(Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum PageTransition {
    #[default]
    Dither,
    Scale,
    Slide,
}

impl PageTransition {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => PageTransition::Dither,
            1 => PageTransition::Scale,
            2 => PageTransition::Slide,
            _ => PageTransition::Dither,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct UserConfig {
    pub transition: PageTransition,
    pub hsv: [u8; 3],
}

impl UserConfig {
    pub const fn new() -> Self {
        Self {
            transition: PageTransition::Dither,
            hsv: [0, 0, 0],
        }
    }

    pub fn load(&mut self) {
        *self = EEConfig::load();
    }

    pub fn save(&self) {
        EEConfig::save(self);
    }
}

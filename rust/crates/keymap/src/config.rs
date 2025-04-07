use alloc::string::String;
use alloc::string::ToString;
use core::cell::RefCell;
use critical_section::Mutex;
use once_cell::sync::Lazy;
use qmk::eeconfig::EEConfig;

pub static SETTINGS: Lazy<Mutex<RefCell<UserConfig>>> = Lazy::new(|| {
    let mut config = UserConfig::new();
    config.load();
    Mutex::new(RefCell::new(config))
});

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(serde::Serialize, serde::Deserialize, layout_inspect::Inspect)
)]
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

#[derive(Debug, Default, Clone)]
#[cfg_attr(target_arch = "wasm32", derive(serde::Serialize, serde::Deserialize))]
pub struct Hsv(pub [u8; 3]);

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(serde::Serialize, serde::Deserialize, layout_inspect::Inspect)
)]
#[repr(C)]
pub struct UserConfig {
    pub transition: PageTransition,
    pub hsv: Hsv,
}

#[cfg(target_arch = "wasm32")]
impl layout_inspect::Inspect for Hsv {
    fn name() -> std::string::String {
        "[u8; 3]".to_string()
    }

    fn align() -> Option<usize> {
        Some(0x1)
    }

    fn def(collector: &mut layout_inspect::TypesCollector) -> layout_inspect::defs::DefType {
        layout_inspect::defs::DefType::Vec(layout_inspect::defs::DefVec {
            name: Self::name(),
            size: Self::size().unwrap(),
            align: Self::align().unwrap(),
            value_type_id: collector.collect::<u8>(),
        })
    }

    fn size() -> Option<usize> {
        Some(core::mem::size_of::<Self>())
    }
}

impl UserConfig {
    pub fn new() -> Self {
        let _ = EEConfig::<UserConfig>::new();
        Self {
            transition: PageTransition::Dither,
            hsv: Hsv([0, 0, 0]),
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn get_layout() -> String {
    use layout_inspect::inspect;
    let types = inspect::<UserConfig>();
    let layout_inspect::defs::DefType::Struct(ref info) = types[0] else {
        panic!("Expected a struct");
    };

    serde_json::to_string_pretty(&info).unwrap()
}

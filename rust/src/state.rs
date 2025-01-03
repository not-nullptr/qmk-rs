use core::cell::RefCell;
use critical_section::Mutex;
use enum_iterator::Sequence;

#[derive(Clone, Copy, Sequence)]
pub enum AppPage {
    Stats,
    Heap,
    KeyD,
    Debug,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub page: AppPage,
    pub debug_count: i32,
    pub cpu_usage: u8,
    pub mem_usage: u8,
}

impl AppState {
    pub const fn new() -> Self {
        Self {
            page: AppPage::Stats,
            debug_count: 0,
            cpu_usage: 0,
            mem_usage: 0,
        }
    }
}

pub static APP_STATE: Mutex<RefCell<AppState>> = Mutex::new(RefCell::new(AppState::new()));

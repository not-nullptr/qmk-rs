use core::cell::RefCell;

use critical_section::Mutex;

#[derive(Clone, Copy)]
pub struct AppState {
    pub count: i32,
}

impl AppState {
    pub const fn new() -> Self {
        Self { count: 0 }
    }
}

pub static APP_STATE: Mutex<RefCell<AppState>> = Mutex::new(RefCell::new(AppState::new()));

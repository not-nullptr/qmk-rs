use alloc::vec::Vec;
use core::cell::RefCell;
use critical_section::Mutex;
use enum_iterator::Sequence;

use crate::key::Keyboard;

#[derive(Clone, Copy, Sequence)]
pub enum AppPage {
    Stats,
    Heap,
    KeyD,
    Debug,
    Credits,
}

impl AppPage {
    pub fn get_title(&self) -> &str {
        match self {
            AppPage::Stats => "STATS",
            AppPage::Debug => "DEBUG",
            AppPage::Heap => "HEAP",
            AppPage::KeyD => "KEYD",
            AppPage::Credits => "CREDS",
        }
    }
}

pub struct GameState {
    pub x: u8,
    pub y: u8,
}

impl GameState {
    pub const fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub struct AppState {
    pub page: AppPage,
    pub debug_count: i32,
    pub cpu_usage: u8,
    pub mem_usage: u8,
    pub animation_counter: u32,
    pub keyboard: Keyboard,
    pub game_state: GameState,
}

impl AppState {
    pub const fn new() -> Self {
        Self {
            page: AppPage::Stats,
            debug_count: 0,
            cpu_usage: 0,
            mem_usage: 0,
            animation_counter: 0,
            keyboard: Keyboard::new(),
            game_state: GameState::new(),
        }
    }
}

pub static APP_STATE: Mutex<RefCell<AppState>> = Mutex::new(RefCell::new(AppState::new()));

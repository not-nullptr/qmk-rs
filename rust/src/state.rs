use alloc::{boxed::Box, string::String, vec::Vec};
use core::cell::RefCell;
use critical_section::Mutex;
use enum_iterator::Sequence;

use crate::{abstractions::Keycode, minigames::game::Game};

#[derive(Clone, Copy, Sequence)]
pub enum AppPage {
    Stats,
    Heap,
    KeyD,
    Tetris,
    FlappyBird,
    Raycast,
    Debug,
    Credits,
}

impl AppPage {
    pub fn get_title(&self) -> Option<&str> {
        match self {
            AppPage::Stats => Some("STATS"),
            AppPage::Debug => Some("DEBUG"),
            AppPage::Heap => Some("HEAP"),
            AppPage::KeyD => Some("KEYD"),
            AppPage::Credits => Some("CREDS"),
            AppPage::Tetris => None,
            AppPage::FlappyBird => None,
            AppPage::Raycast => None,
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
    pub process_count: u16,
    pub animation_counter: u32,
    key_buffer: Vec<Keycode>,
    pub game: Option<Box<dyn Game>>,
    pub debug_str: String,
}

impl AppState {
    pub const fn new() -> Self {
        Self {
            page: AppPage::Stats,
            debug_count: 0,
            cpu_usage: 0,
            mem_usage: 0,
            process_count: 0,
            animation_counter: 0,
            key_buffer: Vec::new(),
            game: None,
            debug_str: String::new(),
        }
    }

    pub fn write_key(&mut self, keycode: Keycode) {
        self.key_buffer.push(keycode);
    }

    pub fn read_keys(&mut self) -> Vec<Keycode> {
        let clone = self.key_buffer.clone();
        self.key_buffer.clear();
        clone
    }
}

pub static APP_STATE: Mutex<RefCell<AppState>> = Mutex::new(RefCell::new(AppState::new()));

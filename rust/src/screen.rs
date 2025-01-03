use alloc::{format, string::ToString};
use critical_section::with;

use crate::{
    abstractions::Screen,
    heap::{HEAP, HEAP_SIZE},
    image::RUST_LOGO,
    key::Keyboard,
    raw_c::{get_u8_str, oled_write},
    state::{AppPage, APP_STATE},
};

#[no_mangle]
pub extern "C" fn oled_task_user_rs() -> bool {
    with(|cs| {
        let state = APP_STATE.borrow(cs).borrow();
        match state.page {
            AppPage::Stats => {
                Screen::draw_text("STATS", true);
                Screen::newline();
                Screen::draw_text("WPM:", true);
                Screen::draw_text(&Keyboard::get_wpm().to_string(), true);
            }

            AppPage::Heap => {
                let used = HEAP.used();
                let free = HEAP.free();
                let used = if used == 0 { 0 } else { used.div_ceil(1000) };
                let free = if free == 0 { 0 } else { free.div_ceil(1000) };
                let critical = used >= HEAP_SIZE / 2;
                let used = format!("{}kb", used);
                let free = format!("{}kb", free);
                Screen::draw_text("HEAP", true);
                Screen::newline();
                Screen::draw_text("Used:", true);
                if critical {
                    Screen::draw_text_inverted(&used, true);
                } else {
                    Screen::draw_text(&used, true);
                }
                Screen::newline();
                Screen::draw_text("Free:", true);
                if critical {
                    Screen::draw_text_inverted(&free, true);
                } else {
                    Screen::draw_text(&free, true);
                }
            }

            AppPage::KeyD => {
                Screen::draw_text("KEYD", true);
                Screen::newline();
                Screen::draw_text("CPU", true);
                Screen::draw_text(&format!("{}%", state.cpu_usage), true);
                Screen::newline();
                Screen::draw_text("RAM", true);
                Screen::draw_text(&format!("{}%", state.mem_usage), true);
            }

            AppPage::Debug => {
                Screen::draw_text("DEBUG", true);
                Screen::newline();
                Screen::draw_text("Count", true);
                Screen::draw_text(&state.debug_count.to_string(), true);
            }
        }
    });

    false
}

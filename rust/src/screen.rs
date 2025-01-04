use alloc::{borrow::ToOwned, format, string::ToString};
use critical_section::with;

use crate::{
    abstractions::{Keycode, Screen},
    animate::animate_frames,
    heap::{HEAP, HEAP_SIZE},
    image::CREDITS,
    key::Keyboard,
    raw_c::{get_u8_str, oled_write},
    state::{AppPage, APP_STATE},
};

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn remap(value: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    new_min + (value - old_min) * (new_max - new_min) / (old_max - old_min)
}

#[no_mangle]
pub extern "C" fn oled_task_user_rs() -> bool {
    with(|cs| {
        let mut state = APP_STATE.borrow(cs).borrow_mut();
        let keys = state.keyboard.read_keys();
        state.animation_counter += 1;
        if let Some(title) = state.page.get_title() {
            Screen::draw_text(title, true);
            Screen::newline();
        }
        match state.page {
            AppPage::Stats => {
                let wpm = Keyboard::get_wpm();
                Screen::draw_text("WPM:", true);
                Screen::draw_text(&wpm.to_string(), true);
            }

            AppPage::Heap => {
                let used = HEAP.used();
                let free = HEAP.free();
                let used = if used == 0 { 0 } else { used.div_ceil(1000) };
                let free = if free == 0 { 0 } else { free.div_ceil(1000) };
                let critical = used >= HEAP_SIZE / 2;
                let used = format!("{}kb", used);
                let free = format!("{}kb", free);
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
                Screen::draw_text("CPU", true);
                Screen::draw_text(&format!("{}%", state.cpu_usage), true);
                Screen::newline();
                Screen::draw_text("RAM", true);
                Screen::draw_text(&format!("{}%", state.mem_usage), true);
                Screen::newline();
                Screen::draw_text("Procs", true);
                Screen::draw_text(&format!("{}", state.process_count), true);
            }

            AppPage::Debug => {}

            AppPage::Credits => {
                Screen::draw_text("wrote", true);
                Screen::draw_text("by", true);
                Screen::draw_text("null-", true);
                Screen::draw_text("ptr", true);
                Screen::draw_text("in rs", true);
                Screen::newline();
                Screen::draw_text("sofle", true);
                Screen::draw_text("ftw!!", true);
                let y = Screen::SCREEN_HEIGHT - CREDITS[0].height - 8;
                Screen::draw_image(&animate_frames(6, &CREDITS, state.animation_counter), 0, y);
            }
        }
    });

    false
}

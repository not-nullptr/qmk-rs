use crate::page::{Page, RenderInfo};
use alloc::{boxed::Box, format, string::String, vec::Vec};
use core::fmt::Write;
use core::sync::atomic::Ordering;
use core::{cell::RefCell, sync::atomic::AtomicU32};
use critical_section::{Mutex, with};
use qmk::keyboard::Keyboard;
use qmk::qmk_callback;
use qmk::screen::Screen;

static TIME: Mutex<RefCell<Option<String>>> = Mutex::new(RefCell::new(None));
static LEVEL: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn on_usb_slave_data(data: *const u8, len: u8) {
    let data = unsafe { core::slice::from_raw_parts(data, len as usize) };
    if !data.starts_with(&[0xFF, 0xCC, 0x00, 0xAA]) {
        return;
    }
    if data.len() < 16 {
        return;
    }
    let Ok(time) = data[4..12].try_into() else {
        return;
    };
    let time = i64::from_le_bytes(time);
    let Ok(level) = data[12..16].try_into() else {
        return;
    };
    let level = u32::from_le_bytes(level);
    let date = format_dd_mm_ss(time);
    let time = format_hh_mm_ss(time);
    let time = format!("{}\n{}", date, time);

    with(|cs| {
        let mut time_ref = TIME.borrow_ref_mut(cs);
        *time_ref = Some(time);
        LEVEL.store(level, Ordering::SeqCst);
    });
}

#[qmk_callback((uint8_t*, uint8_t) -> void)]
fn raw_hid_receive(data: *const u8, len: u8) {
    Keyboard::send_slave(data, len);
}

pub fn format_dd_mm_ss(unix_timestamp: i64) -> String {
    const SECONDS_PER_DAY: i64 = 86400;
    let days_since_epoch = unix_timestamp / SECONDS_PER_DAY;

    let mut year = 1970;
    let mut days_remaining = days_since_epoch;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days_remaining < days_in_year {
            break;
        }
        days_remaining -= days_in_year;
        year += 1;
    }

    let month_days = [
        31,
        if is_leap_year(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let mut month = 0;
    while month < 12 && days_remaining >= month_days[month] {
        days_remaining -= month_days[month];
        month += 1;
    }

    let day = days_remaining + 1;

    let mut result = String::with_capacity(8);

    if day < 10 {
        result.push('0');
    }
    write!(result, "{}", day).unwrap();

    result.push('/');

    let month = month + 1;
    if month < 10 {
        result.push('0');
    }
    write!(result, "{}", month).unwrap();

    result.push('/');

    write!(result, "{:02}", year % 100).unwrap();

    result
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn format_hh_mm_ss(epoch: i64) -> String {
    let total_seconds = epoch.rem_euclid(86400);
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub struct ClockPage;

impl Page for ClockPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        renderer
            .framebuffer
            .draw_text_centered(32, 20, "Clock", false);

        let level = LEVEL.load(Ordering::SeqCst);

        if let Some(time) = TIME.borrow_ref(renderer.cs).clone() {
            // renderer.framebuffer.draw_text_centered(32, 40, time, false);
            let lines: Vec<&str> = time.split('\n').collect();
            for (i, line) in lines.iter().enumerate() {
                let y = 40 + (i * 10);
                renderer.framebuffer.draw_text_centered(32, y, *line, false);
            }
        } else {
            renderer
                .framebuffer
                .draw_text_centered(32, 40, "No Time", false);
        }

        renderer.framebuffer.fill_rect(
            8,
            100,
            level.min((Screen::OLED_DISPLAY_WIDTH - 16) as u32),
            16,
        );

        None
    }
}

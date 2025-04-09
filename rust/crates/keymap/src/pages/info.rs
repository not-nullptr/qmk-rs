use core::sync::atomic::Ordering;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::page::{Page, RenderInfo};
use crate::screen::TICK;
use crate::state::InputEvent;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use qmk::framebuffer::{CHAR_HEIGHT, CHAR_WIDTH};
use qmk::screen::Screen;

use super::HomePage;

const WINDOW_TICKS: u32 = 20 * 5;
const WPM_HISTORY_SIZE: usize = 20;

#[derive(Default)]
pub struct InfoPage {
    key_times: VecDeque<u32>,
    wpm_history: Vec<f32>,
    tick: u32,
}

impl Page for InfoPage {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        if self.wpm_history.len() < WPM_HISTORY_SIZE {
            self.wpm_history = alloc::vec![0.0; WPM_HISTORY_SIZE];
        }

        while let Some(event) = renderer.input.poll() {
            let InputEvent::KeyDown(_) = event else {
                return Some(Box::new(HomePage::default()));
            };
            self.key_times.push_back(self.tick);
        }

        while let Some(&time) = self.key_times.front() {
            if self.tick.saturating_sub(time) > WINDOW_TICKS {
                self.key_times.pop_front();
            } else {
                break;
            }
        }

        let keys_in_window = self.key_times.len();
        let wpm = if keys_in_window > 0 {
            let num_words = keys_in_window as f32 / 5.0;
            let extrapolation_multiplier = 60.0 / (WINDOW_TICKS as f32 / 20.0);
            num_words * extrapolation_multiplier
        } else {
            0.0
        };

        // Update the history on every tick for increased responsiveness.
        self.wpm_history.push(wpm);
        if self.wpm_history.len() > WPM_HISTORY_SIZE {
            self.wpm_history.remove(0);
        }

        renderer
            .framebuffer
            .draw_text_centered(32, 8, "Uptime", false);
        renderer
            .framebuffer
            .draw_text_centered(32, 18, self.uptime(), false);

        self.draw_graph(renderer);

        self.tick = self.tick.wrapping_add(1);

        None
    }
}

impl InfoPage {
    fn draw_graph(&mut self, renderer: &mut RenderInfo) {
        let max_wpm = self
            .wpm_history
            .iter()
            .cloned()
            .fold(0.0, f32::max)
            .max(50.0);

        let min_wpm: f32 = max_wpm - 50.0;

        let max_wpm_str = format!("{:.0}", max_wpm);
        let min_wpm_str = format!("{:.0}", min_wpm);

        let longer_len = max_wpm_str.len().max(min_wpm_str.len());

        const GRAPH_WIDTH: u8 = 32;
        const GRAPH_HEIGHT: u8 = 32;
        let graph_x: u8 = ((Screen::OLED_DISPLAY_WIDTH as u8 - GRAPH_WIDTH) / 2)
            + (longer_len * CHAR_WIDTH) as u8
            - 8;
        const GRAPH_Y: u8 = Screen::OLED_DISPLAY_HEIGHT as u8 - 12 - GRAPH_HEIGHT;

        renderer
            .framebuffer
            .draw_text_centered(32, GRAPH_Y - 12, "WPM", false);

        renderer
            .framebuffer
            .fill_rect(graph_x, GRAPH_Y, 1, GRAPH_HEIGHT);

        renderer
            .framebuffer
            .fill_rect(graph_x, GRAPH_Y + GRAPH_HEIGHT, GRAPH_WIDTH, 1);

        let bar_width = if !self.wpm_history.is_empty() {
            GRAPH_WIDTH as f32 / self.wpm_history.len() as f32
        } else {
            GRAPH_WIDTH as f32
        };

        for (i, &wpm) in self.wpm_history.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let x = (graph_x + 2 + (i as f32 * bar_width) as u8) as f32;
            let y = GRAPH_Y + GRAPH_HEIGHT
                - ((wpm - min_wpm) / (max_wpm - min_wpm) * GRAPH_HEIGHT as f32) as u8;

            let prev_x = graph_x + 2 + (((i - 1) as f32 * bar_width) as u8);
            let prev_y = GRAPH_Y + GRAPH_HEIGHT
                - ((self.wpm_history[i - 1] - min_wpm) / (max_wpm - min_wpm) * GRAPH_HEIGHT as f32)
                    as u8;

            renderer.framebuffer.draw_line(
                (prev_x as f32 + bar_width / 2.0) - 1.0,
                prev_y,
                (x + bar_width / 2.0) - 1.0,
                y,
            );
        }

        renderer.framebuffer.draw_text(
            graph_x - (CHAR_WIDTH * min_wpm_str.len()) as u8 - 2,
            GRAPH_Y + GRAPH_HEIGHT - (CHAR_HEIGHT as u8) + 2,
            min_wpm_str,
            false,
        );

        renderer.framebuffer.draw_text(
            graph_x - (CHAR_WIDTH * max_wpm_str.len()) as u8 - 2,
            GRAPH_Y,
            max_wpm_str,
            false,
        );
    }

    fn uptime(&self) -> String {
        let uptime_ticks = TICK.load(Ordering::SeqCst);
        let uptime_seconds = uptime_ticks / 20;
        if uptime_seconds < 60 {
            pluralize(uptime_seconds, "sec")
        } else if uptime_seconds < 60 * 60 {
            let mins = uptime_seconds / 60;
            pluralize(mins, "min")
        } else if uptime_seconds < 60 * 60 * 24 {
            let hrs = uptime_seconds / (60 * 60);
            pluralize(hrs, "hr")
        } else {
            let days = uptime_seconds / (60 * 60 * 24);
            pluralize(days, "day")
        }
    }
}

fn pluralize(count: u32, unit: &str) -> String {
    if count == 1 {
        format!("{} {}", count, unit)
    } else {
        format!("{} {}s", count, unit)
    }
}

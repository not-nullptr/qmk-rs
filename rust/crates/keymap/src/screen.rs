use core::{
    cell::RefCell,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::{
    page::RenderInfo,
    pages::{DitherTransition, SlideTransition, TRANSITION_TYPE, TransitionHandler},
    state::{INPUT_HANDLER, PAGE},
};
use alloc::boxed::Box;
use critical_section::{CriticalSection, Mutex, with};
use qmk::{framebuffer::Framebuffer, keyboard::Keyboard, qmk_callback, screen::Screen};

pub static TICK: AtomicU32 = AtomicU32::new(0);
static TRANSITION: Mutex<RefCell<Option<Box<dyn TransitionHandler>>>> =
    Mutex::new(RefCell::new(None));

#[qmk_callback(() -> bool)]
fn oled_task_user() -> bool {
    if Keyboard::is_right() {
        return false;
    }
    let tick = TICK.load(Ordering::SeqCst);
    TICK.store(tick.wrapping_add(1), Ordering::SeqCst);
    // we need to use a critical section here because the framebuffer
    // is not actually owned by us -- it's owned by the C code and
    // we need to make sure that they GO AWAY while we're using it
    with(|cs| {
        let mut framebuffer = Framebuffer::new();
        draw_screen(&mut framebuffer, cs);
        draw_border(&mut framebuffer);
        framebuffer.render();
        false
    })
}

fn draw_border(framebuffer: &mut Framebuffer) {
    const BORDER_THICKNESS: u8 = 2;
    const BORDER_ROUNDING: u8 = 2;

    framebuffer.fill_rect(0, 0, Screen::OLED_DISPLAY_WIDTH as u8, BORDER_THICKNESS);
    framebuffer.fill_rect(0, 0, BORDER_THICKNESS, Screen::OLED_DISPLAY_HEIGHT as u8);
    framebuffer.fill_rect(
        Screen::OLED_DISPLAY_WIDTH as u8 - BORDER_THICKNESS,
        0,
        BORDER_THICKNESS,
        Screen::OLED_DISPLAY_HEIGHT as u8,
    );
    framebuffer.fill_rect(
        0,
        Screen::OLED_DISPLAY_HEIGHT as u8 - BORDER_THICKNESS,
        Screen::OLED_DISPLAY_WIDTH as u8,
        BORDER_THICKNESS,
    );

    const BORDER_ROUNDING_CALCULATED: u8 = BORDER_ROUNDING + (BORDER_THICKNESS * 2);

    for offset in 0..BORDER_ROUNDING {
        framebuffer.draw_line(
            0,
            BORDER_ROUNDING_CALCULATED - offset - 1,
            BORDER_ROUNDING_CALCULATED - offset - 1,
            0,
        );
    }

    for offset in 0..BORDER_ROUNDING {
        framebuffer.draw_line(
            Screen::OLED_DISPLAY_WIDTH as u8 - BORDER_ROUNDING_CALCULATED + offset,
            0,
            Screen::OLED_DISPLAY_WIDTH as u8,
            BORDER_ROUNDING_CALCULATED - offset,
        );
    }

    for offset in 0..BORDER_ROUNDING {
        framebuffer.draw_line(
            Screen::OLED_DISPLAY_WIDTH as u8 - BORDER_ROUNDING_CALCULATED + offset - 1,
            Screen::OLED_DISPLAY_HEIGHT as u8,
            Screen::OLED_DISPLAY_WIDTH as u8,
            Screen::OLED_DISPLAY_HEIGHT as u8 - BORDER_ROUNDING_CALCULATED + offset - 1,
        );
    }

    for offset in 0..BORDER_ROUNDING {
        framebuffer.draw_line(
            0,
            Screen::OLED_DISPLAY_HEIGHT as u8 - BORDER_ROUNDING_CALCULATED + offset,
            BORDER_ROUNDING_CALCULATED - offset,
            Screen::OLED_DISPLAY_HEIGHT as u8,
        );
    }
}

fn draw_screen(framebuffer: &mut Framebuffer, cs: CriticalSection) {
    let tick = TICK.load(Ordering::SeqCst);
    let mut input = INPUT_HANDLER.borrow_ref_mut(cs);
    let mut info = RenderInfo {
        framebuffer,
        cs,
        tick,
        input: &mut *input,
    };
    let mut transitioning = TRANSITION.borrow_ref_mut(cs);
    if let Some(mut transition) = transitioning.take() {
        if transition.render(&mut info) {
            let new_page = transition.take_page();
            let mut page = PAGE.borrow_ref_mut(cs);
            *page = new_page;
            drop(page);
            drop(input);
            drop(transitioning);
            draw_screen(framebuffer, cs);
        } else {
            *transitioning = Some(transition);
        }
        return;
    }

    let mut page = PAGE.borrow_ref_mut(cs);
    if let Some(new_page) = page.render(&mut info) {
        drop(page);
        drop(input);
        *transitioning = match TRANSITION_TYPE.load(Ordering::SeqCst) {
            0 => Some(Box::new(DitherTransition::new(new_page))),
            1 => Some(Box::new(SlideTransition::new(new_page))),
            _ => Some(Box::new(DitherTransition::new(new_page))),
        };
        drop(transitioning);
        draw_screen(framebuffer, cs);
    }
}

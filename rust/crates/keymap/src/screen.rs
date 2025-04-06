use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};

use crate::{
    config::PageTransition,
    page::{Page as _, RenderInfo},
    pages::{
        Actions, ClockPage, DitherTransition, DoomTransition, ScaleTransition, SlideTransition,
        TRANSITION_TYPE, TransitionHandler,
    },
    state::{INPUT_HANDLER, PAGE},
};
use alloc::{boxed::Box, vec::Vec};
use critical_section::{CriticalSection, Mutex, with};
use qmk::{framebuffer::Framebuffer, keyboard::Keyboard, qmk_callback, screen::Screen};

pub static TICK: AtomicU32 = AtomicU32::new(0);
pub static TRANSITION: Mutex<RefCell<Option<Box<dyn TransitionHandler>>>> =
    Mutex::new(RefCell::new(None));
pub static IS_TRANSITIONING: AtomicBool = AtomicBool::new(false);
static RIGHT_HAND_PAGE: Mutex<RefCell<Option<ClockPage>>> = Mutex::new(RefCell::new(None));

#[cfg(not(target_arch = "wasm32"))]
#[qmk_callback(() -> bool)]
fn oled_task_user() -> bool {
    let (actions, fb) = if Keyboard::is_right() {
        render_right()
    } else {
        render_left()
    };

    fb.render();

    for action in actions {
        action();
    }

    false
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn oled_task_user_wasm(canvas: web_sys::HtmlCanvasElement) {
    let (actions, fb) = if Keyboard::is_right() {
        render_right()
    } else {
        render_left()
    };

    fb.render(canvas);

    for action in actions {
        action();
    }
}

fn render_left() -> (Actions, Framebuffer) {
    let tick = TICK.load(Ordering::SeqCst);
    TICK.store(tick.wrapping_add(1), Ordering::SeqCst);
    with(|cs| {
        let mut framebuffer = Framebuffer::default();
        let actions = draw_screen(&mut framebuffer, cs);
        draw_border(&mut framebuffer);
        (actions, framebuffer)
    })
}

fn render_right() -> (Actions, Framebuffer) {
    with(|cs| {
        if RIGHT_HAND_PAGE.borrow_ref(cs).is_none() {
            let mut page = RIGHT_HAND_PAGE.borrow_ref_mut(cs);
            *page = Some(ClockPage);
        }
        let mut framebuffer = Framebuffer::default();
        let mut page = RIGHT_HAND_PAGE.borrow_ref_mut(cs);
        let mut actions = alloc::vec![];
        if let Some(ref mut page) = *page {
            page.render(&mut RenderInfo {
                framebuffer: &mut framebuffer,
                cs,
                tick: TICK.load(Ordering::SeqCst),
                input: &mut INPUT_HANDLER.borrow_ref_mut(cs),
                actions: &mut actions,
            });
        }

        TICK.store(
            TICK.load(Ordering::SeqCst).wrapping_add(1),
            Ordering::SeqCst,
        );

        draw_border(&mut framebuffer);

        (actions, framebuffer)
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

fn draw_screen(framebuffer: &mut Framebuffer, cs: CriticalSection) -> Vec<Box<dyn FnOnce()>> {
    let tick = TICK.load(Ordering::SeqCst);
    let mut input = INPUT_HANDLER.borrow_ref_mut(cs);
    let mut actions = alloc::vec![];
    let mut info = RenderInfo {
        framebuffer,
        cs,
        tick,
        input: &mut input,
        actions: &mut actions,
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
            IS_TRANSITIONING.store(false, Ordering::SeqCst);
            actions.extend(draw_screen(framebuffer, cs));
        } else {
            *transitioning = Some(transition);
        }
        return actions;
    }

    let mut page = PAGE.borrow_ref_mut(cs);
    if let Some(mut new_page) = page.render(&mut info) {
        new_page.init(&mut info);
        drop(page);
        drop(input);
        *transitioning = match PageTransition::from_u8(TRANSITION_TYPE.load(Ordering::SeqCst)) {
            PageTransition::Dither => Some(Box::new(DitherTransition::new(new_page))),
            PageTransition::Scale => Some(Box::new(ScaleTransition::new(new_page))),
            PageTransition::Slide => Some(Box::new(SlideTransition::new(new_page))),
            PageTransition::Doom => Some(Box::new(DoomTransition::new(new_page))),
        };
        IS_TRANSITIONING.store(true, Ordering::SeqCst);
        drop(transitioning);
        actions.extend(draw_screen(framebuffer, cs));
        return actions;
    }

    actions
}

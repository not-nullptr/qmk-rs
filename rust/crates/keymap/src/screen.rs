use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};

use crate::{
    animation::{AngularFrequency, DampingRatio, DeltaTime, Spring, fps},
    cat::Cat,
    config::PageTransition,
    page::{Page as _, RenderInfo},
    pages::{
        Actions, ClockPage, DitherTransition, DoomTransition, NoneTransition, ScaleTransition,
        SlideTransition, TRANSITION_TYPE, TransitionHandler,
    },
    state::{INPUT_HANDLER, PAGE},
};
use alloc::{boxed::Box, vec::Vec};
use alloc::{format, string::String};
use critical_section::{CriticalSection, Mutex, with};
use once_cell::sync::Lazy;
use qmk::{
    OledRotation,
    framebuffer::{Affine2, CHAR_WIDTH, FixedNumber, Framebuffer, FramebufferTransparency},
    keyboard::Keyboard,
    screen::Screen,
};
use qmk::{framebuffer::CHAR_HEIGHT, qmk_callback};

pub static TICK: AtomicU32 = AtomicU32::new(0);
pub static TRANSITION: Mutex<RefCell<Option<Box<dyn TransitionHandler>>>> =
    Mutex::new(RefCell::new(None));
pub static IS_TRANSITIONING: AtomicBool = AtomicBool::new(false);
static RIGHT_HAND_PAGE: Mutex<RefCell<Option<ClockPage>>> = Mutex::new(RefCell::new(None));

static MARQUEE_HEIGHT_SPRING: Mutex<RefCell<Lazy<Spring>>> =
    Mutex::new(RefCell::new(Lazy::new(|| {
        Spring::new(DeltaTime(fps(15)), AngularFrequency(9.0), DampingRatio(0.2))
    })));

static MARQUEE_TEXT: Mutex<RefCell<Option<String>>> = Mutex::new(RefCell::new(None));

const MARQUEE_HEIGHT: u16 = CHAR_HEIGHT as u16 + 4;

pub fn marquee(text: impl AsRef<str>) {
    let text = text.as_ref();
    let text_len = text.len();
    let screen_chars = Screen::OLED_DISPLAY_WIDTH / CHAR_WIDTH;

    let padding_count = screen_chars.saturating_sub(text_len);
    let padding = " ".repeat(padding_count);
    let marquee_text = format!("{}  {}", text, padding);

    with(|cs| {
        let mut spring = MARQUEE_HEIGHT_SPRING.borrow_ref_mut(cs);
        spring.set(MARQUEE_HEIGHT as f32);

        let mut marquee_text_ref = MARQUEE_TEXT.borrow_ref_mut(cs);
        *marquee_text_ref = Some(marquee_text);
    });
}

pub fn disable_marquee(text: impl AsRef<str>) {
    with(|cs| {
        let text = text.as_ref();
        let marquee_text = MARQUEE_TEXT.borrow_ref(cs);
        let Some(existing_text) = marquee_text.as_ref() else {
            return;
        };

        if text != existing_text.trim() {
            return;
        }

        let mut spring = MARQUEE_HEIGHT_SPRING.borrow_ref_mut(cs);
        spring.set(0.0);
    });
}

#[qmk_callback((oled_rotation_t) -> oled_rotation_t)]
fn oled_init_user(_: OledRotation::Type) -> OledRotation::Type {
    OledRotation::OLED_ROTATION_0
}

static CAT: Mutex<RefCell<Cat>> = Mutex::new(RefCell::new(Cat::new()));

#[cfg(not(target_arch = "wasm32"))]
#[qmk_callback(() -> bool)]
fn oled_task_user() -> bool {
    let mut handler = with(|cs| INPUT_HANDLER.borrow_ref(cs).clone());

    let actions = with(|cs| {
        let (actions, mut fb) = if Keyboard::is_right() {
            render_right()
        } else {
            render_left()
        };

        let mut info = RenderInfo {
            framebuffer: &mut fb,
            cs,
            tick: TICK.load(Ordering::SeqCst),
            input: &mut handler,
            actions: &mut alloc::vec![],
        };

        CAT.borrow_ref_mut(cs).draw(&mut info);

        fb.render();

        actions
    });

    for action in actions {
        action();
    }

    false
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn oled_task_user_wasm(canvas: web_sys::HtmlCanvasElement) {
    let mut handler = with(|cs| INPUT_HANDLER.borrow_ref(cs).clone());

    let (actions, mut fb) = if Keyboard::is_right() {
        render_right()
    } else {
        render_left()
    };

    with(|cs| {
        let mut info = RenderInfo {
            framebuffer: &mut fb,
            cs,
            tick: TICK.load(Ordering::SeqCst),
            input: &mut handler,
            actions: &mut alloc::vec![],
        };

        CAT.borrow_ref_mut(cs).draw(&mut info);
    });

    fb.render(canvas);

    for action in actions {
        action();
    }
}

fn render_left() -> (Actions, Framebuffer) {
    let tick = TICK.load(Ordering::SeqCst);
    TICK.store(tick.wrapping_add(1), Ordering::SeqCst);
    let is_game_mode = Keyboard::layer_state_is(2);
    with(|cs| {
        let mut framebuffer = Framebuffer::default();
        let (actions, should_draw_border) = draw_screen(&mut framebuffer, cs);
        if is_game_mode {
            marquee("Game layer activated");
        } else {
            disable_marquee("Game layer activated");
        }
        let mut spring = MARQUEE_HEIGHT_SPRING.borrow_ref_mut(cs);
        spring.update();
        let current = spring.current();
        drop(spring);
        draw_marquee(&mut framebuffer, tick, current);
        if should_draw_border {
            draw_border(&mut framebuffer)
        };
        (actions, framebuffer)
    })
}

fn remap(x: f32, src_start: f32, src_end: f32, tgt_start: f32, tgt_end: f32) -> f32 {
    let src_range = src_end - src_start;
    if src_range == 0.0 {
        return src_start;
    }
    let tgt_range = tgt_end - tgt_start;
    (x - src_start) / src_range * tgt_range + tgt_start
}

fn remap_fixed(
    x: FixedNumber,
    src_start: FixedNumber,
    src_end: FixedNumber,
    tgt_start: FixedNumber,
    tgt_end: FixedNumber,
) -> FixedNumber {
    let src_range = src_end - src_start;
    if src_range.is_zero() {
        return src_start;
    }
    let tgt_range = tgt_end - tgt_start;
    (x - src_start) / src_range * tgt_range + tgt_start
}

pub fn draw_marquee(framebuffer: &mut Framebuffer, tick: u32, marquee_spring_y: f32) {
    with(|cs| {
        const MARQUEE_SPEED: u32 = 1;
        const MARQUEE_WIDTH: u16 = Screen::OLED_DISPLAY_WIDTH as u16;
        let text = MARQUEE_TEXT.borrow_ref(cs);

        let Some(text) = text.as_ref() else {
            return;
        };

        let text_width = text.len() as u16 * CHAR_WIDTH as u16;

        let span = text_width + MARQUEE_WIDTH;
        let offset = (tick.wrapping_mul(MARQUEE_SPEED) % span as u32) as i32;
        let x0 = MARQUEE_WIDTH as i32 - offset;

        let marquee_y = Screen::OLED_DISPLAY_HEIGHT as i32 - marquee_spring_y as i32;
        let text_y =
            (Screen::OLED_DISPLAY_HEIGHT as i32 + marquee_y) / 2 - (CHAR_HEIGHT as i32 / 2);

        framebuffer.fill_rect(0, marquee_y, MARQUEE_WIDTH, Screen::OLED_DISPLAY_HEIGHT);

        let mut text_framebuffer = Framebuffer::default();
        text_framebuffer.fill_rect(
            0,
            0,
            Screen::OLED_DISPLAY_WIDTH,
            Screen::OLED_DISPLAY_HEIGHT,
        );
        text_framebuffer.draw_text(x0, -1, text, true);
        let dither_progress = remap(marquee_spring_y, 0.0, MARQUEE_HEIGHT as f32, 20.0, -12.0);
        let scale_progress = remap(marquee_spring_y, 0.0, MARQUEE_HEIGHT as f32, 0.1, 0.0);

        if scale_progress != 0.0 {
            text_framebuffer.mode_7(
                |row| {
                    let row = row as f32;
                    Affine2::identity().origin(
                        FixedNumber::lit("32"),
                        FixedNumber::from_num(CHAR_HEIGHT / 2),
                        |affine| {
                            affine.scale(
                                FixedNumber::from_num((row * scale_progress) + 1.0),
                                FixedNumber::from_num((row * scale_progress) + 1.0),
                            )
                        },
                    )
                },
                true,
            );
        }

        text_framebuffer.dither(dither_progress.clamp(0.0, 8.0), true);
        framebuffer.draw_framebuffer_at(
            0,
            text_y,
            MARQUEE_WIDTH,
            Screen::OLED_DISPLAY_HEIGHT,
            &text_framebuffer.take_framebuffer(),
            FramebufferTransparency::IgnoreWhite,
        );
    });
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
            if let Ok(mut input_handler) = INPUT_HANDLER.borrow(cs).try_borrow_mut() {
                page.render(&mut RenderInfo {
                    framebuffer: &mut framebuffer,
                    cs,
                    tick: TICK.load(Ordering::SeqCst),
                    input: &mut input_handler,
                    actions: &mut actions,
                });
            };
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
    const BORDER_THICKNESS: i32 = 2;
    const BORDER_ROUNDING: i32 = 2;

    let marquee_spring_y = with(|cs| {
        let spring = MARQUEE_HEIGHT_SPRING.borrow_ref_mut(cs);
        spring.current() as i32
    });

    let height = Screen::OLED_DISPLAY_HEIGHT as i32 - marquee_spring_y;

    framebuffer.fill_rect(0, 0, Screen::OLED_DISPLAY_WIDTH as i32, BORDER_THICKNESS);
    framebuffer.fill_rect(0, 0, BORDER_THICKNESS, height);
    framebuffer.fill_rect(
        Screen::OLED_DISPLAY_WIDTH as i32 - BORDER_THICKNESS,
        0,
        BORDER_THICKNESS,
        height,
    );
    framebuffer.fill_rect(
        0,
        height - BORDER_THICKNESS,
        Screen::OLED_DISPLAY_WIDTH as i32,
        BORDER_THICKNESS,
    );

    const BORDER_ROUNDING_CALCULATED: i32 = BORDER_ROUNDING + (BORDER_THICKNESS * 2);

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
            Screen::OLED_DISPLAY_WIDTH as i32 - BORDER_ROUNDING_CALCULATED + offset,
            0,
            Screen::OLED_DISPLAY_WIDTH as i32,
            BORDER_ROUNDING_CALCULATED - offset,
        );
    }

    for offset in 0..BORDER_ROUNDING {
        framebuffer.draw_line(
            Screen::OLED_DISPLAY_WIDTH as i32 - BORDER_ROUNDING_CALCULATED + offset - 1,
            height,
            Screen::OLED_DISPLAY_WIDTH as i32,
            height - BORDER_ROUNDING_CALCULATED + offset - 1,
        );
    }

    for offset in 0..BORDER_ROUNDING {
        framebuffer.draw_line(
            0,
            height - BORDER_ROUNDING_CALCULATED + offset,
            BORDER_ROUNDING_CALCULATED - offset,
            height,
        );
    }
}

fn draw_screen(
    framebuffer: &mut Framebuffer,
    cs: CriticalSection,
) -> (Vec<Box<dyn FnOnce()>>, bool) {
    let tick = TICK.load(Ordering::SeqCst);
    let Ok(mut input) = INPUT_HANDLER.borrow(cs).try_borrow_mut() else {
        return (alloc::vec![], false);
    };
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
        let should_draw_border = if transition.render(&mut info) {
            let new_page = transition.take_page();
            let mut page = PAGE.borrow_ref_mut(cs);
            *page = new_page;
            drop(page);
            drop(input);
            drop(transitioning);
            IS_TRANSITIONING.store(false, Ordering::SeqCst);
            let (new_actions, should_draw_border) = draw_screen(framebuffer, cs);
            actions.extend(new_actions);
            should_draw_border
        } else {
            let should_draw_border = transition.page().should_draw_border();
            *transitioning = Some(transition);
            should_draw_border
        };

        return (actions, should_draw_border);
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
            PageTransition::None => Some(Box::new(NoneTransition::new(new_page))),
        };
        IS_TRANSITIONING.store(true, Ordering::SeqCst);
        drop(transitioning);
        let (new_actions, should_draw_border) = draw_screen(framebuffer, cs);
        actions.extend(new_actions);
        return (actions, should_draw_border);
    }

    let should_draw_border = page.should_draw_border();

    (actions, should_draw_border)
}

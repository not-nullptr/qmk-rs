use super::HomePage;
use crate::{
    image::{BOOT, CREDIT},
    page::{Page, RenderInfo},
    screen::{IS_TRANSITIONING, TICK},
    state::InputEvent,
};
use alloc::{boxed::Box, format};
use critical_section::with;
use qmk::{
    framebuffer::{Affine2, FixedNumber},
    screen::Screen,
};

pub struct Mode7Page {
    tick: u32,
    position: FixedNumber,
    rotation: FixedNumber,
}

impl Default for Mode7Page {
    fn default() -> Self {
        Self {
            tick: 0,
            position: FixedNumber::lit("0.1"),
            rotation: FixedNumber::lit("0.0"),
        }
    }
}

const CENTER_X: FixedNumber = FixedNumber::lit("32");
const CENTER_Y: FixedNumber = FixedNumber::lit("64");

impl Page for Mode7Page {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        while let Some(event) = renderer.input.poll() {
            // if let InputEvent::EncoderClick(_) = event {
            //     return Some(Box::new(HomePage::default()));
            // }

            match event {
                InputEvent::EncoderClick(_) => {
                    with(|cs| {
                        IS_TRANSITIONING.store(true, core::sync::atomic::Ordering::SeqCst);
                        TICK.store(0, core::sync::atomic::Ordering::SeqCst);
                    });
                    return Some(Box::new(HomePage::default()));
                }
                InputEvent::EncoderScroll(index, clockwise) => {
                    if index == 0 {
                        if clockwise {
                            self.position += FixedNumber::lit("0.01");
                        } else {
                            self.position -= FixedNumber::lit("0.01");
                        }
                    } else if clockwise {
                        self.rotation += FixedNumber::from_num(8.0f32.to_radians())
                    } else {
                        self.rotation -= FixedNumber::from_num(8.0f32.to_radians())
                    }
                }
                _ => {}
            }
        }

        let transitioning = IS_TRANSITIONING.load(core::sync::atomic::Ordering::SeqCst);
        if !transitioning {
            self.tick += 1;
        }

        if self.tick < 5 {
            return None;
        }

        let tick = FixedNumber::from_num((self.tick as f32).to_radians() % 360.0);

        renderer.framebuffer.draw_image_inverted(0, 0, &CREDIT);
        renderer.framebuffer.mode_7_optimized(|row| {
            let row = FixedNumber::from_num(row);
            Affine2::identity().origin(CENTER_X, CENTER_Y, |affine| {
                let scale = row * self.position;
                affine.scale(scale, scale).rotate(tick)
            })
        });

        // renderer
        //     .framebuffer
        //     .draw_text(4, 16, format!("H: {}", self.horizon), true);

        // renderer
        //     .framebuffer
        //     .draw_text(4, 24, format!("P: {}", self.perspective_constant), true);

        None
    }
}

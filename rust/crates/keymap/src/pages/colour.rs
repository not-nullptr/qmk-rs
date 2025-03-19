use super::HomePage;
use crate::{
    config::SETTINGS,
    image::{COLOUR_GRADIENT, LEFT_ARROW, RIGHT_ARROW},
    page::{Page, RenderInfo},
    state::InputEvent,
};
use alloc::boxed::Box;
#[cfg(not(test))]
use micromath::F32Ext;
use qmk::rgb::RGBLight;

fn map_color(value: u8) -> u8 {
    let fraction = value as f32 / 255.0;
    let mapped = (1.0 - fraction) * (105.0 - 24.0) + 24.0;
    mapped.round() as u8 - 4
}

#[derive(Default)]
pub struct ColourPage {
    hue: u8,
    sat: u8,
    val: u8,
}

impl Page for ColourPage {
    fn init(&mut self, renderer: &mut RenderInfo) {
        let settings = SETTINGS.borrow_ref(renderer.cs);
        self.hue = settings.hsv[0];
        self.sat = settings.hsv[1];
        self.val = settings.hsv[2];
    }

    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        while let Some(event) = renderer.input.poll() {
            match event {
                InputEvent::EncoderClick(i) => {
                    if i == 0 {
                        let mut settings = SETTINGS.borrow_ref_mut(renderer.cs);
                        settings.hsv[0] = self.hue;
                        settings.hsv[1] = self.sat;
                        settings.hsv[2] = self.val;
                        settings.save();
                        return Some(Box::new(HomePage::default()));
                    }
                }

                InputEvent::EncoderScroll(i, clockwise) => {
                    if i == 0 {
                        if renderer.input.right_encoder_down() {
                            if clockwise {
                                self.val = self.val.saturating_add(4);
                            } else {
                                self.val = self.val.saturating_sub(4);
                            }
                        } else {
                            if clockwise {
                                self.hue = self.hue.saturating_add(4);
                            } else {
                                self.hue = self.hue.saturating_sub(4);
                            }
                        }
                    } else if i == 1 {
                        if clockwise {
                            self.sat = self.sat.saturating_add(4);
                        } else {
                            self.sat = self.sat.saturating_sub(4);
                        }
                    }
                }
            }
        }

        renderer
            .framebuffer
            .draw_text_centered(32, 8, "Colour", false);

        renderer.framebuffer.draw_image(31, 20, &COLOUR_GRADIENT);
        renderer.framebuffer.draw_image(47, 20, &COLOUR_GRADIENT);

        let hue_pos = map_color(self.hue);
        renderer.framebuffer.draw_image(9, hue_pos, &RIGHT_ARROW);
        renderer
            .framebuffer
            .draw_image(9 + 12, hue_pos, &LEFT_ARROW);

        let sat_pos = map_color(self.sat);
        renderer.framebuffer.draw_image(26, sat_pos, &RIGHT_ARROW);
        renderer
            .framebuffer
            .draw_image(26 + 12, sat_pos, &LEFT_ARROW);

        let val_pos = map_color(self.val);
        renderer.framebuffer.draw_image(42, val_pos, &RIGHT_ARROW);
        renderer
            .framebuffer
            .draw_image(42 + 12, val_pos, &LEFT_ARROW);

        let (hue, sat, val) = (self.hue, self.sat, self.val);

        renderer.actions.push(Box::new(move || {
            RGBLight::set_hsv(hue, sat, val);
        }));

        None
    }
}

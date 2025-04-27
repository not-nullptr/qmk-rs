use super::HomePage;
use crate::{
    image::{BOOT, CREDIT, WHY},
    page::{Page, RenderInfo},
    screen::{IS_TRANSITIONING, TICK},
    state::InputEvent,
};
use alloc::{boxed::Box, format};
use critical_section::with;
use once_cell::sync::Lazy;
use qmk::{
    framebuffer::{Affine2, FixedNumber, TRIG_TABLE_F32},
    screen::Screen,
};

pub struct Mode7Page {
    tick: u32,
}

impl Default for Mode7Page {
    fn default() -> Self {
        Self { tick: 0 }
    }
}

trait Lerp<T> {
    fn lerp(self, from: T, to: T) -> T;
    fn inv_lerp(self, from: T, to: T) -> T;
    fn remap(self, from_range: (T, T), to_range: (T, T)) -> T;
}

macro_rules! lerp_impl {
    ($one:literal, $typ:ty) => {
        impl Lerp<$typ> for $typ {
            #[inline(always)]
            fn lerp(self, from: $typ, to: $typ) -> $typ {
                //! Linear interpolate on the scale given by a to b, using t as the point on that scale.
                ($one - self) * from + self * to
            }
            #[inline(always)]
            fn inv_lerp(self, from: $typ, to: $typ) -> $typ {
                //! Inverse Linar Interpolation, get the fraction between a and b on which v resides.
                (self - from) / (to - from)
            }
            #[inline(always)]
            fn remap(self, from_range: ($typ, $typ), to_range: ($typ, $typ)) -> $typ {
                //! Remap values from one linear scale to another, a combination of lerp and inv_lerp.
                //! i_range is the scale of the original value,
                //! o_range is the scale of the resulting value.
                self.inv_lerp(from_range.0, from_range.1)
                    .lerp(to_range.0, to_range.1)
            }
        }
    };
}

lerp_impl!(1., f32);

const CENTER_X: FixedNumber = FixedNumber::lit("32");
const CENTER_Y: FixedNumber = FixedNumber::lit("64");

impl Page for Mode7Page {
    fn render(&mut self, renderer: &mut RenderInfo) -> Option<Box<dyn Page>> {
        while let Some(event) = renderer.input.poll() {
            if let InputEvent::EncoderClick(_) = event {
                return Some(Box::new(HomePage::default()));
            }
        }

        self.tick += 1;

        renderer.framebuffer.draw_image(0, 0, &WHY);

        renderer.framebuffer.mode_7(
            |row| {
                // let sin = TRIG_TABLE_F32
                //     .sin((self.tick + row as u32) as f32 / 10.0)
                //     .abs()
                //     .remap((0.0, 1.0), (1.0, 1.25));
                // let sin = FixedNumber::from_num(sin);
                let row_sin = (row as f32).remap((0.0, 127.0), (0.0, 180.0)).to_radians();
                let row_sin = TRIG_TABLE_F32.sin(row_sin).remap((0.0, 1.0), (1.0, 1.5));
                let row_sin = FixedNumber::from_num(row_sin);

                Affine2::identity()
                    .origin(CENTER_X, CENTER_Y, |affine| affine.scale(row_sin, row_sin))
                    .origin(CENTER_X, FixedNumber::ZERO, |affine| {
                        affine.rotate(FixedNumber::from_num(
                            (TRIG_TABLE_F32.sin(self.tick as f32 / 7.0)
                                * ((128.0 - row as f32) / 4.0))
                                .to_radians(),
                        ))
                    })
                    .translate(
                        FixedNumber::ZERO,
                        FixedNumber::from_num(
                            TRIG_TABLE_F32.sin(self.tick as f32 / 7.0).abs() * -32.0,
                        ),
                    )
            },
            false,
        );

        None
    }
}

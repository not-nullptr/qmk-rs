use crate::screen::Screen;
use crate::{alloc::string::ToString, qmk_log};
use alloc::{string::String, vec::Vec};
use ape_table_trig::{TrigTableF32, trig_table_gen_f32};
use core::hint::black_box;
use core::sync::atomic::AtomicU32;
use core::{fmt::Display, ops::Mul};
use fixed::FixedI32;
use fixed::types::extra::{U8, U16};
use fixed::{FixedI16, types::extra::U7};
use include_image::QmkImage;
use num_traits::{Num, ToPrimitive};
use once_cell::sync::Lazy;

macro_rules! set_pixel {
    ($fb:expr, $x:expr, $y:expr) => {
        $fb[$x + ($y / 8) * Screen::OLED_DISPLAY_WIDTH] |= 1 << ($y % 8);
    };
}

macro_rules! clear_pixel {
    ($fb:expr, $x:expr, $y:expr) => {
        $fb[$x + ($y / 8) * Screen::OLED_DISPLAY_WIDTH] &= !(1 << ($y % 8));
    };
}

macro_rules! get_pixel {
    ($fb:expr, $x:expr, $y:expr) => {
        ($fb[$x + ($y / 8) * Screen::OLED_DISPLAY_WIDTH] >> ($y % 8)) & 1 == 1
    };
}

const TABLE: [f32; 256] = trig_table_gen_f32!(256);

pub static TRIG_TABLE_F32: Lazy<TrigTableF32> = Lazy::new(|| TrigTableF32::new(&TABLE));

pub type FixedNumber = FixedI16<U7>;

#[derive(Clone, Copy, Debug)]
pub struct Affine2 {
    pub m00: FixedNumber,
    pub m01: FixedNumber,
    pub m10: FixedNumber,
    pub m11: FixedNumber,
    pub tx: FixedNumber,
    pub ty: FixedNumber,
}

impl Affine2 {
    /// Returns the identity matrix. Use this as a starting point.
    pub const fn identity() -> Self {
        Self {
            m00: FixedNumber::ONE,
            m01: FixedNumber::ZERO,
            m10: FixedNumber::ZERO,
            m11: FixedNumber::ONE,
            tx: FixedNumber::ZERO,
            ty: FixedNumber::ZERO,
        }
    }

    pub fn inverse(&self) -> Option<Affine2> {
        // Compute determinant
        let det = self.m00 * self.m11 - self.m01 * self.m10;
        if det.abs() < f32::EPSILON {
            // Not invertible
            return None;
        }
        let inv_det = FixedNumber::ONE / det;

        let m00 = self.m11 * inv_det;
        let m01 = -self.m01 * inv_det;
        let m10 = -self.m10 * inv_det;
        let m11 = self.m00 * inv_det;

        let tx = -(m00 * self.tx + m01 * self.ty);
        let ty = -(m10 * self.tx + m11 * self.ty);

        Some(Affine2 {
            m00,
            m01,
            m10,
            m11,
            tx,
            ty,
        })
    }

    /// Performs a series of transformations on the matrix about the given origin.
    /// Usage: Affine2::identity().origin(ox, oy, Affine2::identity().rotate(a).scale(sx, sy))
    pub fn origin<F>(self, x: FixedNumber, y: FixedNumber, affine_fn: F) -> Self
    where
        F: FnOnce(Affine2) -> Affine2,
    {
        let affine = self.translate(-x, -y);
        let affine = affine_fn(affine);
        affine.translate(x, y)
    }

    /// Rotates the matrix by angle (radians)
    pub fn rotate(self, angle: FixedNumber) -> Self {
        let (s, c) = (
            TRIG_TABLE_F32.sin(angle.to_num()),
            TRIG_TABLE_F32.cos(angle.to_num()),
        );
        let (s, c) = (
            FixedNumber::saturating_from_num(s),
            FixedNumber::saturating_from_num(c),
        );
        let rot = Self {
            m00: c,
            m01: -s,
            m10: s,
            m11: c,
            tx: FixedNumber::ZERO,
            ty: FixedNumber::ZERO,
        };
        rot.mul(self)
    }

    /// Scales the matrix
    pub fn scale(self, sx: FixedNumber, sy: FixedNumber) -> Self {
        let scl = Self {
            m00: sx,
            m01: FixedNumber::ZERO,
            m10: FixedNumber::ZERO,
            m11: sy,
            tx: FixedNumber::ZERO,
            ty: FixedNumber::ZERO,
        };
        scl.mul(self)
    }

    /// Translates the matrix
    pub fn translate(self, dx: FixedNumber, dy: FixedNumber) -> Self {
        let tr = Self {
            m00: FixedNumber::ONE,
            m01: FixedNumber::ZERO,
            m10: FixedNumber::ZERO,
            m11: FixedNumber::ONE,
            tx: dx,
            ty: dy,
        };
        tr.mul(self)
    }

    /// Calculates the given coordinate transformed by our matrix.
    pub fn transform_point(&self, x: FixedNumber, y: FixedNumber) -> (FixedNumber, FixedNumber) {
        (
            self.m00 * x + self.m01 * y + self.tx,
            self.m10 * x + self.m11 * y + self.ty,
        )
    }
}

static TICK: AtomicU32 = AtomicU32::new(0);

impl Mul for Affine2 {
    type Output = Self;

    //
    fn mul(self, other: Self) -> Self {
        Self {
            m00: self.m00 * other.m00 + self.m01 * other.m10,
            m01: self.m00 * other.m01 + self.m01 * other.m11,
            m10: self.m10 * other.m00 + self.m11 * other.m10,
            m11: self.m10 * other.m01 + self.m11 * other.m11,
            tx: self.m00 * other.tx + self.m01 * other.ty + self.tx,
            ty: self.m10 * other.tx + self.m11 * other.ty + self.ty,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum FramebufferTransparency {
    None,
    IgnoreBlack,
    IgnoreWhite,
}

const FONTPLATE: [u8; 1344] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x6A, 0x52, 0x6A, 0x3C, 0x00, 0x3C, 0x6A, 0x4E, 0x6A,
    0x3C, 0x00, 0x18, 0x3C, 0x78, 0x3C, 0x18, 0x00, 0x10, 0x38, 0x7C, 0x38, 0x10, 0x00, 0x18, 0x5E,
    0x6E, 0x5E, 0x18, 0x00, 0x30, 0x38, 0x5C, 0x38, 0x30, 0x00, 0x00, 0x18, 0x3C, 0x3C, 0x18, 0x00,
    0xFF, 0xE7, 0xC3, 0xC3, 0xE7, 0xFF, 0x38, 0x44, 0x44, 0x44, 0x38, 0x00, 0xC7, 0xBB, 0xBB, 0xBB,
    0xC7, 0xFF, 0x0C, 0x22, 0x72, 0x22, 0x0C, 0x00, 0x30, 0x4A, 0x46, 0x4E, 0x30, 0x00, 0x30, 0x4A,
    0xE6, 0x4E, 0x30, 0x00, 0x60, 0x7C, 0x0C, 0x6C, 0x7C, 0x00, 0x54, 0x38, 0x6C, 0x38, 0x54, 0x00,
    0x7C, 0x7C, 0x38, 0x10, 0x00, 0x00, 0x00, 0x10, 0x38, 0x7C, 0x7C, 0x00, 0x24, 0x66, 0xFF, 0x66,
    0x24, 0x00, 0x00, 0x5E, 0x00, 0x5E, 0x00, 0x00, 0x0C, 0x12, 0x7E, 0x02, 0x7E, 0x00, 0x1C, 0x5A,
    0x5A, 0x5A, 0x38, 0x00, 0x60, 0x60, 0x60, 0x60, 0x60, 0x00, 0x94, 0xB6, 0xFF, 0xB6, 0x94, 0x00,
    0x08, 0x7C, 0x7E, 0x7C, 0x08, 0x00, 0x10, 0x3E, 0x7E, 0x3E, 0x10, 0x00, 0x38, 0x38, 0x7C, 0x38,
    0x10, 0x00, 0x10, 0x38, 0x7C, 0x38, 0x38, 0x00, 0x38, 0x20, 0x20, 0x20, 0x20, 0x00, 0x1C, 0x3E,
    0x08, 0x3E, 0x1C, 0x00, 0x60, 0x70, 0x78, 0x70, 0x60, 0x00, 0x06, 0x0E, 0x1E, 0x0E, 0x06, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6E, 0x6E, 0x00, 0x00, 0x00, 0x06, 0x0E, 0x00, 0x0E,
    0x06, 0x00, 0x14, 0x3E, 0x14, 0x3E, 0x14, 0x00, 0x58, 0x54, 0x7E, 0x54, 0x34, 0x00, 0x66, 0x36,
    0x18, 0x6C, 0x66, 0x00, 0x34, 0x4A, 0x5A, 0x34, 0x50, 0x00, 0x00, 0x08, 0x0E, 0x06, 0x00, 0x00,
    0x00, 0x3C, 0x66, 0x42, 0x00, 0x00, 0x00, 0x42, 0x66, 0x3C, 0x00, 0x00, 0x2A, 0x1C, 0x3E, 0x1C,
    0x2A, 0x00, 0x00, 0x08, 0x1C, 0x08, 0x00, 0x00, 0x00, 0x80, 0xE0, 0x60, 0x00, 0x00, 0x08, 0x08,
    0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x60, 0x60, 0x00, 0x00, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x00,
    0x38, 0x44, 0x44, 0x44, 0x38, 0x00, 0x00, 0x44, 0x7C, 0x40, 0x00, 0x00, 0x74, 0x54, 0x54, 0x54,
    0x5C, 0x00, 0x54, 0x54, 0x54, 0x54, 0x7C, 0x00, 0x30, 0x28, 0x24, 0x7C, 0x20, 0x00, 0x5C, 0x54,
    0x54, 0x54, 0x74, 0x00, 0x7C, 0x54, 0x54, 0x54, 0x74, 0x00, 0x04, 0x44, 0x24, 0x14, 0x0C, 0x00,
    0x7C, 0x54, 0x54, 0x54, 0x7C, 0x00, 0x5C, 0x54, 0x54, 0x54, 0x7C, 0x00, 0x00, 0x00, 0x6C, 0x6C,
    0x00, 0x00, 0x00, 0x80, 0xEC, 0x6C, 0x00, 0x00, 0x10, 0x38, 0x6C, 0x44, 0x00, 0x00, 0x28, 0x28,
    0x28, 0x28, 0x28, 0x00, 0x00, 0x44, 0x6C, 0x38, 0x10, 0x00, 0x04, 0x02, 0x52, 0x12, 0x0C, 0x00,
    0x3C, 0x42, 0x5A, 0x5A, 0x1C, 0x00, 0x7C, 0x14, 0x14, 0x14, 0x7C, 0x00, 0x7C, 0x54, 0x54, 0x54,
    0x38, 0x00, 0x7C, 0x44, 0x44, 0x44, 0x44, 0x00, 0x7C, 0x44, 0x44, 0x44, 0x38, 0x00, 0x7C, 0x54,
    0x54, 0x54, 0x54, 0x00, 0x7C, 0x14, 0x14, 0x14, 0x14, 0x00, 0x7C, 0x44, 0x54, 0x54, 0x74, 0x00,
    0x7C, 0x10, 0x10, 0x10, 0x7C, 0x00, 0x00, 0x44, 0x7C, 0x44, 0x00, 0x20, 0x40, 0x44, 0x44, 0x3C,
    0x00, 0x00, 0x7C, 0x10, 0x10, 0x28, 0x44, 0x00, 0x7C, 0x40, 0x40, 0x40, 0x40, 0x00, 0x7C, 0x08,
    0x10, 0x08, 0x7C, 0x00, 0x7C, 0x08, 0x10, 0x20, 0x7C, 0x00, 0x7C, 0x44, 0x44, 0x44, 0x7C, 0x00,
    0x7C, 0x14, 0x14, 0x14, 0x1C, 0x00, 0x7C, 0x44, 0x44, 0x64, 0x7C, 0x00, 0x7C, 0x14, 0x14, 0x74,
    0x5C, 0x00, 0x5C, 0x54, 0x54, 0x54, 0x74, 0x00, 0x04, 0x04, 0x7C, 0x04, 0x04, 0x00, 0x7C, 0x40,
    0x40, 0x40, 0x7C, 0x00, 0x1C, 0x20, 0x40, 0x20, 0x1C, 0x00, 0x3C, 0x40, 0x3C, 0x40, 0x3C, 0x00,
    0x44, 0x28, 0x10, 0x28, 0x44, 0x00, 0x0C, 0x10, 0x60, 0x10, 0x0C, 0x00, 0x44, 0x64, 0x54, 0x4C,
    0x44, 0x00, 0x00, 0x7E, 0x42, 0x42, 0x00, 0x00, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x00, 0x00, 0x42,
    0x42, 0x7E, 0x00, 0x00, 0x04, 0x06, 0x03, 0x06, 0x04, 0x00, 0x40, 0x40, 0x40, 0x40, 0x40, 0x00,
    0x00, 0x03, 0x07, 0x04, 0x00, 0x00, 0x7C, 0x14, 0x14, 0x14, 0x7C, 0x00, 0x7C, 0x54, 0x54, 0x54,
    0x38, 0x00, 0x7C, 0x44, 0x44, 0x44, 0x44, 0x00, 0x7C, 0x44, 0x44, 0x44, 0x38, 0x00, 0x7C, 0x54,
    0x54, 0x54, 0x54, 0x00, 0x7C, 0x14, 0x14, 0x14, 0x14, 0x00, 0x7C, 0x44, 0x54, 0x54, 0x74, 0x00,
    0x7C, 0x10, 0x10, 0x10, 0x7C, 0x00, 0x00, 0x44, 0x7C, 0x44, 0x00, 0x20, 0x40, 0x44, 0x44, 0x3C,
    0x00, 0x00, 0x7C, 0x10, 0x10, 0x28, 0x44, 0x00, 0x7C, 0x40, 0x40, 0x40, 0x40, 0x00, 0x7C, 0x08,
    0x10, 0x08, 0x7C, 0x00, 0x7C, 0x08, 0x10, 0x20, 0x7C, 0x00, 0x7C, 0x44, 0x44, 0x44, 0x7C, 0x00,
    0x7C, 0x14, 0x14, 0x14, 0x1C, 0x00, 0x7C, 0x44, 0x44, 0x64, 0x7C, 0x00, 0x7C, 0x14, 0x14, 0x74,
    0x5C, 0x00, 0x5C, 0x54, 0x54, 0x54, 0x74, 0x00, 0x04, 0x04, 0x7C, 0x04, 0x04, 0x00, 0x7C, 0x40,
    0x40, 0x40, 0x7C, 0x00, 0x1C, 0x20, 0x40, 0x20, 0x1C, 0x00, 0x3C, 0x40, 0x3C, 0x40, 0x3C, 0x00,
    0x44, 0x28, 0x10, 0x28, 0x44, 0x00, 0x0C, 0x10, 0x60, 0x10, 0x0C, 0x00, 0x44, 0x64, 0x54, 0x4C,
    0x44, 0x00, 0x00, 0x18, 0x66, 0x42, 0x00, 0x00, 0x00, 0x00, 0x66, 0x00, 0x00, 0x00, 0x00, 0x42,
    0x66, 0x18, 0x00, 0x00, 0x0C, 0x06, 0x06, 0x0C, 0x06, 0x00, 0x70, 0x48, 0x44, 0x48, 0x70, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const FONT_WIDTH: usize = 192;
const FONT_HEIGHT: usize = 56;
pub const CHAR_WIDTH: usize = 6;
pub const CHAR_HEIGHT: usize = 8;
pub const CHAR_SIZE: usize = CHAR_WIDTH * CHAR_HEIGHT;
pub const CHAR_ROWS: usize = FONT_WIDTH / CHAR_WIDTH;
pub const CHAR_COLS: usize = FONT_HEIGHT / CHAR_HEIGHT;

#[cfg(not(target_arch = "wasm32"))]
unsafe extern "C" {
    static mut oled_buffer: [u8; Screen::OLED_DISPLAY_SIZE];
    static mut oled_dirty: u16;
}

#[cfg(not(target_arch = "wasm32"))]
const ALL_DIRTY: u16 = (((1 << (16 - 1)) - 1) << 1) | 1;

type FramebufferArray = [u8; Screen::OLED_DISPLAY_SIZE];

pub struct Framebuffer {
    framebuffer: FramebufferArray,
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self {
            framebuffer: [0; Screen::OLED_DISPLAY_SIZE],
        }
    }
}

impl Framebuffer {
    pub fn from_array(framebuffer: FramebufferArray) -> Self {
        Self { framebuffer }
    }

    pub fn take_framebuffer(self) -> FramebufferArray {
        self.framebuffer
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn render(self) {
        unsafe {
            #[allow(static_mut_refs)]
            core::ptr::write(&raw mut oled_buffer, self.framebuffer);
            oled_dirty = ALL_DIRTY;
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn render(&self, canvas: web_sys::HtmlCanvasElement) {
        use web_sys::wasm_bindgen::JsCast;

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // clear as black
        ctx.set_fill_style_str("black");
        ctx.fill_rect(
            0.0,
            0.0,
            Screen::OLED_DISPLAY_WIDTH as f64,
            Screen::OLED_DISPLAY_HEIGHT as f64,
        );
        ctx.set_fill_style_str("white");
        for y in 0..Screen::OLED_DISPLAY_HEIGHT {
            for x in 0..Screen::OLED_DISPLAY_WIDTH {
                if self.get_pixel(x, y) {
                    ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
                }
            }
        }
    }

    pub fn get_pixel<T, U>(&self, x: T, y: U) -> bool
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let x = x.to_u8().unwrap_or(255);
        let y = y.to_u8().unwrap_or(255);
        if x >= Screen::OLED_DISPLAY_WIDTH as u8 || y >= Screen::OLED_DISPLAY_HEIGHT as u8 {
            return false;
        }

        let byte_index = (x as usize) + ((y as usize) / 8) * Screen::OLED_DISPLAY_WIDTH;
        let bit_position = y % 8;

        self.framebuffer[byte_index] & (1 << bit_position) != 0
    }

    pub fn draw_pixel<T, U>(&mut self, x: T, y: U)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let x = x.to_u8().unwrap_or(255);
        let y = y.to_u8().unwrap_or(255);
        if x >= Screen::OLED_DISPLAY_WIDTH as u8 || y >= Screen::OLED_DISPLAY_HEIGHT as u8 {
            return;
        }

        let byte_index = (x as usize) + ((y as usize) / 8) * Screen::OLED_DISPLAY_WIDTH;
        let bit_position = y % 8;

        self.framebuffer[byte_index] |= 1 << bit_position;
    }

    pub fn clear_pixel<T, U>(&mut self, x: T, y: U)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let x = x.to_u8().unwrap_or(255);
        let y = y.to_u8().unwrap_or(255);

        if x >= Screen::OLED_DISPLAY_WIDTH as u8 || y >= Screen::OLED_DISPLAY_HEIGHT as u8 {
            return;
        }

        let byte_index = (x as usize) + ((y as usize) / 8) * Screen::OLED_DISPLAY_WIDTH;
        let bit_position = y % 8;

        self.framebuffer[byte_index] &= !(1 << bit_position);
    }

    pub fn affine(&mut self, affine: Affine2, clear_with_white: bool) {
        let width = Screen::OLED_DISPLAY_WIDTH;
        let height = Screen::OLED_DISPLAY_HEIGHT;

        let original_fb = Framebuffer::from_array(self.framebuffer);

        for y in 0..height {
            for x in 0..width {
                // clear_pixel!(self.framebuffer, x, y);
                if clear_with_white {
                    set_pixel!(self.framebuffer, x, y);
                } else {
                    clear_pixel!(self.framebuffer, x, y);
                }
                let fixed_x = FixedNumber::from_num(x);
                let fixed_y = FixedNumber::from_num(y);
                let (src_x, src_y) = affine.transform_point(fixed_x, fixed_y);

                let src_x = src_x.to_num::<isize>();
                let src_y = src_y.to_num::<isize>();

                let Ok(src_x) = TryInto::<usize>::try_into(src_x) else {
                    continue;
                };

                let Ok(src_y) = TryInto::<usize>::try_into(src_y) else {
                    continue;
                };

                if src_x >= width {
                    continue;
                }
                if src_y >= height {
                    continue;
                }

                if get_pixel!(original_fb.framebuffer, src_x, src_y) {
                    set_pixel!(self.framebuffer, x, y);
                } else {
                    clear_pixel!(self.framebuffer, x, y);
                }
            }
        }
    }

    pub fn mode_7<F>(&mut self, affine_function: F, clear_with_white: bool)
    where
        F: Fn(u8) -> Affine2,
    {
        let width = Screen::OLED_DISPLAY_WIDTH;
        let height = Screen::OLED_DISPLAY_HEIGHT;

        let original_fb = Framebuffer::from_array(self.framebuffer);

        for y in 0..height {
            let Some(affine) = affine_function(y as u8).inverse() else {
                continue;
            };
            let fixed_y = FixedNumber::from_num(y);
            for x in 0..width {
                // clear_pixel!(self.framebuffer, x, y);
                if clear_with_white {
                    set_pixel!(self.framebuffer, x, y);
                } else {
                    clear_pixel!(self.framebuffer, x, y);
                }

                let fixed_x = FixedNumber::from_num(x);
                let (src_x, src_y) = affine.transform_point(fixed_x, fixed_y);

                let src_x = src_x.to_num::<isize>();
                let src_y = src_y.to_num::<isize>();

                let Ok(src_x) = TryInto::<usize>::try_into(src_x) else {
                    continue;
                };

                let Ok(src_y) = TryInto::<usize>::try_into(src_y) else {
                    continue;
                };

                if src_x >= width {
                    continue;
                }
                if src_y >= height {
                    continue;
                }

                if get_pixel!(original_fb.framebuffer, src_x, src_y) {
                    set_pixel!(self.framebuffer, x, y);
                } else {
                    clear_pixel!(self.framebuffer, x, y);
                }
            }
        }
    }

    pub fn draw_char<T, U>(&mut self, x: T, y: U, ch: char, inverted: bool, transparent: bool)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let offset_x = x.to_i32().unwrap_or(255);
        let offset_y = y.to_i32().unwrap_or(255);

        let ascii_code = ch as usize;
        if ascii_code >= (CHAR_ROWS * CHAR_COLS) {
            return;
        }

        let cell_x = (ascii_code % CHAR_ROWS) * CHAR_WIDTH;
        let cell_y = (ascii_code / CHAR_ROWS) * CHAR_HEIGHT;
        let font_row = cell_y / 8;

        for cx in 0..CHAR_WIDTH {
            let font_index = (cell_x + cx) + font_row * FONT_WIDTH;
            let font_column = FONTPLATE[font_index];

            for bit in 0..CHAR_HEIGHT {
                if inverted {
                    if font_column & (1 << bit) != 0 {
                        self.clear_pixel(offset_x + cx as i32, offset_y + bit as i32);
                    } else if !transparent {
                        self.draw_pixel(offset_x + cx as i32, offset_y + bit as i32);
                    }
                } else if font_column & (1 << bit) != 0 {
                    self.draw_pixel(offset_x + cx as i32, offset_y + bit as i32);
                } else if !transparent {
                    self.clear_pixel(offset_x + cx as i32, offset_y + bit as i32);
                }
            }
        }
    }

    pub fn scale_around<T, U, V, W>(&mut self, x: T, y: U, width: V, height: W)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
        V: Num + ToPrimitive,
        W: Num + ToPrimitive,
    {
        let center = (x.to_i32().unwrap_or(0), y.to_i32().unwrap_or(0));
        let new_width = width.to_i32().unwrap_or(0);
        let new_height = height.to_i32().unwrap_or(0);
        let cloned_fb = Framebuffer::from_array(self.framebuffer);

        type Decimal = FixedI16<U7>;
        const ZERO: Decimal = Decimal::lit("0.0");
        const ONE: Decimal = Decimal::lit("1.0");

        let center_x = Decimal::saturating_from_num(center.0);
        let center_y = Decimal::saturating_from_num(center.1);
        let new_width = Decimal::saturating_from_num(new_width);
        let new_height = Decimal::saturating_from_num(new_height);
        let width = Decimal::saturating_from_num(Screen::OLED_DISPLAY_WIDTH as i32);
        let height = Decimal::saturating_from_num(Screen::OLED_DISPLAY_HEIGHT as i32);
        let scale_x = new_width / width;
        let scale_y = new_height / height;

        let mut dst_y = ZERO;
        let mut dst_x = ZERO;

        while dst_y < height {
            while dst_x < width {
                let rel_x = dst_x - center_x;
                let rel_y = dst_y - center_y;

                let src_x = (rel_x * scale_x + center_x).to_num::<i32>();
                let src_y = (rel_y * scale_y + center_y).to_num::<i32>();
                if src_x < 0 || src_x >= Screen::OLED_DISPLAY_WIDTH as i32 {
                    dst_x += ONE;
                    continue;
                }
                if src_y < 0 || src_y >= Screen::OLED_DISPLAY_HEIGHT as i32 {
                    dst_x += ONE;
                    continue;
                }

                {
                    let dst_x = dst_x.to_num::<i32>();
                    let dst_y = dst_y.to_num::<i32>();

                    if cloned_fb.get_pixel(src_x, src_y) {
                        self.draw_pixel(dst_x as u8, dst_y as u8);
                    } else {
                        self.clear_pixel(dst_x as u8, dst_y as u8);
                    }
                }

                dst_x += ONE;
            }
            dst_x = ZERO;
            dst_y += ONE;
        }
    }

    pub fn draw_text<T, U>(&mut self, x: T, y: U, text: impl Display, inverted: bool)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let offset_x = x.to_i32().unwrap_or(255);
        let offset_y = y.to_i32().unwrap_or(255);

        let text = text.to_string();
        for (i, ch) in text.chars().enumerate() {
            self.draw_char(
                offset_x + (i * CHAR_WIDTH) as i32,
                offset_y,
                ch,
                inverted,
                false,
            );
        }
    }

    pub fn draw_text_transparent<T, U>(&mut self, x: T, y: U, text: impl Display, inverted: bool)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let offset_x = x.to_i32().unwrap_or(255);
        let offset_y = y.to_i32().unwrap_or(255);

        let text = text.to_string();
        for (i, ch) in text.chars().enumerate() {
            self.draw_char(
                offset_x + (i * CHAR_WIDTH) as i32,
                offset_y,
                ch,
                inverted,
                true,
            );
        }
    }

    pub fn draw_text_centered<T, U>(&mut self, x: T, y: U, text: impl Into<String>, inverted: bool)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let offset_y = y.to_u8().unwrap_or(255);
        let text = text.into();
        let text_length = text.chars().count() as u8;
        let offset_x = x.to_u8().unwrap_or(255);
        let offset_x = offset_x - (text_length * CHAR_WIDTH as u8) / 2;
        self.draw_text(offset_x, offset_y, text, inverted);
    }

    pub fn draw_line<T, U, V, X>(&mut self, x0: T, y0: U, x1: V, y1: X)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
        V: Num + ToPrimitive,
        X: Num + ToPrimitive,
    {
        let x0 = x0.to_i16().unwrap_or(255);
        let y0 = y0.to_i16().unwrap_or(255);
        let x1 = x1.to_i16().unwrap_or(255);
        let y1 = y1.to_i16().unwrap_or(255);

        let (mut x0, mut y0) = (x0, y0);
        let (x1, y1) = (x1, y1);

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.draw_pixel(x0 as u8, y0 as u8);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn fill_rect<X, Y, W, H>(&mut self, x: X, y: Y, width: W, height: H)
    where
        X: Num + ToPrimitive,
        Y: Num + ToPrimitive,
        W: Num + ToPrimitive,
        H: Num + ToPrimitive,
    {
        let x = x.to_i16().unwrap_or(255);
        let y = y.to_i16().unwrap_or(255);
        let width = width.to_u8().unwrap_or(255);
        let height = height.to_u8().unwrap_or(255);

        for i in 0..width {
            for j in 0..height {
                let ia16 = i as i16;
                let ja16 = j as i16;
                if x + ia16 < 0 || x + ia16 >= Screen::OLED_DISPLAY_WIDTH as i16 {
                    continue;
                }
                if y + ja16 < 0 || y + ja16 >= Screen::OLED_DISPLAY_HEIGHT as i16 {
                    continue;
                }
                let x = x as u8;
                let y = y as u8;
                self.draw_pixel(x + i, y + j);
            }
        }
    }

    pub fn draw_framebuffer<T, U>(&mut self, x: T, y: U, framebuffer: FramebufferArray)
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let offset_x = x.to_u8().unwrap_or(255);
        let offset_y = y.to_u8().unwrap_or(255);

        let src_width = Screen::OLED_DISPLAY_WIDTH;
        let num_byte_rows = framebuffer.len() / src_width;

        for src_x in 0..src_width {
            for byte_row in 0..num_byte_rows {
                let byte = framebuffer[src_x + byte_row * src_width];
                for bit in 0..8 {
                    let dest_pixel_x = offset_x as usize + src_x;
                    let dest_pixel_y = offset_y as usize + (byte_row * 8) + bit;
                    if dest_pixel_x < Screen::OLED_DISPLAY_WIDTH
                        && dest_pixel_y < Screen::OLED_DISPLAY_HEIGHT
                    {
                        let dest_byte_index =
                            dest_pixel_x + (dest_pixel_y / 8) * Screen::OLED_DISPLAY_WIDTH;
                        let dest_bit = dest_pixel_y % 8;
                        if byte & (1 << bit) != 0 {
                            self.framebuffer[dest_byte_index] |= 1 << dest_bit;
                        } else {
                            self.framebuffer[dest_byte_index] &= !(1 << dest_bit);
                        }
                    }
                }
            }
        }
    }

    pub fn get_framebuffer_at<T, U, V, W>(&self, x: T, y: U, width: V, height: W) -> Vec<u8>
    where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
        V: Num + ToPrimitive,
        W: Num + ToPrimitive,
    {
        let x = x.to_u8().unwrap_or(255);
        let y = y.to_u8().unwrap_or(255);
        let width = width.to_u8().unwrap_or(255);
        let height = height.to_u8().unwrap_or(255);

        let x = x as usize;
        let y = y as usize;
        let width = width as usize;
        let height = height as usize;

        let byte_rows = height.div_ceil(8);
        let mut out = alloc::vec![0u8; width * byte_rows];

        for col in 0..width {
            for byte_row in 0..byte_rows {
                let mut byte: u8 = 0;
                for bit in 0..8 {
                    let pixel_y = y + byte_row * 8 + bit;
                    if pixel_y < y + height
                        && col + x < Screen::OLED_DISPLAY_WIDTH
                        && pixel_y < Screen::OLED_DISPLAY_HEIGHT
                    {
                        let fb_index = (x + col) + ((pixel_y) / 8) * Screen::OLED_DISPLAY_WIDTH;
                        if self.framebuffer[fb_index] & (1 << (pixel_y % 8)) != 0 {
                            byte |= 1 << bit;
                        }
                    }
                }
                out[col + byte_row * width] = byte;
            }
        }
        out
    }

    pub fn draw_framebuffer_at<T, U, V, W>(
        &mut self,
        x: T,
        y: U,
        width: V,
        height: W,
        source: &[u8],
        transparency: FramebufferTransparency,
    ) where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
        V: Num + ToPrimitive,
        W: Num + ToPrimitive,
    {
        let x = x.to_i32().unwrap_or(255);
        let y = y.to_i32().unwrap_or(255);
        let width = width.to_i32().unwrap_or(255);
        let height = height.to_i32().unwrap_or(255);

        let x = x as usize;
        let y = y as usize;
        let width = width as usize;
        let height = height as usize;

        let byte_rows = height.div_ceil(8);

        for col in 0..width {
            for byte_row in 0..byte_rows {
                let byte = source[col + byte_row * width];
                for bit in 0..8 {
                    let dest_y = y + byte_row * 8 + bit;
                    let dest_x = x + col;
                    if dest_y < y + height
                        && dest_x < Screen::OLED_DISPLAY_WIDTH
                        && dest_y < Screen::OLED_DISPLAY_HEIGHT
                    {
                        let fb_index = dest_x + (dest_y / 8) * Screen::OLED_DISPLAY_WIDTH;
                        if byte & (1 << bit) != 0 {
                            if transparency != FramebufferTransparency::IgnoreWhite {
                                self.framebuffer[fb_index] |= 1 << (dest_y % 8);
                            }
                        } else if transparency != FramebufferTransparency::IgnoreBlack {
                            self.framebuffer[fb_index] &= !(1 << (dest_y % 8));
                        }
                    }
                }
            }
        }
    }

    pub fn draw_image<T, U, const M: usize>(
        &mut self,
        offset_x: T,
        offset_y: U,
        image: &QmkImage<M>,
    ) where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let offset_x = offset_x.to_u8().unwrap_or(255);
        let offset_y = offset_y.to_u8().unwrap_or(255);

        let img_pages = (image.height as usize).div_ceil(8);
        let display_width = Screen::OLED_DISPLAY_WIDTH;

        for x in 0..image.width as usize {
            for y in 0..image.height as usize {
                let src_page = y / 8;
                let src_bit = 7 - (y % 8);
                let src_index = x * img_pages + src_page;

                let pixel_on = (image.bytes[src_index] >> src_bit) & 1;

                let dest_x = offset_x as usize + x;
                let dest_y = offset_y as usize + y;
                let dest_page = dest_y / 8;
                let dest_bit = dest_y % 8;
                let dest_index = dest_page * display_width + dest_x;

                if dest_index < self.framebuffer.len() && dest_x < display_width {
                    if pixel_on == 1 {
                        self.framebuffer[dest_index] |= 1 << dest_bit;
                    } else {
                        self.framebuffer[dest_index] &= !(1 << dest_bit);
                    }
                }
            }
        }
    }

    pub fn draw_image_inverted<T, U, const M: usize>(
        &mut self,
        offset_x: T,
        offset_y: U,
        image: &QmkImage<M>,
    ) where
        T: Num + ToPrimitive,
        U: Num + ToPrimitive,
    {
        let offset_x = offset_x.to_i16().unwrap_or(0);
        let offset_y = offset_y.to_i16().unwrap_or(0);

        let img_pages = (image.height as usize).div_ceil(8);
        let display_width = Screen::OLED_DISPLAY_WIDTH;

        for x in 0..image.width as usize {
            for y in 0..image.height as usize {
                let src_page = y / 8;
                let src_bit = 7 - (y % 8);
                let src_index = x * img_pages + src_page;

                let pixel_on = (image.bytes[src_index] >> src_bit) & 1;

                let dest_x = offset_x as usize + x;
                let dest_y = offset_y as usize + y;
                let dest_page = dest_y / 8;
                let dest_bit = dest_y % 8;
                let dest_index = dest_page * display_width + dest_x;

                if dest_index < self.framebuffer.len() && dest_x < display_width {
                    if pixel_on == 1 {
                        self.framebuffer[dest_index] &= !(1 << dest_bit);
                    } else {
                        self.framebuffer[dest_index] |= 1 << dest_bit;
                    }
                }
            }
        }
    }

    pub fn dither<T>(&mut self, progress: T, inverted: bool)
    where
        T: Num + ToPrimitive,
    {
        if progress.is_zero() {
            return;
        }
        const BAYER_MATRIX: [[u8; 4]; 4] =
            [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

        let progress = progress.to_u8().unwrap_or(0);
        let progress = progress % 16;

        for y in 0..Screen::OLED_DISPLAY_HEIGHT {
            for x in 0..Screen::OLED_DISPLAY_WIDTH {
                let bayer_x = x % 4;
                let bayer_y = y % 4;
                let bayer_value = BAYER_MATRIX[bayer_y][bayer_x];
                let threshold = progress * 2;

                if bayer_value > threshold {
                    continue;
                }

                if inverted {
                    self.draw_pixel(x, y);
                } else {
                    self.clear_pixel(x, y);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer = [0; Screen::OLED_DISPLAY_SIZE];
    }
}

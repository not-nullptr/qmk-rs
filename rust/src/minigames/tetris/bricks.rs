use alloc::vec;
use alloc::vec::Vec;
use enum_iterator::{all, Sequence};

use crate::random::rnd;

type Pixel = (isize, isize);

pub type BrickInfo<'a> = &'a [Pixel];

pub static SHAPE_I: BrickInfo = &[(0, 2), (0, 1), (0, -1)];
pub static SHAPE_O: BrickInfo = &[(0, 1), (1, 1), (1, 0)];
pub static SHAPE_T: BrickInfo = &[(-1, 0), (0, 1), (1, 0)];
pub static SHAPE_S: BrickInfo = &[(1, 0), (0, 1), (1, -1)];
pub static SHAPE_Z: BrickInfo = &[(0, 1), (-1, 0), (-1, -1)];
pub static SHAPE_J: BrickInfo = &[(0, 1), (0, -1), (-1, -1)];
pub static SHAPE_L: BrickInfo = &[(0, 1), (0, -1), (1, -1)];

#[derive(Sequence, Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum BrickType {
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
}

#[derive(Debug, Clone)]
pub struct Brick {
    pub brick_type: BrickType,
    pub pixels: Vec<Pixel>,
}

impl Brick {
    pub fn limits(&self) -> (isize, isize, isize, isize) {
        if self.pixels.len() == 0 {
            return (0, 0, 0, 0);
        }
        self.pixels.iter().fold(
            (
                core::isize::MAX,
                core::isize::MIN,
                core::isize::MAX,
                core::isize::MIN,
            ),
            |(min_x, max_x, min_y, max_y), &(x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            },
        )
    }
    pub fn get_size(&self) -> (usize, usize) {
        let (min_x, max_x, min_y, max_y) = self.limits();
        ((max_x - min_x) as usize + 1, (max_y - min_y) as usize + 1)
    }

    pub fn new(b: BrickType) -> Self {
        let ps: BrickInfo = match b {
            BrickType::I => SHAPE_I,
            BrickType::O => SHAPE_O,
            BrickType::T => SHAPE_T,
            BrickType::S => SHAPE_S,
            BrickType::Z => SHAPE_Z,
            BrickType::L => SHAPE_L,
            BrickType::J => SHAPE_J,
        };
        Self {
            brick_type: b,
            pixels: ps.to_vec(),
        }
    }
    pub fn rotate(&mut self) {
        for i in 0..self.pixels.len() {
            let (x, y) = self.pixels[i];
            self.pixels[i] = (y, -x);
        }
    }
    pub fn pixels_info(&self, offset_x: isize, offset_y: isize) -> Vec<(isize, isize)> {
        let mut absolute_positions: Vec<(isize, isize)> = vec![(offset_x, offset_y)];
        for e in &self.pixels {
            absolute_positions.push((offset_x + e.0, (offset_y - e.1)))
        }
        absolute_positions
    }

    pub fn random() -> Brick {
        let types: Vec<_> = all::<BrickType>().collect();
        let piece = types.get(rnd() as usize % types.len()).unwrap();
        Brick::new(*piece)
    }
}

use crate::random::rnd;
fn remap_value(x: u8, n: u8) -> u8 {
    let max = 127;
    let range = max - 2 * n;
    let y = n + ((range as u16 * x as u16) / max as u16) as u8;
    y
}

pub struct Pipe {
    hole_at: u8,
}

impl Pipe {
    const GAP_SIZE: u8 = 32;

    pub fn new() -> Self {
        Self {
            hole_at: remap_value((rnd() % 128) as u8, 32),
        }
    }

    pub fn is_dead(&self, y: u8) -> bool {
        let lower_bound = self.hole_at.saturating_sub(Self::GAP_SIZE / 2);
        let upper_bound = self.hole_at.saturating_add(Self::GAP_SIZE / 2);

        !(y >= lower_bound && y <= upper_bound)
    }
}

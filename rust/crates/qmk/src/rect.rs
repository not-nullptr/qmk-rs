use num_traits::{Num, ToPrimitive};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect<T>
where
    T: Num + ToPrimitive,
{
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T>
where
    T: Num + ToPrimitive + Copy,
{
    pub fn intersects<U>(&self, other: &Rect<U>) -> bool
    where
        U: Num + ToPrimitive,
    {
        let self_x = self.x.to_i32().unwrap_or(0);
        let self_y = self.y.to_i32().unwrap_or(0);
        let self_width = self.width.to_i32().unwrap_or(0);
        let self_height = self.height.to_i32().unwrap_or(0);

        let other_x = other.x.to_i32().unwrap_or(0);
        let other_y = other.y.to_i32().unwrap_or(0);
        let other_width = other.width.to_i32().unwrap_or(0);
        let other_height = other.height.to_i32().unwrap_or(0);

        !(self_x >= other_x + other_width
            || self_x + self_width <= other_x
            || self_y >= other_y + other_height
            || self_y + self_height <= other_y)
    }

    pub fn bottom(&self) -> T {
        self.y + self.height
    }

    pub fn top(&self) -> T {
        self.y
    }
}

use alloc::vec::Vec;
use micromath::F32Ext;
use qmk::{rect::Rect, screen::Screen};

use crate::{
    image::cat,
    page::RenderInfo,
    random::{rand, rand_between},
    state::InputEvent,
};

#[derive(PartialEq)]
enum CatState {
    Idle,
    Jumping,
    Falling,
    Walking,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CatAction {
    /// Stop X velocity.
    Idle,
    /// Attempt to move and jump to a given target rectangle.
    Target(Rect<i16>),
    /// Fall asleep, with N ticks remaining.
    Sleep(u8),
    /// Randomly walk around with N ticks remaining.
    Wander(u8),
}

impl CatState {
    pub fn is_midair(&self) -> bool {
        matches!(self, CatState::Jumping | CatState::Falling)
    }

    pub fn is_grounded(&self) -> bool {
        matches!(self, CatState::Idle | CatState::Walking)
    }

    pub fn is_walking(&self) -> bool {
        matches!(self, CatState::Walking)
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, CatState::Idle)
    }
}

#[derive(PartialEq)]
enum CatDirection {
    Left,
    Right,
}

pub struct Cat {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    state: CatState,
    direction: CatDirection,
    action: CatAction,
    // target: Option<Rect<i16>>,
}

impl Cat {
    const WALK_SPEED: f32 = 3.5;

    pub const fn new() -> Self {
        Cat {
            x: Screen::OLED_DISPLAY_WIDTH as f32 / 2.0,
            y: Screen::OLED_DISPLAY_HEIGHT as f32 - 28.0,
            vx: 0.0,
            vy: 0.0,
            state: CatState::Idle,
            direction: CatDirection::Right,
            action: CatAction::Idle,
        }
    }

    fn ai_set_action(&mut self, info: &mut RenderInfo<'_>) {
        match self.action {
            CatAction::Target(t) if !info.framebuffer.bounds().contains(&t) => {
                self.action = CatAction::Idle;
                self.vy += 1.0;
                self.state = CatState::Falling;
            }

            CatAction::Idle if rand() % 100 < 10 => {
                let time = rand_between(10..40) as u8;
                self.action = CatAction::Wander(time);
            }

            // 5% chance each tick to pick a new target
            CatAction::Idle if rand() % 100 < 5 => {
                let bounds = info.framebuffer.bounds();
                if bounds.is_empty() {
                    return;
                }

                let target = bounds[rand() as usize % bounds.len()];
                self.action = CatAction::Target(target);
            }

            CatAction::Idle if rand() % 100 < 5 => {
                self.action = CatAction::Sleep(200 + (rand() % 100) as u8);
            }

            CatAction::Target(target) => {
                if self.state.is_midair() {
                    return;
                }

                if let Some(grounded) = self.is_grounded(self.x, self.y, info) {
                    if grounded == target {
                        self.action = CatAction::Idle;
                        self.vy = grounded.top() as f32 - (self.y + 28.0);
                    }
                }
            }

            CatAction::Sleep(_) if self.vy != 0.0 => {
                self.action = CatAction::Idle;
            }

            CatAction::Sleep(ticks) => {
                if ticks == 0 {
                    self.action = CatAction::Idle;
                } else {
                    self.action = CatAction::Sleep(ticks - 1);
                }
            }

            CatAction::Wander(ticks) => {
                if ticks == 0 {
                    self.action = CatAction::Idle;
                } else {
                    self.action = CatAction::Wander(ticks - 1);
                }
            }

            // random events were covered above -- no need to do anything here
            CatAction::Idle => {}
        }
    }

    fn ai_apply_action(&mut self, info: &mut RenderInfo<'_>) {
        let old_x = self.x - self.vx;
        let old_y = self.y - self.vy;

        match self.action {
            CatAction::Idle => {
                self.vx = 0.0;
            }

            CatAction::Target(t) => {
                if !info.framebuffer.bounds().contains(&t) {
                    self.action = CatAction::Idle;
                    return;
                }

                let grounded = self.is_grounded(old_x, old_y, info);

                if let Some(g) = grounded {
                    if g == t {
                        self.action = CatAction::Idle;
                        self.vx = 0.0;
                        self.vy = 0.0;
                        return;
                    }
                } else {
                    return;
                }

                let (min_vy, ticks_to_reach) = Self::ticks_to_reach(self.y, t.top() as f32, 1.0);

                let dist_x = (t.x as f32 + t.width as f32 / 2.0) - self.x;
                let needed_vx = dist_x / ticks_to_reach;
                self.vy = min_vy;
                self.vx = needed_vx;
            }

            CatAction::Sleep(_) => {
                self.vx = 0.0;
            }

            CatAction::Wander(_) => {
                if rand() % 100 < 5 {
                    self.vy = -(rand_between(8..12) as f32);
                }

                self.vx = if self.direction == CatDirection::Left {
                    -Self::WALK_SPEED
                } else {
                    Self::WALK_SPEED
                }
            }
        }
    }

    fn ai_update(&mut self, info: &mut RenderInfo<'_>) {
        self.ai_set_action(info);
        self.ai_apply_action(info);
    }

    fn min_velocity_to_reach(current_y: f32, target_y: f32, gravity_per_tick: f32) -> f32 {
        if target_y >= current_y {
            return -3.0;
        }

        let height_diff = current_y - target_y;
        -f32::sqrt(2.0 * gravity_per_tick * height_diff)
    }

    fn ticks_to_reach(current_y: f32, target_y: f32, gravity_per_tick: f32) -> (f32, f32) {
        if target_y >= current_y {
            return (-3.0, (target_y - current_y) / -3.0);
        }

        let height_diff = current_y - target_y;
        let velocity = Self::min_velocity_to_reach(current_y, target_y, gravity_per_tick);

        (
            velocity,
            f32::sqrt(2.0 * height_diff / gravity_per_tick.abs()),
        )
    }

    fn update(&mut self, info: &mut RenderInfo<'_>) {
        let old_x = self.x;
        let old_y = self.y;

        self.ai_update(info);

        if self.is_grounded(old_x, old_y, info).is_none() {
            self.vy += 1.0;
        }

        self.x += self.vx;
        self.y += self.vy;

        if self.x < 0.0 {
            self.x = 0.0;
            self.vx = -self.vx;
        }

        if self.x > Screen::OLED_DISPLAY_WIDTH as f32 - 1.0 {
            self.x = Screen::OLED_DISPLAY_WIDTH as f32 - 1.0;
            self.vx = -self.vx;
        }

        self.update_state();
    }

    fn floor() -> Rect<i16> {
        Rect {
            x: 0,
            y: Screen::OLED_DISPLAY_HEIGHT as i16 - 4,
            width: Screen::OLED_DISPLAY_WIDTH as i16,
            height: 4,
        }
    }

    fn is_grounded(&self, old_x: f32, old_y: f32, info: &mut RenderInfo<'_>) -> Option<Rect<i16>> {
        if self.vy < 0.0 {
            return None; // jumping upwards
        }

        if self.y >= Screen::OLED_DISPLAY_HEIGHT as f32 - 4.0 {
            return Some(Self::floor());
        }

        let ours = self.rect();
        let new_foot_rect = Rect {
            x: ours.x,
            y: ours.bottom() - 4.0,
            width: ours.width,
            height: 4.0,
        };

        let old_foot_rect = Rect {
            x: old_x - 16.0,
            y: old_y + 28.0 - 4.0,
            width: ours.width,
            height: 4.0,
        };

        // create a rect that encompasses both the old and new foot rects to ensure collisions aren't missed between frames
        let foot_rect = Rect {
            x: new_foot_rect.x.min(old_foot_rect.x),
            y: new_foot_rect.y.min(old_foot_rect.y),
            width: new_foot_rect.width.max(old_foot_rect.width),
            height: (new_foot_rect.bottom() - new_foot_rect.y)
                + (old_foot_rect.bottom() - old_foot_rect.y),
        };

        if let CatAction::Target(target) = self.action {
            if foot_rect.intersects(&Rect {
                x: target.x as f32,
                y: target.y as f32,
                width: target.width as f32,
                height: target.height as f32,
            }) {
                return Some(target);
            }

            for bound in info.framebuffer.bounds() {
                if bound.bottom() < target.top() {
                    continue;
                }

                let bound_f = Rect {
                    x: bound.x as f32,
                    y: bound.y as f32,
                    width: bound.width as f32,
                    height: bound.height as f32,
                };

                if foot_rect.intersects(&bound_f) {
                    return Some(*bound);
                }
            }
        } else {
            for bound in info.framebuffer.bounds() {
                let bound_f = Rect {
                    x: bound.x as f32,
                    y: bound.y as f32,
                    width: bound.width as f32,
                    height: bound.height as f32,
                };

                if foot_rect.intersects(&bound_f) {
                    return Some(*bound);
                }
            }
        }

        None
    }

    fn rect(&self) -> Rect<f32> {
        Self::rect_at(self.x, self.y)
    }

    fn rect_at(x: f32, y: f32) -> Rect<f32> {
        Rect {
            x: x - 16.0,
            y: y - 28.0,
            width: 32.0,
            height: 28.0,
        }
    }

    fn update_state(&mut self) {
        if self.y > Screen::OLED_DISPLAY_HEIGHT as f32 - 4.0 {
            if self.vx != 0.0 {
                self.state = CatState::Walking;
            } else {
                self.state = CatState::Idle;
            }
        }

        if self.y > Screen::OLED_DISPLAY_HEIGHT as f32 - 4.0 {
            self.y = Screen::OLED_DISPLAY_HEIGHT as f32 - 4.0;
            self.vy = 0.0;

            if self.vx != 0.0 {
                self.state = CatState::Walking;
            } else {
                self.state = CatState::Idle;
            }
        }

        match self.vx.partial_cmp(&0.0) {
            Some(core::cmp::Ordering::Greater) => self.direction = CatDirection::Right,
            Some(core::cmp::Ordering::Less) => self.direction = CatDirection::Left,
            Some(core::cmp::Ordering::Equal) | None => {}
        }

        match self.vy.partial_cmp(&0.0) {
            Some(core::cmp::Ordering::Greater) => self.state = CatState::Falling,
            Some(core::cmp::Ordering::Less) => self.state = CatState::Jumping,
            Some(core::cmp::Ordering::Equal) | None => {
                self.state = if self.vx != 0.0 {
                    CatState::Walking
                } else {
                    CatState::Idle
                }
            }
        }
    }

    pub fn draw(&mut self, info: &mut RenderInfo<'_>) {
        info.framebuffer.finalize_bounds();
        self.update(info);

        let sprite = match self.state {
            CatState::Idle => match self.action {
                CatAction::Sleep(t) if t > 20 => {
                    Self::get_anim([&cat::SLEEP_1, &cat::SLEEP_2], info.tick, 2)
                }
                _ => &cat::AWAKE,
            },

            CatState::Falling if self.direction == CatDirection::Left => &cat::DOWN_LEFT_1,
            CatState::Falling => &cat::DOWN_RIGHT_1,

            CatState::Walking if self.direction == CatDirection::Left => Self::get_anim(
                [&cat::LEFT_1, &cat::LEFT_2],
                info.tick,
                Self::WALK_SPEED as u32 * 2,
            ),
            CatState::Walking => Self::get_anim(
                [&cat::RIGHT_1, &cat::RIGHT_2],
                info.tick,
                Self::WALK_SPEED as u32 * 2,
            ),

            CatState::Jumping if self.direction == CatDirection::Left => &cat::UP_LEFT_2,
            CatState::Jumping => &cat::UP_RIGHT_2,
        };

        info.framebuffer
            .draw_image(self.x as i16 - 16, self.y as i16 - 28, sprite);
    }

    fn get_anim<T, const S: usize>(arr: [&T; S], tick: u32, speed: u32) -> &T {
        arr[((tick / (20 / speed.max(1))) as usize) % S]
    }
}

use core::{i16, intrinsics::powf32};

use alloc::{format, string::ToString};
use rp2040_hal::rom_data::float_funcs::{fcos, fsin, fsqrt};

use super::game::{Game, GameContext};
use crate::abstractions::{Keycode, Screen};

static TABLE: [u8; 91] = [
    0, 4, 8, 13, 17, 22, 26, 31, 35, 39, 44, 48, 53, 57, 61, 65, 70, 74, 78, 83, 87, 91, 95, 99,
    103, 107, 111, 115, 119, 123, 127, 131, 135, 138, 142, 146, 149, 153, 156, 160, 163, 167, 170,
    173, 177, 180, 183, 186, 189, 192, 195, 198, 200, 203, 206, 208, 211, 213, 216, 218, 220, 223,
    225, 227, 229, 231, 232, 234, 236, 238, 239, 241, 242, 243, 245, 246, 247, 248, 249, 250, 251,
    251, 252, 253, 253, 254, 254, 254, 254, 254, 255,
];

fn isin(i: i16) -> f32 {
    let i = (i % 360 + 360) % 360;
    if i < 180 {
        return TABLE[(if i < 90 { i } else { 180 - i }) as usize] as f32 / 255.0;
    }
    return -(TABLE[(if i < 270 { i - 180 } else { 360 - i }) as usize] as f32 / 255.0);
}

fn icos(i: i16) -> f32 {
    isin(i + 90)
}

pub fn floor_f32(x: f32) -> i32 {
    let as_int = x as i32;
    if x < 0.0 && (x as f32) != as_int as f32 {
        as_int - 1
    } else {
        as_int
    }
}

struct Player {
    x: f32,
    y: f32,
    angle: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 6.0,
            y: 6.0,
            angle: -180.0,
        }
    }
}

pub struct RaycasterGame {
    player: Player,
}

impl RaycasterGame {
    const INT_SCALE: u32 = 100;
    const SCREEN_WIDTH: u32 = Screen::SCREEN_WIDTH as u32;
    const SCREEN_HEIGHT: u32 = Screen::SCREEN_HEIGHT as u32;
    const SCREEN_HALF_WIDTH: u32 = Self::SCREEN_WIDTH / 2;
    const SCREEN_HALF_HEIGHT: u32 = Self::SCREEN_HEIGHT / 2;
    const FOV: u32 = 60;
    const HALF_FOV: u32 = Self::FOV / 2;
    const INCREMENT_ANGLE: f32 = (Self::FOV as f32) / (Self::SCREEN_WIDTH as f32);
    const PRECISION: f32 = 64.0;
    const MAP: [[u32; 8]; 8] = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 1, 0, 0, 0, 1],
        [1, 0, 1, 1, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];
}

impl Game for RaycasterGame {
    fn create() -> Self {
        RaycasterGame {
            player: Player::new(),
        }
    }

    fn logic_tick(&mut self, ctx: &mut GameContext) {
        for key in ctx.key_buffer {
            match key {
                Keycode::KC_G => {
                    self.player.angle += 25.0_f32.to_radians();
                }
                Keycode::KC_D => {
                    self.player.angle -= 25.0_f32.to_radians();
                }
                Keycode::KC_F => {
                    self.player.x += fcos(self.player.angle) * 0.25;
                    self.player.y += fsin(self.player.angle) * 0.25;
                }
                _ => {}
            }
        }
    }

    fn render_tick(&mut self, ctx: &mut GameContext) {
        let ray_angle = self.player.angle - (Self::HALF_FOV as f32);
        let precision = Self::PRECISION as f32;
        Screen::clear(false);
        // Screen::draw_text(&self.player.x.to_string(), true);
        // Screen::draw_text(&self.player.y.to_string(), true);
        // Screen::draw_text(&self.player.angle.to_string(), true);
        for x in 0..Self::SCREEN_WIDTH {
            let mut ray_x = self.player.x as f32;
            let mut ray_y = self.player.y as f32;

            let ray_cos = fcos(ray_angle.to_radians()) / Self::PRECISION;
            let ray_sin = fsin(ray_angle.to_radians()) / Self::PRECISION;

            let mut wall = 0;
            while wall == 0 {
                ray_x += ray_cos;
                ray_y += ray_sin;
                wall =
                    Self::MAP[floor_f32(ray_y).max(0) as usize][floor_f32(ray_x).max(0) as usize];
            }

            let distance = unsafe {
                fsqrt(powf32(self.player.x - ray_x, 2.0) + powf32(self.player.y - ray_y, 2.0))
            };

            let wall_height = floor_f32(Self::SCREEN_HALF_HEIGHT as f32 / distance);
            Screen::draw_line(
                x as u8,
                (Self::SCREEN_HALF_HEIGHT - wall_height as u32) as u8,
                x as u8,
                (Self::SCREEN_HALF_HEIGHT + wall_height as u32) as u8,
            );
        }

        // for y in 0..8 {
        //     for x in 0..8 {
        //         let cell = Self::MAP[y][x];
        //         Screen::set_pixel(x as u8, y as u8 + 64, cell == 1);
        //     }
        // }

        // Screen::set_pixel(
        //     floor_f32(self.player.x) as u8,
        //     floor_f32(self.player.y) as u8 + 64,
        //     true,
        // );
    }

    fn logic_delay(&self) -> u8 {
        1
    }

    fn destroy(&mut self) {}
}

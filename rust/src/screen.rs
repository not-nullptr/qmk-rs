use crate::abstractions::Screen;
use qmk_macro::qmk_callback;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn remap(value: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    new_min + (value - old_min) * (new_max - new_min) / (old_max - old_min)
}

#[qmk_callback(bool)]
fn oled_task_user() -> bool {
    Screen::render();
    false
}

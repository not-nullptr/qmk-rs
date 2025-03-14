use crate::image::HELLO_WORLD;
use qmk::{qmk_callback, screen::Screen};

#[qmk_callback(() -> bool)]
fn oled_task_user() -> bool {
    Screen::draw_image(0, 0, HELLO_WORLD);
    Screen::draw_image(0, 0, HELLO_WORLD);

    false
}

use qmk_macro::qmk_callback;

#[qmk_callback(() -> bool)]
fn oled_task_user() -> bool {
    false
}

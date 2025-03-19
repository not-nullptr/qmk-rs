use core::sync::atomic::Ordering;

use crate::{config::SETTINGS, pages::TRANSITION_TYPE};
use critical_section::with;
use qmk::{qmk_callback, rgb::RGBLight};

unsafe extern "C" {
    fn do_that_stuff_man();
}

#[qmk_callback(() -> void)]
fn keyboard_post_init_user() {
    let settings = with(|cs| {
        let mut settings = SETTINGS.borrow_ref_mut(cs);
        settings.load();
        settings.clone()
    });
    RGBLight::set_hsv(settings.hsv[0], settings.hsv[1], settings.hsv[2]);
    TRANSITION_TYPE.store(settings.transition as u8, Ordering::SeqCst);
    unsafe {
        do_that_stuff_man();
    }
}

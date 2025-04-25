use core::sync::atomic::Ordering;

use crate::{config::SETTINGS, pages::TRANSITION_TYPE, screen::marquee};
use critical_section::with;
use qmk::{qmk_callback, rgb::RGBLight};

#[cfg(not(target_arch = "wasm32"))]
unsafe extern "C" {
    fn do_that_stuff_man();
}

#[qmk_callback(() -> void)]
fn keyboard_post_init_user() {
    let settings = with(|cs| {
        let settings = SETTINGS.borrow_ref(cs);
        #[allow(clippy::clone_on_copy)]
        settings.clone()
    });
    RGBLight::set_hsv(settings.hsv.0[0], settings.hsv.0[1], settings.hsv.0[2]);
    TRANSITION_TYPE.store(settings.transition as u8, Ordering::SeqCst);
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        do_that_stuff_man();
    };
}

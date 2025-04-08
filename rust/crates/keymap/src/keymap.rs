use qmk::keymap;

keymap! {
    "sofle/rev1",
    {
        KC_ESC,   KC_1,   KC_2,    KC_3,    KC_4,    KC_5,                     KC_6,    KC_7,    KC_8,    KC_9,    KC_0,  KC_GRV,
        KC_TAB,   KC_Q,   KC_W,    KC_E,    KC_R,    KC_T,                     KC_Y,    KC_U,    KC_I,    KC_O,    KC_P,  KC_BSPC,
        KC_LSFT,  KC_A,   KC_S,    KC_D,    KC_F,    KC_G,                     KC_H,    KC_J,    KC_K,    KC_L, KC_SCLN,  KC_QUOT,
        KC_LCTL,  KC_Z,   KC_X,    KC_C,    KC_V,    KC_B, KC_F20,    KC_F21,  KC_N,    KC_M,    KC_COMM, KC_DOT,KC_SLSH, KC_RSFT,
                         KC_LGUI,KC_LALT,KC_LCTL, TL_LOWR, KC_SPC,    KC_ENT,  TL_UPPR, KC_RCTL, KC_RALT, KC_RGUI
    },
}

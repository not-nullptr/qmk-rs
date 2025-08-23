use qmk::{keymap, keys::QK_USER_0, mo, to};

#[allow(dead_code)]
const NUM_LAYERS: u8 = 3;

const CS_LOWER: u16 = mo!(1);
const CS_GO_GAME: u16 = to!(2);
const CS_GO_DEF: u16 = to!(0);
pub const CS_RESET: u16 = QK_USER_0 as u16;

keymap! {
    "sofle/rev1",
    {
        KC_ESC,   KC_1,   KC_2,    KC_3,    KC_4,    KC_5,                        KC_6,     KC_7,    KC_8,    KC_9,    KC_0,  KC_GRV,
        KC_TAB,   KC_Q,   KC_W,    KC_E,    KC_R,    KC_T,                        KC_Y,     KC_U,    KC_I,    KC_O,    KC_P,  KC_BSPC,
        KC_LSFT,  KC_A,   KC_S,    KC_D,    KC_F,    KC_G,                        KC_H,     KC_J,    KC_K,    KC_L, KC_SCLN,  KC_QUOT,
        KC_LCTL,  KC_Z,   KC_X,    KC_C,    KC_V,    KC_B, KC_F20,    KC_F21,     KC_N,     KC_M,    KC_COMM, KC_DOT,KC_SLSH, KC_RSFT,
                         KC_LGUI,KC_LALT,KC_LCTL, CS_LOWER, KC_SPC,    KC_ENT, CS_GO_GAME, KC_RCTL, KC_RALT, KC_RIGHT
    },
    {
        _______,   KC_F1,   KC_F2,   KC_F3,   KC_F4,   KC_F5,                       KC_F6,   KC_F7,   KC_F8,   KC_F9,  KC_F10,  KC_F11,
        KC_GRV,    KC_1,    KC_2,    KC_3,    KC_4,    KC_5,                       KC_6,    KC_7,    KC_8,    KC_9,    KC_0,  KC_F12,
        _______, KC_EXLM,   KC_AT, KC_HASH,  KC_DLR, KC_PERC,                       KC_CIRC, KC_AMPR, KC_ASTR, KC_LPRN, KC_RPRN, KC_PIPE,
        _______,  KC_EQL, KC_MINS, KC_PLUS, KC_LCBR, KC_RCBR, _______,       _______, KC_LBRC, KC_RBRC, KC_SCLN, KC_COLN, KC_BSLS, _______,
                             _______, _______, _______, _______, _______,       _______, _______, _______, _______, _______
    },
    {
        XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,                     XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX
        XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,                     XXXXXXX, XXXXXXX,  KC_UP,  XXXXXXX, XXXXXXX, XXXXXXX
        XXXXXXX, XXXXXXX, XXXXXXX, KC_PIPE, XXXXXXX, XXXXXXX,                     XXXXXXX, KC_LEFT, KC_DOWN, KC_RIGHT,XXXXXXX, XXXXXXX
        XXXXXXX, XXXXXXX,   KC_Z,    KC_X,   KC_C,   XXXXXXX, XXXXXXX,    XXXXXXX,XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX
                         XXXXXXX,XXXXXXX,XXXXXXX, XXXXXXX,  KC_SPC,    KC_ENT , CS_GO_DEF, CS_RESET,XXXXXXX, XXXXXXX
    },
}

pub use qmk_sys::qk_keycode_defines::*;

pub const QK_LCTL: u16 = 0x0100;
pub const QK_LSFT: u16 = 0x0200;
pub const QK_LALT: u16 = 0x0400;
pub const QK_LGUI: u16 = 0x0800;
pub const QK_RMODS_MIN: u16 = 0x1000;
pub const QK_RCTL: u16 = 0x1100;
pub const QK_RSFT: u16 = 0x1200;
pub const QK_RALT: u16 = 0x1400;
pub const QK_RGUI: u16 = 0x1800;

pub use qmk_macro::keymap;

#[macro_export]
macro_rules! key {
    ($key:ident) => {
        $crate::keys::$key as u16
    };
    ($(_)*) => {
        $crate::key!(KC_NO)
    };
}

#[macro_export]
macro_rules! s {
    ($e:expr) => {
        (QK_LSFT | ($e as u16))
    };
}

pub const KC_TILD: u16 = s!(key!(KC_GRV));
pub const KC_EXLM: u16 = s!(key!(KC_1));
pub const KC_AT: u16 = s!(key!(KC_2));
pub const KC_HASH: u16 = s!(key!(KC_3));
pub const KC_DLR: u16 = s!(key!(KC_4));
pub const KC_PERC: u16 = s!(key!(KC_5));
pub const KC_CIRC: u16 = s!(key!(KC_6));
pub const KC_AMPR: u16 = s!(key!(KC_7));
pub const KC_ASTR: u16 = s!(key!(KC_8));
pub const KC_LPRN: u16 = s!(key!(KC_9));
pub const KC_RPRN: u16 = s!(key!(KC_0));
pub const KC_UNDS: u16 = s!(key!(KC_MINUS));
pub const KC_PLUS: u16 = s!(key!(KC_EQUAL));
pub const KC_LCBR: u16 = s!(key!(KC_LEFT_BRACKET));
pub const KC_RCBR: u16 = s!(key!(KC_RIGHT_BRACKET));
pub const KC_PIPE: u16 = s!(key!(KC_BACKSLASH));
pub const KC_COLN: u16 = s!(key!(KC_SEMICOLON));
pub const KC_DQUO: u16 = s!(key!(KC_QUOTE));
pub const KC_LABK: u16 = s!(key!(KC_COMMA));
pub const KC_RABK: u16 = s!(key!(KC_DOT));
pub const KC_QUES: u16 = s!(key!(KC_SLASH));

pub const KC_TILDE: u16 = KC_TILD;
pub const KC_EXCLAIM: u16 = KC_EXLM;
pub const KC_DOLLAR: u16 = KC_DLR;
pub const KC_PERCENT: u16 = KC_PERC;
pub const KC_CIRCUMFLEX: u16 = KC_CIRC;
pub const KC_AMPERSAND: u16 = KC_AMPR;
pub const KC_ASTERISK: u16 = KC_ASTR;
pub const KC_LEFT_PAREN: u16 = KC_LPRN;
pub const KC_RIGHT_PAREN: u16 = KC_RPRN;
pub const KC_UNDERSCORE: u16 = KC_UNDS;
pub const KC_LEFT_CURLY_BRACE: u16 = KC_LCBR;
pub const KC_RIGHT_CURLY_BRACE: u16 = KC_RCBR;
pub const KC_COLON: u16 = KC_COLN;
pub const KC_DOUBLE_QUOTE: u16 = KC_DQUO;
pub const KC_DQT: u16 = KC_DQUO;
pub const KC_LEFT_ANGLE_BRACKET: u16 = KC_LABK;
pub const KC_LT: u16 = KC_LABK;
pub const KC_RIGHT_ANGLE_BRACKET: u16 = KC_RABK;
pub const KC_GT: u16 = KC_RABK;
pub const KC_QUESTION: u16 = KC_QUES;

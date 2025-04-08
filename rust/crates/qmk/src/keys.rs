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

// #[macro_export]
macro_rules! keymap {
    ($($name:ident => $val:literal),*; $($arr:expr),*$(,)?) => {
        $(
            #[allow(non_upper_case_globals)]
            const $name: usize = $val;
        )*

        #[unsafe(no_mangle)]
        #[allow(non_upper_case_globals)]
        static keymaps: [[[u16; MATRIX_COLS]; MATRIX_ROWS]; NUM_LAYERS] =
            [
                $($arr),*
            ];
    };
}

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
macro_rules! layer {
    (
        $k00:ident,
        $k01:ident,
        $k02:ident,
        $k03:ident,
        $k04:ident,
        $k05:ident,
        $k06:ident,
        $k07:ident,
        $k08:ident,
        $k09:ident,
        $k10:ident,
        $k11:ident,
        $k12:ident,
        $k13:ident,
        $k14:ident,
        $k15:ident,
        $k16:ident,
        $k17:ident,
        $k18:ident,
        $k19:ident,
        $k20:ident,
        $k21:ident,
        $k22:ident,
        $k23:ident,
        $k24:ident,
        $k25:ident,
        $k26:ident,
        $k27:ident,
        $k28:ident,
        $k29:ident,
        $k30:ident,
        $k31:ident,
        $k32:ident,
        $k33:ident,
        $k34:ident,
        $k35:ident,
        $k36:ident,
        $k37:ident,
        $k38:ident,
        $k39:ident,
        $k40:ident,
        $k41:ident,
        $k42:ident,
        $k43:ident,
        $k44:ident,
        $k45:ident,
        $k46:ident,
        $k47:ident,
        $k48:ident,
        $k49:ident,
        $k50:ident,
        $k51:ident,
        $k52:ident,
        $k53:ident,
        $k54:ident,
        $k55:ident,
        $k56:ident,
        $k57:ident,
        $k58:ident,
        $k59:ident $(,)?
    ) => {
        [
            [
                $crate::key!($k00),
                $crate::key!($k01),
                $crate::key!($k02),
                $crate::key!($k03),
                $crate::key!($k04),
                $crate::key!($k05),
            ],
            [
                $crate::key!($k12),
                $crate::key!($k13),
                $crate::key!($k14),
                $crate::key!($k15),
                $crate::key!($k16),
                $crate::key!($k17),
            ],
            [
                $crate::key!($k24),
                $crate::key!($k25),
                $crate::key!($k26),
                $crate::key!($k27),
                $crate::key!($k28),
                $crate::key!($k29),
            ],
            [
                $crate::key!($k36),
                $crate::key!($k37),
                $crate::key!($k38),
                $crate::key!($k39),
                $crate::key!($k40),
                $crate::key!($k41),
            ],
            [
                $crate::key!($k50),
                $crate::key!($k51),
                $crate::key!($k52),
                $crate::key!($k53),
                $crate::key!($k54),
                $crate::key!($k42),
            ],
            [
                $crate::key!($k11),
                $crate::key!($k10),
                $crate::key!($k09),
                $crate::key!($k08),
                $crate::key!($k07),
                $crate::key!($k06),
            ],
            [
                $crate::key!($k23),
                $crate::key!($k22),
                $crate::key!($k21),
                $crate::key!($k20),
                $crate::key!($k19),
                $crate::key!($k18),
            ],
            [
                $crate::key!($k35),
                $crate::key!($k34),
                $crate::key!($k33),
                $crate::key!($k32),
                $crate::key!($k31),
                $crate::key!($k30),
            ],
            [
                $crate::key!($k49),
                $crate::key!($k48),
                $crate::key!($k47),
                $crate::key!($k46),
                $crate::key!($k45),
                $crate::key!($k44),
            ],
            [
                $crate::key!($k59),
                $crate::key!($k58),
                $crate::key!($k57),
                $crate::key!($k56),
                $crate::key!($k55),
                $crate::key!($k43),
            ],
        ]
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

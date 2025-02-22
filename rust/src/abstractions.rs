#![allow(warnings)]

use core::any::{type_name, type_name_of_val, Any, TypeId};

use alloc::{
    borrow::ToOwned,
    boxed::Box,
    fmt, format,
    string::{String, ToString},
    vec::Vec,
};
use critical_section::with;
use enum_iterator::{first, next};
use include_image_structs::QmkImage;

use crate::{
    animate::animate_frames,
    heap::{HEAP, HEAP_SIZE},
    image::CREDITS,
    keyboard::Keyboard,
    minigames::{
        flappy_bird::FlappyBird,
        game::{Game, GameContext},
        tetris::Tetris,
    },
    raw_c::{
        oled_clear, oled_render_dirty, oled_set_cursor, oled_write as oled_write_C,
        oled_write_pixel, oled_write_raw, tap_code16,
    },
    state::{AppPage, AppState, APP_STATE},
};

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum Keycode {
    KC_NO = 0,
    KC_TRANSPARENT = 1,
    KC_A = 4,
    KC_B = 5,
    KC_C = 6,
    KC_D = 7,
    KC_E = 8,
    KC_F = 9,
    KC_G = 10,
    KC_H = 11,
    KC_I = 12,
    KC_J = 13,
    KC_K = 14,
    KC_L = 15,
    KC_M = 16,
    KC_N = 17,
    KC_O = 18,
    KC_P = 19,
    KC_Q = 20,
    KC_R = 21,
    KC_S = 22,
    KC_T = 23,
    KC_U = 24,
    KC_V = 25,
    KC_W = 26,
    KC_X = 27,
    KC_Y = 28,
    KC_Z = 29,
    KC_1 = 30,
    KC_2 = 31,
    KC_3 = 32,
    KC_4 = 33,
    KC_5 = 34,
    KC_6 = 35,
    KC_7 = 36,
    KC_8 = 37,
    KC_9 = 38,
    KC_0 = 39,
    KC_ENTER = 40,
    KC_ESCAPE = 41,
    KC_BACKSPACE = 42,
    KC_TAB = 43,
    KC_SPACE = 44,
    KC_MINUS = 45,
    KC_EQUAL = 46,
    KC_LEFT_BRACKET = 47,
    KC_RIGHT_BRACKET = 48,
    KC_BACKSLASH = 49,
    KC_NONUS_HASH = 50,
    KC_SEMICOLON = 51,
    KC_QUOTE = 52,
    KC_GRAVE = 53,
    KC_COMMA = 54,
    KC_DOT = 55,
    KC_SLASH = 56,
    KC_CAPS_LOCK = 57,
    KC_F1 = 58,
    KC_F2 = 59,
    KC_F3 = 60,
    KC_F4 = 61,
    KC_F5 = 62,
    KC_F6 = 63,
    KC_F7 = 64,
    KC_F8 = 65,
    KC_F9 = 66,
    KC_F10 = 67,
    KC_F11 = 68,
    KC_F12 = 69,
    KC_PRINT_SCREEN = 70,
    KC_SCROLL_LOCK = 71,
    KC_PAUSE = 72,
    KC_INSERT = 73,
    KC_HOME = 74,
    KC_PAGE_UP = 75,
    KC_DELETE = 76,
    KC_END = 77,
    KC_PAGE_DOWN = 78,
    KC_RIGHT = 79,
    KC_LEFT = 80,
    KC_DOWN = 81,
    KC_UP = 82,
    KC_NUM_LOCK = 83,
    KC_KP_SLASH = 84,
    KC_KP_ASTERISK = 85,
    KC_KP_MINUS = 86,
    KC_KP_PLUS = 87,
    KC_KP_ENTER = 88,
    KC_KP_1 = 89,
    KC_KP_2 = 90,
    KC_KP_3 = 91,
    KC_KP_4 = 92,
    KC_KP_5 = 93,
    KC_KP_6 = 94,
    KC_KP_7 = 95,
    KC_KP_8 = 96,
    KC_KP_9 = 97,
    KC_KP_0 = 98,
    KC_KP_DOT = 99,
    KC_NONUS_BACKSLASH = 100,
    KC_APPLICATION = 101,
    KC_KB_POWER = 102,
    KC_KP_EQUAL = 103,
    KC_F13 = 104,
    KC_F14 = 105,
    KC_F15 = 106,
    KC_F16 = 107,
    KC_F17 = 108,
    KC_F18 = 109,
    KC_F19 = 110,
    KC_F20 = 111,
    KC_F21 = 112,
    KC_F22 = 113,
    KC_F23 = 114,
    KC_F24 = 115,
    KC_EXECUTE = 116,
    KC_HELP = 117,
    KC_MENU = 118,
    KC_SELECT = 119,
    KC_STOP = 120,
    KC_AGAIN = 121,
    KC_UNDO = 122,
    KC_CUT = 123,
    KC_COPY = 124,
    KC_PASTE = 125,
    KC_FIND = 126,
    KC_KB_MUTE = 127,
    KC_KB_VOLUME_UP = 128,
    KC_KB_VOLUME_DOWN = 129,
    KC_LOCKING_CAPS_LOCK = 130,
    KC_LOCKING_NUM_LOCK = 131,
    KC_LOCKING_SCROLL_LOCK = 132,
    KC_KP_COMMA = 133,
    KC_KP_EQUAL_AS400 = 134,
    KC_INTERNATIONAL_1 = 135,
    KC_INTERNATIONAL_2 = 136,
    KC_INTERNATIONAL_3 = 137,
    KC_INTERNATIONAL_4 = 138,
    KC_INTERNATIONAL_5 = 139,
    KC_INTERNATIONAL_6 = 140,
    KC_INTERNATIONAL_7 = 141,
    KC_INTERNATIONAL_8 = 142,
    KC_INTERNATIONAL_9 = 143,
    KC_LANGUAGE_1 = 144,
    KC_LANGUAGE_2 = 145,
    KC_LANGUAGE_3 = 146,
    KC_LANGUAGE_4 = 147,
    KC_LANGUAGE_5 = 148,
    KC_LANGUAGE_6 = 149,
    KC_LANGUAGE_7 = 150,
    KC_LANGUAGE_8 = 151,
    KC_LANGUAGE_9 = 152,
    KC_ALTERNATE_ERASE = 153,
    KC_SYSTEM_REQUEST = 154,
    KC_CANCEL = 155,
    KC_CLEAR = 156,
    KC_PRIOR = 157,
    KC_RETURN = 158,
    KC_SEPARATOR = 159,
    KC_OUT = 160,
    KC_OPER = 161,
    KC_CLEAR_AGAIN = 162,
    KC_CRSEL = 163,
    KC_EXSEL = 164,
    KC_SYSTEM_POWER = 165,
    KC_SYSTEM_SLEEP = 166,
    KC_SYSTEM_WAKE = 167,
    KC_AUDIO_MUTE = 168,
    KC_AUDIO_VOL_UP = 169,
    KC_AUDIO_VOL_DOWN = 170,
    KC_MEDIA_NEXT_TRACK = 171,
    KC_MEDIA_PREV_TRACK = 172,
    KC_MEDIA_STOP = 173,
    KC_MEDIA_PLAY_PAUSE = 174,
    KC_MEDIA_SELECT = 175,
    KC_MEDIA_EJECT = 176,
    KC_MAIL = 177,
    KC_CALCULATOR = 178,
    KC_MY_COMPUTER = 179,
    KC_WWW_SEARCH = 180,
    KC_WWW_HOME = 181,
    KC_WWW_BACK = 182,
    KC_WWW_FORWARD = 183,
    KC_WWW_STOP = 184,
    KC_WWW_REFRESH = 185,
    KC_WWW_FAVORITES = 186,
    KC_MEDIA_FAST_FORWARD = 187,
    KC_MEDIA_REWIND = 188,
    KC_BRIGHTNESS_UP = 189,
    KC_BRIGHTNESS_DOWN = 190,
    KC_CONTROL_PANEL = 191,
    KC_ASSISTANT = 192,
    KC_MISSION_CONTROL = 193,
    KC_LAUNCHPAD = 194,
    QK_MOUSE_CURSOR_UP = 205,
    QK_MOUSE_CURSOR_DOWN = 206,
    QK_MOUSE_CURSOR_LEFT = 207,
    QK_MOUSE_CURSOR_RIGHT = 208,
    QK_MOUSE_BUTTON_1 = 209,
    QK_MOUSE_BUTTON_2 = 210,
    QK_MOUSE_BUTTON_3 = 211,
    QK_MOUSE_BUTTON_4 = 212,
    QK_MOUSE_BUTTON_5 = 213,
    QK_MOUSE_BUTTON_6 = 214,
    QK_MOUSE_BUTTON_7 = 215,
    QK_MOUSE_BUTTON_8 = 216,
    QK_MOUSE_WHEEL_UP = 217,
    QK_MOUSE_WHEEL_DOWN = 218,
    QK_MOUSE_WHEEL_LEFT = 219,
    QK_MOUSE_WHEEL_RIGHT = 220,
    QK_MOUSE_ACCELERATION_0 = 221,
    QK_MOUSE_ACCELERATION_1 = 222,
    QK_MOUSE_ACCELERATION_2 = 223,
    KC_LEFT_CTRL = 224,
    KC_LEFT_SHIFT = 225,
    KC_LEFT_ALT = 226,
    KC_LEFT_GUI = 227,
    KC_RIGHT_CTRL = 228,
    KC_RIGHT_SHIFT = 229,
    KC_RIGHT_ALT = 230,
    KC_RIGHT_GUI = 231,
    QK_SWAP_HANDS_TOGGLE = 22256,
    QK_SWAP_HANDS_TAP_TOGGLE = 22257,
    QK_SWAP_HANDS_MOMENTARY_ON = 22258,
    QK_SWAP_HANDS_MOMENTARY_OFF = 22259,
    QK_SWAP_HANDS_OFF = 22260,
    QK_SWAP_HANDS_ON = 22261,
    QK_SWAP_HANDS_ONE_SHOT = 22262,
    QK_MAGIC_SWAP_CONTROL_CAPS_LOCK = 28672,
    QK_MAGIC_UNSWAP_CONTROL_CAPS_LOCK = 28673,
    QK_MAGIC_TOGGLE_CONTROL_CAPS_LOCK = 28674,
    QK_MAGIC_CAPS_LOCK_AS_CONTROL_OFF = 28675,
    QK_MAGIC_CAPS_LOCK_AS_CONTROL_ON = 28676,
    QK_MAGIC_SWAP_LALT_LGUI = 28677,
    QK_MAGIC_UNSWAP_LALT_LGUI = 28678,
    QK_MAGIC_SWAP_RALT_RGUI = 28679,
    QK_MAGIC_UNSWAP_RALT_RGUI = 28680,
    QK_MAGIC_GUI_ON = 28681,
    QK_MAGIC_GUI_OFF = 28682,
    QK_MAGIC_TOGGLE_GUI = 28683,
    QK_MAGIC_SWAP_GRAVE_ESC = 28684,
    QK_MAGIC_UNSWAP_GRAVE_ESC = 28685,
    QK_MAGIC_SWAP_BACKSLASH_BACKSPACE = 28686,
    QK_MAGIC_UNSWAP_BACKSLASH_BACKSPACE = 28687,
    QK_MAGIC_TOGGLE_BACKSLASH_BACKSPACE = 28688,
    QK_MAGIC_NKRO_ON = 28689,
    QK_MAGIC_NKRO_OFF = 28690,
    QK_MAGIC_TOGGLE_NKRO = 28691,
    QK_MAGIC_SWAP_ALT_GUI = 28692,
    QK_MAGIC_UNSWAP_ALT_GUI = 28693,
    QK_MAGIC_TOGGLE_ALT_GUI = 28694,
    QK_MAGIC_SWAP_LCTL_LGUI = 28695,
    QK_MAGIC_UNSWAP_LCTL_LGUI = 28696,
    QK_MAGIC_SWAP_RCTL_RGUI = 28697,
    QK_MAGIC_UNSWAP_RCTL_RGUI = 28698,
    QK_MAGIC_SWAP_CTL_GUI = 28699,
    QK_MAGIC_UNSWAP_CTL_GUI = 28700,
    QK_MAGIC_TOGGLE_CTL_GUI = 28701,
    QK_MAGIC_EE_HANDS_LEFT = 28702,
    QK_MAGIC_EE_HANDS_RIGHT = 28703,
    QK_MAGIC_SWAP_ESCAPE_CAPS_LOCK = 28704,
    QK_MAGIC_UNSWAP_ESCAPE_CAPS_LOCK = 28705,
    QK_MAGIC_TOGGLE_ESCAPE_CAPS_LOCK = 28706,
    QK_MIDI_ON = 28928,
    QK_MIDI_OFF = 28929,
    QK_MIDI_TOGGLE = 28930,
    QK_MIDI_NOTE_C_0 = 28931,
    QK_MIDI_NOTE_C_SHARP_0 = 28932,
    QK_MIDI_NOTE_D_0 = 28933,
    QK_MIDI_NOTE_D_SHARP_0 = 28934,
    QK_MIDI_NOTE_E_0 = 28935,
    QK_MIDI_NOTE_F_0 = 28936,
    QK_MIDI_NOTE_F_SHARP_0 = 28937,
    QK_MIDI_NOTE_G_0 = 28938,
    QK_MIDI_NOTE_G_SHARP_0 = 28939,
    QK_MIDI_NOTE_A_0 = 28940,
    QK_MIDI_NOTE_A_SHARP_0 = 28941,
    QK_MIDI_NOTE_B_0 = 28942,
    QK_MIDI_NOTE_C_1 = 28943,
    QK_MIDI_NOTE_C_SHARP_1 = 28944,
    QK_MIDI_NOTE_D_1 = 28945,
    QK_MIDI_NOTE_D_SHARP_1 = 28946,
    QK_MIDI_NOTE_E_1 = 28947,
    QK_MIDI_NOTE_F_1 = 28948,
    QK_MIDI_NOTE_F_SHARP_1 = 28949,
    QK_MIDI_NOTE_G_1 = 28950,
    QK_MIDI_NOTE_G_SHARP_1 = 28951,
    QK_MIDI_NOTE_A_1 = 28952,
    QK_MIDI_NOTE_A_SHARP_1 = 28953,
    QK_MIDI_NOTE_B_1 = 28954,
    QK_MIDI_NOTE_C_2 = 28955,
    QK_MIDI_NOTE_C_SHARP_2 = 28956,
    QK_MIDI_NOTE_D_2 = 28957,
    QK_MIDI_NOTE_D_SHARP_2 = 28958,
    QK_MIDI_NOTE_E_2 = 28959,
    QK_MIDI_NOTE_F_2 = 28960,
    QK_MIDI_NOTE_F_SHARP_2 = 28961,
    QK_MIDI_NOTE_G_2 = 28962,
    QK_MIDI_NOTE_G_SHARP_2 = 28963,
    QK_MIDI_NOTE_A_2 = 28964,
    QK_MIDI_NOTE_A_SHARP_2 = 28965,
    QK_MIDI_NOTE_B_2 = 28966,
    QK_MIDI_NOTE_C_3 = 28967,
    QK_MIDI_NOTE_C_SHARP_3 = 28968,
    QK_MIDI_NOTE_D_3 = 28969,
    QK_MIDI_NOTE_D_SHARP_3 = 28970,
    QK_MIDI_NOTE_E_3 = 28971,
    QK_MIDI_NOTE_F_3 = 28972,
    QK_MIDI_NOTE_F_SHARP_3 = 28973,
    QK_MIDI_NOTE_G_3 = 28974,
    QK_MIDI_NOTE_G_SHARP_3 = 28975,
    QK_MIDI_NOTE_A_3 = 28976,
    QK_MIDI_NOTE_A_SHARP_3 = 28977,
    QK_MIDI_NOTE_B_3 = 28978,
    QK_MIDI_NOTE_C_4 = 28979,
    QK_MIDI_NOTE_C_SHARP_4 = 28980,
    QK_MIDI_NOTE_D_4 = 28981,
    QK_MIDI_NOTE_D_SHARP_4 = 28982,
    QK_MIDI_NOTE_E_4 = 28983,
    QK_MIDI_NOTE_F_4 = 28984,
    QK_MIDI_NOTE_F_SHARP_4 = 28985,
    QK_MIDI_NOTE_G_4 = 28986,
    QK_MIDI_NOTE_G_SHARP_4 = 28987,
    QK_MIDI_NOTE_A_4 = 28988,
    QK_MIDI_NOTE_A_SHARP_4 = 28989,
    QK_MIDI_NOTE_B_4 = 28990,
    QK_MIDI_NOTE_C_5 = 28991,
    QK_MIDI_NOTE_C_SHARP_5 = 28992,
    QK_MIDI_NOTE_D_5 = 28993,
    QK_MIDI_NOTE_D_SHARP_5 = 28994,
    QK_MIDI_NOTE_E_5 = 28995,
    QK_MIDI_NOTE_F_5 = 28996,
    QK_MIDI_NOTE_F_SHARP_5 = 28997,
    QK_MIDI_NOTE_G_5 = 28998,
    QK_MIDI_NOTE_G_SHARP_5 = 28999,
    QK_MIDI_NOTE_A_5 = 29000,
    QK_MIDI_NOTE_A_SHARP_5 = 29001,
    QK_MIDI_NOTE_B_5 = 29002,
    QK_MIDI_OCTAVE_N2 = 29003,
    QK_MIDI_OCTAVE_N1 = 29004,
    QK_MIDI_OCTAVE_0 = 29005,
    QK_MIDI_OCTAVE_1 = 29006,
    QK_MIDI_OCTAVE_2 = 29007,
    QK_MIDI_OCTAVE_3 = 29008,
    QK_MIDI_OCTAVE_4 = 29009,
    QK_MIDI_OCTAVE_5 = 29010,
    QK_MIDI_OCTAVE_6 = 29011,
    QK_MIDI_OCTAVE_7 = 29012,
    QK_MIDI_OCTAVE_DOWN = 29013,
    QK_MIDI_OCTAVE_UP = 29014,
    QK_MIDI_TRANSPOSE_N6 = 29015,
    QK_MIDI_TRANSPOSE_N5 = 29016,
    QK_MIDI_TRANSPOSE_N4 = 29017,
    QK_MIDI_TRANSPOSE_N3 = 29018,
    QK_MIDI_TRANSPOSE_N2 = 29019,
    QK_MIDI_TRANSPOSE_N1 = 29020,
    QK_MIDI_TRANSPOSE_0 = 29021,
    QK_MIDI_TRANSPOSE_1 = 29022,
    QK_MIDI_TRANSPOSE_2 = 29023,
    QK_MIDI_TRANSPOSE_3 = 29024,
    QK_MIDI_TRANSPOSE_4 = 29025,
    QK_MIDI_TRANSPOSE_5 = 29026,
    QK_MIDI_TRANSPOSE_6 = 29027,
    QK_MIDI_TRANSPOSE_DOWN = 29028,
    QK_MIDI_TRANSPOSE_UP = 29029,
    QK_MIDI_VELOCITY_0 = 29030,
    QK_MIDI_VELOCITY_1 = 29031,
    QK_MIDI_VELOCITY_2 = 29032,
    QK_MIDI_VELOCITY_3 = 29033,
    QK_MIDI_VELOCITY_4 = 29034,
    QK_MIDI_VELOCITY_5 = 29035,
    QK_MIDI_VELOCITY_6 = 29036,
    QK_MIDI_VELOCITY_7 = 29037,
    QK_MIDI_VELOCITY_8 = 29038,
    QK_MIDI_VELOCITY_9 = 29039,
    QK_MIDI_VELOCITY_10 = 29040,
    QK_MIDI_VELOCITY_DOWN = 29041,
    QK_MIDI_VELOCITY_UP = 29042,
    QK_MIDI_CHANNEL_1 = 29043,
    QK_MIDI_CHANNEL_2 = 29044,
    QK_MIDI_CHANNEL_3 = 29045,
    QK_MIDI_CHANNEL_4 = 29046,
    QK_MIDI_CHANNEL_5 = 29047,
    QK_MIDI_CHANNEL_6 = 29048,
    QK_MIDI_CHANNEL_7 = 29049,
    QK_MIDI_CHANNEL_8 = 29050,
    QK_MIDI_CHANNEL_9 = 29051,
    QK_MIDI_CHANNEL_10 = 29052,
    QK_MIDI_CHANNEL_11 = 29053,
    QK_MIDI_CHANNEL_12 = 29054,
    QK_MIDI_CHANNEL_13 = 29055,
    QK_MIDI_CHANNEL_14 = 29056,
    QK_MIDI_CHANNEL_15 = 29057,
    QK_MIDI_CHANNEL_16 = 29058,
    QK_MIDI_CHANNEL_DOWN = 29059,
    QK_MIDI_CHANNEL_UP = 29060,
    QK_MIDI_ALL_NOTES_OFF = 29061,
    QK_MIDI_SUSTAIN = 29062,
    QK_MIDI_PORTAMENTO = 29063,
    QK_MIDI_SOSTENUTO = 29064,
    QK_MIDI_SOFT = 29065,
    QK_MIDI_LEGATO = 29066,
    QK_MIDI_MODULATION = 29067,
    QK_MIDI_MODULATION_SPEED_DOWN = 29068,
    QK_MIDI_MODULATION_SPEED_UP = 29069,
    QK_MIDI_PITCH_BEND_DOWN = 29070,
    QK_MIDI_PITCH_BEND_UP = 29071,
    QK_SEQUENCER_ON = 29184,
    QK_SEQUENCER_OFF = 29185,
    QK_SEQUENCER_TOGGLE = 29186,
    QK_SEQUENCER_TEMPO_DOWN = 29187,
    QK_SEQUENCER_TEMPO_UP = 29188,
    QK_SEQUENCER_RESOLUTION_DOWN = 29189,
    QK_SEQUENCER_RESOLUTION_UP = 29190,
    QK_SEQUENCER_STEPS_ALL = 29191,
    QK_SEQUENCER_STEPS_CLEAR = 29192,
    QK_JOYSTICK_BUTTON_0 = 29696,
    QK_JOYSTICK_BUTTON_1 = 29697,
    QK_JOYSTICK_BUTTON_2 = 29698,
    QK_JOYSTICK_BUTTON_3 = 29699,
    QK_JOYSTICK_BUTTON_4 = 29700,
    QK_JOYSTICK_BUTTON_5 = 29701,
    QK_JOYSTICK_BUTTON_6 = 29702,
    QK_JOYSTICK_BUTTON_7 = 29703,
    QK_JOYSTICK_BUTTON_8 = 29704,
    QK_JOYSTICK_BUTTON_9 = 29705,
    QK_JOYSTICK_BUTTON_10 = 29706,
    QK_JOYSTICK_BUTTON_11 = 29707,
    QK_JOYSTICK_BUTTON_12 = 29708,
    QK_JOYSTICK_BUTTON_13 = 29709,
    QK_JOYSTICK_BUTTON_14 = 29710,
    QK_JOYSTICK_BUTTON_15 = 29711,
    QK_JOYSTICK_BUTTON_16 = 29712,
    QK_JOYSTICK_BUTTON_17 = 29713,
    QK_JOYSTICK_BUTTON_18 = 29714,
    QK_JOYSTICK_BUTTON_19 = 29715,
    QK_JOYSTICK_BUTTON_20 = 29716,
    QK_JOYSTICK_BUTTON_21 = 29717,
    QK_JOYSTICK_BUTTON_22 = 29718,
    QK_JOYSTICK_BUTTON_23 = 29719,
    QK_JOYSTICK_BUTTON_24 = 29720,
    QK_JOYSTICK_BUTTON_25 = 29721,
    QK_JOYSTICK_BUTTON_26 = 29722,
    QK_JOYSTICK_BUTTON_27 = 29723,
    QK_JOYSTICK_BUTTON_28 = 29724,
    QK_JOYSTICK_BUTTON_29 = 29725,
    QK_JOYSTICK_BUTTON_30 = 29726,
    QK_JOYSTICK_BUTTON_31 = 29727,
    QK_PROGRAMMABLE_BUTTON_1 = 29760,
    QK_PROGRAMMABLE_BUTTON_2 = 29761,
    QK_PROGRAMMABLE_BUTTON_3 = 29762,
    QK_PROGRAMMABLE_BUTTON_4 = 29763,
    QK_PROGRAMMABLE_BUTTON_5 = 29764,
    QK_PROGRAMMABLE_BUTTON_6 = 29765,
    QK_PROGRAMMABLE_BUTTON_7 = 29766,
    QK_PROGRAMMABLE_BUTTON_8 = 29767,
    QK_PROGRAMMABLE_BUTTON_9 = 29768,
    QK_PROGRAMMABLE_BUTTON_10 = 29769,
    QK_PROGRAMMABLE_BUTTON_11 = 29770,
    QK_PROGRAMMABLE_BUTTON_12 = 29771,
    QK_PROGRAMMABLE_BUTTON_13 = 29772,
    QK_PROGRAMMABLE_BUTTON_14 = 29773,
    QK_PROGRAMMABLE_BUTTON_15 = 29774,
    QK_PROGRAMMABLE_BUTTON_16 = 29775,
    QK_PROGRAMMABLE_BUTTON_17 = 29776,
    QK_PROGRAMMABLE_BUTTON_18 = 29777,
    QK_PROGRAMMABLE_BUTTON_19 = 29778,
    QK_PROGRAMMABLE_BUTTON_20 = 29779,
    QK_PROGRAMMABLE_BUTTON_21 = 29780,
    QK_PROGRAMMABLE_BUTTON_22 = 29781,
    QK_PROGRAMMABLE_BUTTON_23 = 29782,
    QK_PROGRAMMABLE_BUTTON_24 = 29783,
    QK_PROGRAMMABLE_BUTTON_25 = 29784,
    QK_PROGRAMMABLE_BUTTON_26 = 29785,
    QK_PROGRAMMABLE_BUTTON_27 = 29786,
    QK_PROGRAMMABLE_BUTTON_28 = 29787,
    QK_PROGRAMMABLE_BUTTON_29 = 29788,
    QK_PROGRAMMABLE_BUTTON_30 = 29789,
    QK_PROGRAMMABLE_BUTTON_31 = 29790,
    QK_PROGRAMMABLE_BUTTON_32 = 29791,
    QK_AUDIO_ON = 29824,
    QK_AUDIO_OFF = 29825,
    QK_AUDIO_TOGGLE = 29826,
    QK_AUDIO_CLICKY_TOGGLE = 29834,
    QK_AUDIO_CLICKY_ON = 29835,
    QK_AUDIO_CLICKY_OFF = 29836,
    QK_AUDIO_CLICKY_UP = 29837,
    QK_AUDIO_CLICKY_DOWN = 29838,
    QK_AUDIO_CLICKY_RESET = 29839,
    QK_MUSIC_ON = 29840,
    QK_MUSIC_OFF = 29841,
    QK_MUSIC_TOGGLE = 29842,
    QK_MUSIC_MODE_NEXT = 29843,
    QK_AUDIO_VOICE_NEXT = 29844,
    QK_AUDIO_VOICE_PREVIOUS = 29845,
    QK_STENO_BOLT = 29936,
    QK_STENO_GEMINI = 29937,
    QK_STENO_COMB = 29938,
    QK_STENO_COMB_MAX = 29948,
    QK_MACRO_0 = 30464,
    QK_MACRO_1 = 30465,
    QK_MACRO_2 = 30466,
    QK_MACRO_3 = 30467,
    QK_MACRO_4 = 30468,
    QK_MACRO_5 = 30469,
    QK_MACRO_6 = 30470,
    QK_MACRO_7 = 30471,
    QK_MACRO_8 = 30472,
    QK_MACRO_9 = 30473,
    QK_MACRO_10 = 30474,
    QK_MACRO_11 = 30475,
    QK_MACRO_12 = 30476,
    QK_MACRO_13 = 30477,
    QK_MACRO_14 = 30478,
    QK_MACRO_15 = 30479,
    QK_MACRO_16 = 30480,
    QK_MACRO_17 = 30481,
    QK_MACRO_18 = 30482,
    QK_MACRO_19 = 30483,
    QK_MACRO_20 = 30484,
    QK_MACRO_21 = 30485,
    QK_MACRO_22 = 30486,
    QK_MACRO_23 = 30487,
    QK_MACRO_24 = 30488,
    QK_MACRO_25 = 30489,
    QK_MACRO_26 = 30490,
    QK_MACRO_27 = 30491,
    QK_MACRO_28 = 30492,
    QK_MACRO_29 = 30493,
    QK_MACRO_30 = 30494,
    QK_MACRO_31 = 30495,
    QK_OUTPUT_AUTO = 30592,
    QK_OUTPUT_NEXT = 30593,
    QK_OUTPUT_PREV = 30594,
    QK_OUTPUT_NONE = 30595,
    QK_OUTPUT_USB = 30596,
    QK_OUTPUT_2P4GHZ = 30597,
    QK_OUTPUT_BLUETOOTH = 30598,
    QK_BLUETOOTH_PROFILE_NEXT = 30608,
    QK_BLUETOOTH_PROFILE_PREV = 30609,
    QK_BLUETOOTH_UNPAIR = 30610,
    QK_BLUETOOTH_PROFILE1 = 30611,
    QK_BLUETOOTH_PROFILE2 = 30612,
    QK_BLUETOOTH_PROFILE3 = 30613,
    QK_BLUETOOTH_PROFILE4 = 30614,
    QK_BLUETOOTH_PROFILE5 = 30615,
    QK_BACKLIGHT_ON = 30720,
    QK_BACKLIGHT_OFF = 30721,
    QK_BACKLIGHT_TOGGLE = 30722,
    QK_BACKLIGHT_DOWN = 30723,
    QK_BACKLIGHT_UP = 30724,
    QK_BACKLIGHT_STEP = 30725,
    QK_BACKLIGHT_TOGGLE_BREATHING = 30726,
    QK_LED_MATRIX_ON = 30736,
    QK_LED_MATRIX_OFF = 30737,
    QK_LED_MATRIX_TOGGLE = 30738,
    QK_LED_MATRIX_MODE_NEXT = 30739,
    QK_LED_MATRIX_MODE_PREVIOUS = 30740,
    QK_LED_MATRIX_BRIGHTNESS_UP = 30741,
    QK_LED_MATRIX_BRIGHTNESS_DOWN = 30742,
    QK_LED_MATRIX_SPEED_UP = 30743,
    QK_LED_MATRIX_SPEED_DOWN = 30744,
    QK_UNDERGLOW_TOGGLE = 30752,
    QK_UNDERGLOW_MODE_NEXT = 30753,
    QK_UNDERGLOW_MODE_PREVIOUS = 30754,
    QK_UNDERGLOW_HUE_UP = 30755,
    QK_UNDERGLOW_HUE_DOWN = 30756,
    QK_UNDERGLOW_SATURATION_UP = 30757,
    QK_UNDERGLOW_SATURATION_DOWN = 30758,
    QK_UNDERGLOW_VALUE_UP = 30759,
    QK_UNDERGLOW_VALUE_DOWN = 30760,
    QK_UNDERGLOW_SPEED_UP = 30761,
    QK_UNDERGLOW_SPEED_DOWN = 30762,
    RGB_MODE_PLAIN = 30763,
    RGB_MODE_BREATHE = 30764,
    RGB_MODE_RAINBOW = 30765,
    RGB_MODE_SWIRL = 30766,
    RGB_MODE_SNAKE = 30767,
    RGB_MODE_KNIGHT = 30768,
    RGB_MODE_XMAS = 30769,
    RGB_MODE_GRADIENT = 30770,
    RGB_MODE_RGBTEST = 30771,
    RGB_MODE_TWINKLE = 30772,
    QK_RGB_MATRIX_ON = 30784,
    QK_RGB_MATRIX_OFF = 30785,
    QK_RGB_MATRIX_TOGGLE = 30786,
    QK_RGB_MATRIX_MODE_NEXT = 30787,
    QK_RGB_MATRIX_MODE_PREVIOUS = 30788,
    QK_RGB_MATRIX_HUE_UP = 30789,
    QK_RGB_MATRIX_HUE_DOWN = 30790,
    QK_RGB_MATRIX_SATURATION_UP = 30791,
    QK_RGB_MATRIX_SATURATION_DOWN = 30792,
    QK_RGB_MATRIX_VALUE_UP = 30793,
    QK_RGB_MATRIX_VALUE_DOWN = 30794,
    QK_RGB_MATRIX_SPEED_UP = 30795,
    QK_RGB_MATRIX_SPEED_DOWN = 30796,
    QK_BOOTLOADER = 31744,
    QK_REBOOT = 31745,
    QK_DEBUG_TOGGLE = 31746,
    QK_CLEAR_EEPROM = 31747,
    QK_MAKE = 31748,
    QK_AUTO_SHIFT_DOWN = 31760,
    QK_AUTO_SHIFT_UP = 31761,
    QK_AUTO_SHIFT_REPORT = 31762,
    QK_AUTO_SHIFT_ON = 31763,
    QK_AUTO_SHIFT_OFF = 31764,
    QK_AUTO_SHIFT_TOGGLE = 31765,
    QK_GRAVE_ESCAPE = 31766,
    QK_VELOCIKEY_TOGGLE = 31767,
    QK_SPACE_CADET_LEFT_CTRL_PARENTHESIS_OPEN = 31768,
    QK_SPACE_CADET_RIGHT_CTRL_PARENTHESIS_CLOSE = 31769,
    QK_SPACE_CADET_LEFT_SHIFT_PARENTHESIS_OPEN = 31770,
    QK_SPACE_CADET_RIGHT_SHIFT_PARENTHESIS_CLOSE = 31771,
    QK_SPACE_CADET_LEFT_ALT_PARENTHESIS_OPEN = 31772,
    QK_SPACE_CADET_RIGHT_ALT_PARENTHESIS_CLOSE = 31773,
    QK_SPACE_CADET_RIGHT_SHIFT_ENTER = 31774,
    QK_UNICODE_MODE_NEXT = 31792,
    QK_UNICODE_MODE_PREVIOUS = 31793,
    QK_UNICODE_MODE_MACOS = 31794,
    QK_UNICODE_MODE_LINUX = 31795,
    QK_UNICODE_MODE_WINDOWS = 31796,
    QK_UNICODE_MODE_BSD = 31797,
    QK_UNICODE_MODE_WINCOMPOSE = 31798,
    QK_UNICODE_MODE_EMACS = 31799,
    QK_HAPTIC_ON = 31808,
    QK_HAPTIC_OFF = 31809,
    QK_HAPTIC_TOGGLE = 31810,
    QK_HAPTIC_RESET = 31811,
    QK_HAPTIC_FEEDBACK_TOGGLE = 31812,
    QK_HAPTIC_BUZZ_TOGGLE = 31813,
    QK_HAPTIC_MODE_NEXT = 31814,
    QK_HAPTIC_MODE_PREVIOUS = 31815,
    QK_HAPTIC_CONTINUOUS_TOGGLE = 31816,
    QK_HAPTIC_CONTINUOUS_UP = 31817,
    QK_HAPTIC_CONTINUOUS_DOWN = 31818,
    QK_HAPTIC_DWELL_UP = 31819,
    QK_HAPTIC_DWELL_DOWN = 31820,
    QK_COMBO_ON = 31824,
    QK_COMBO_OFF = 31825,
    QK_COMBO_TOGGLE = 31826,
    QK_DYNAMIC_MACRO_RECORD_START_1 = 31827,
    QK_DYNAMIC_MACRO_RECORD_START_2 = 31828,
    QK_DYNAMIC_MACRO_RECORD_STOP = 31829,
    QK_DYNAMIC_MACRO_PLAY_1 = 31830,
    QK_DYNAMIC_MACRO_PLAY_2 = 31831,
    QK_LEADER = 31832,
    QK_LOCK = 31833,
    QK_ONE_SHOT_ON = 31834,
    QK_ONE_SHOT_OFF = 31835,
    QK_ONE_SHOT_TOGGLE = 31836,
    QK_KEY_OVERRIDE_TOGGLE = 31837,
    QK_KEY_OVERRIDE_ON = 31838,
    QK_KEY_OVERRIDE_OFF = 31839,
    QK_SECURE_LOCK = 31840,
    QK_SECURE_UNLOCK = 31841,
    QK_SECURE_TOGGLE = 31842,
    QK_SECURE_REQUEST = 31843,
    QK_DYNAMIC_TAPPING_TERM_PRINT = 31856,
    QK_DYNAMIC_TAPPING_TERM_UP = 31857,
    QK_DYNAMIC_TAPPING_TERM_DOWN = 31858,
    QK_CAPS_WORD_TOGGLE = 31859,
    QK_AUTOCORRECT_ON = 31860,
    QK_AUTOCORRECT_OFF = 31861,
    QK_AUTOCORRECT_TOGGLE = 31862,
    QK_TRI_LAYER_LOWER = 31863,
    QK_TRI_LAYER_UPPER = 31864,
    QK_REPEAT_KEY = 31865,
    QK_ALT_REPEAT_KEY = 31866,
    QK_LAYER_LOCK = 31867,
    QK_KB_0 = 32256,
    QK_KB_1 = 32257,
    QK_KB_2 = 32258,
    QK_KB_3 = 32259,
    QK_KB_4 = 32260,
    QK_KB_5 = 32261,
    QK_KB_6 = 32262,
    QK_KB_7 = 32263,
    QK_KB_8 = 32264,
    QK_KB_9 = 32265,
    QK_KB_10 = 32266,
    QK_KB_11 = 32267,
    QK_KB_12 = 32268,
    QK_KB_13 = 32269,
    QK_KB_14 = 32270,
    QK_KB_15 = 32271,
    QK_KB_16 = 32272,
    QK_KB_17 = 32273,
    QK_KB_18 = 32274,
    QK_KB_19 = 32275,
    QK_KB_20 = 32276,
    QK_KB_21 = 32277,
    QK_KB_22 = 32278,
    QK_KB_23 = 32279,
    QK_KB_24 = 32280,
    QK_KB_25 = 32281,
    QK_KB_26 = 32282,
    QK_KB_27 = 32283,
    QK_KB_28 = 32284,
    QK_KB_29 = 32285,
    QK_KB_30 = 32286,
    QK_KB_31 = 32287,
    QK_USER_0 = 32320,
    QK_USER_1 = 32321,
    QK_USER_2 = 32322,
    QK_USER_3 = 32323,
    QK_USER_4 = 32324,
    QK_USER_5 = 32325,
    QK_USER_6 = 32326,
    QK_USER_7 = 32327,
    QK_USER_8 = 32328,
    QK_USER_9 = 32329,
    QK_USER_10 = 32330,
    QK_USER_11 = 32331,
    QK_USER_12 = 32332,
    QK_USER_13 = 32333,
    QK_USER_14 = 32334,
    QK_USER_15 = 32335,
    QK_USER_16 = 32336,
    QK_USER_17 = 32337,
    QK_USER_18 = 32338,
    QK_USER_19 = 32339,
    QK_USER_20 = 32340,
    QK_USER_21 = 32341,
    QK_USER_22 = 32342,
    QK_USER_23 = 32343,
    QK_USER_24 = 32344,
    QK_USER_25 = 32345,
    QK_USER_26 = 32346,
    QK_USER_27 = 32347,
    QK_USER_28 = 32348,
    QK_USER_29 = 32349,
    QK_USER_30 = 32350,
    QK_USER_31 = 32351,
}

impl fmt::Display for Keycode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn press_key(keycode: Keycode) {
    unsafe {
        tap_code16(keycode);
    }
}

fn round_towards_infinity(x: i32, y: i32) -> i32 {
    if x <= 0 {
        y
    } else {
        (x + y - 1) / y * y
    }
}

pub struct Screen;

impl Screen {
    pub const SCREEN_WIDTH: u8 = 32;
    pub const SCREEN_HEIGHT: u8 = 128;
    pub const SCREEN_ROWS: u8 = 5; // as in 01234 or 0..5
    pub const SCREEN_COLS: u8 = 16; // as in 0123456789012345 or 0..16

    pub fn set_cursor(x: u8, y: u8) {
        unsafe {
            oled_set_cursor(x, y);
        }
    }

    fn draw_text_internal(s: &str, newline: bool, invert: bool) {
        let mut s = s.to_owned();
        if newline {
            let len = s.len() as i32;
            let pad_num = round_towards_infinity(len, Self::SCREEN_ROWS as i32) - len;
            for _ in 0..pad_num {
                s.push(' ');
            }
        }
        let mut b = s.as_bytes().to_vec();
        b.push(0);
        unsafe {
            oled_write_C(b.as_ptr(), invert);
        }
    }

    pub fn draw_text(s: &str, newline: bool) {
        Self::draw_text_internal(s, newline, false);
    }

    pub fn draw_text_inverted(s: &str, newline: bool) {
        Self::draw_text_internal(s, newline, true);
    }

    pub fn clear(rerender: bool) {
        unsafe {
            oled_clear();
            if rerender {
                oled_render_dirty(true);
            }
        }
    }

    pub fn newline() {
        Self::draw_text("", true);
    }

    pub fn draw_image<const N: usize>(image: &QmkImage<N>, offset_x: u8, offset_y: u8) {
        let columns = u32::div_ceil(image.height as u32, 8);
        for y_block in 0..columns {
            let y_offset = y_block * 8;
            for x in 0..image.width {
                let byte = image.bytes[x as usize + y_block as usize * image.width as usize];
                for bit in 0..8 {
                    let is_on = (byte & (1 << bit)) != 0;
                    let x = x + offset_x;
                    let y_offset = y_offset as u8 + offset_y;
                    Screen::set_pixel(x, (y_offset + bit) as u8, is_on);
                }
            }
        }
    }

    pub fn set_pixel(x: u8, y: u8, is_on: bool) {
        unsafe {
            oled_write_pixel(x, y, is_on);
        }
    }

    pub fn render() {
        with(|cs| {
            let mut state = APP_STATE.borrow(cs).borrow_mut();
            let keys = state.read_keys();
            state.animation_counter += 1;
            let animation_counter = state.animation_counter;
            if let Some(title) = state.page.get_title() {
                Screen::draw_text(title, true);
                Screen::newline();
            }

            // let load_game = |game: impl Game, state: &mut AppState| state.game;

            fn run_minigame<T>(state: &mut AppState, keys: &Vec<Keycode>)
            where
                T: Game + 'static,
            {
                let mut should_load = false;

                if let Some(game) = &mut state.game {
                    should_load = T::id() != game.idv();
                } else {
                    should_load = true;
                }

                if should_load {
                    state.game = Some(Box::new(T::create()));
                }

                let game = state.game.as_mut().unwrap();

                if state.animation_counter % game.logic_delay() as u32 == 0 {
                    game.logic_tick(&mut GameContext {
                        tick_num: state.animation_counter,
                        key_buffer: keys,
                    });
                }

                game.render_tick(&mut GameContext {
                    tick_num: state.animation_counter,
                    key_buffer: keys,
                });
            }

            match state.page {
                AppPage::Stats => {
                    let wpm = Keyboard::get_wpm();
                    Screen::draw_text("WPM:", true);
                    Screen::draw_text(&wpm.to_string(), true);
                }

                AppPage::Heap => {
                    let used = HEAP.used();
                    let free = HEAP.free();
                    let used = if used == 0 { 0 } else { used.div_ceil(1000) };
                    let free = if free == 0 { 0 } else { free.div_ceil(1000) };
                    let critical = used >= HEAP_SIZE / 2;
                    let used = format!("{}kb", used);
                    let free = format!("{}kb", free);
                    Screen::draw_text("Used:", true);
                    if critical {
                        Screen::draw_text_inverted(&used, true);
                    } else {
                        Screen::draw_text(&used, true);
                    }
                    Screen::newline();
                    Screen::draw_text("Free:", true);
                    if critical {
                        Screen::draw_text_inverted(&free, true);
                    } else {
                        Screen::draw_text(&free, true);
                    }
                }

                AppPage::KeyD => {
                    Screen::draw_text("CPU", true);
                    Screen::draw_text(&format!("{}%", state.cpu_usage), true);
                    Screen::newline();
                    Screen::draw_text("RAM", true);
                    Screen::draw_text(&format!("{}%", state.mem_usage), true);
                    Screen::newline();
                    Screen::draw_text("Procs", true);
                    Screen::draw_text(&state.process_count.to_string(), true);
                }

                AppPage::Debug => {
                    Screen::draw_text(&state.debug_str, true);
                }

                AppPage::Tetris => {
                    run_minigame::<Tetris>(&mut state, &keys);
                }

                AppPage::FlappyBird => {
                    run_minigame::<FlappyBird>(&mut state, &keys);
                }

                AppPage::Credits => {
                    Screen::draw_text("wrote", true);
                    Screen::draw_text("by", true);
                    Screen::draw_text("null-", true);
                    Screen::draw_text("ptr", true);
                    Screen::draw_text("in rs", true);
                    Screen::newline();
                    Screen::draw_text("sofle", true);
                    Screen::draw_text("ftw!!", true);
                    let y = Screen::SCREEN_HEIGHT - CREDITS[0].height - 8;
                    Screen::draw_image(&animate_frames(6, &CREDITS, state.animation_counter), 0, y);
                }
            }
        });
    }

    pub fn change_page(state: &mut AppState) {
        let Some(next) = next(&state.page) else {
            let Some(first) = first::<AppPage>() else {
                return;
            };
            state.page = first;
            Screen::clear(false);
            return;
        };
        state.page = next;
        Screen::clear(false);
    }

    pub fn draw_line(x0: u8, y0: u8, x1: u8, y1: u8) {
        let x0 = x0 as i32;
        let x1 = x1 as i32;
        let y0 = y0 as i32;
        let y1 = y1 as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        let mut iterations = 0;

        loop {
            Screen::set_pixel(x.max(0).min(255) as u8, y.max(0).min(255) as u8, true);

            if x == x1 && y == y1 || iterations >= 128 {
                break;
            }

            let e2 = err * 2;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }

            iterations += 1;
        }
    }
}

// Copyright 2024 Santosh Kumar (@santosh)
// SPDX-License-Identifier: GPL-2.0-or-later

#pragma once

#define OLED_DISPLAY_64X128
#define OLED_FONT_H "keyboards/sofle/keymaps/nulls_keymap/glcdfont.c"
#define OLED_UPDATE_PROCESS_LIMIT 2

#define SPLIT_TRANSACTION_IDS_USER HID_SYNC

#define RGBLIGHT_ENABLE

#define TRI_LAYER_LOWER_LAYER 2
#define TRI_LAYER_UPPER_LAYER 3
#define TRI_LAYER_ADJUST_LAYER 4
#define I2C1_CLOCK_SPEED 400000
#define RAW_USAGE_PAGE 0xFF60
#define RAW_USAGE_ID 0x61

#define CUSTOM_LAYER_READ //if you remove this it causes issues - needs better guarding

#define QUICK_TAP_TERM 0
#ifdef TAPPING_TERM
    #undef TAPPING_TERM
    #define TAPPING_TERM 200
#endif
#define ENCODER_DIRECTION_FLIP

#define RGBLIGHT_SLEEP
#define WS2812_DI_PIN D3

#ifdef RGBLIGHT_ENABLE
    #undef RGBLIGHT_LED_COUNT
	#define RGBLIGHT_EFFECT_RAINBOW_MOOD
    #define RGBLIGHT_LED_COUNT 70
	#undef RGBLED_SPLIT
	#define RGBLED_SPLIT { 35, 35 }
    #define RGBLIGHT_LIMIT_VAL 255
    #define RGBLIGHT_HUE_STEP 10
    #define RGBLIGHT_SAT_STEP 17
    #define RGBLIGHT_VAL_STEP 17
#endif
// ### NULLPTR'S STUFF BEGINS HERE -- DO NOT TOUCH! DON'T EVEN MODIFY THIS COMMENT!
#define EECONFIG_USER_DATA_SIZE 5

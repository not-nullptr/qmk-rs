// Copyright 2023 QMK
// SPDX-License-Identifier: GPL-2.0-or-later
#include QMK_KEYBOARD_H
#include "transactions.h"
#include <rust_bindings.c>

enum sofle_layers {
    /* _M_XYZ = Mac Os, _W_XYZ = Win/Linux */
    _QWERTY,
    _COLEMAK,
    _LOWER,
    _RAISE,
    _ADJUST,
};

enum custom_keycodes {
    KC_PRVWD = QK_USER,
    KC_NXTWD,
    KC_LSTRT,
    KC_LEND
};

#define KC_QWERTY PDF(_QWERTY)
#define KC_COLEMAK PDF(_COLEMAK)

const extern uint16_t PROGMEM keymaps[1][MATRIX_ROWS][MATRIX_COLS];

oled_rotation_t oled_init_user(oled_rotation_t rotation) {
  return OLED_ROTATION_0;
}

void hid_sync_slave_handler(uint8_t in_buflen, const void* in_data, uint8_t out_buflen, void* out_data) {
  const uint8_t *data = (const uint8_t*)in_data;
  on_usb_slave_data(data, in_buflen);
}

void do_that_stuff_man(void) {
  transaction_register_rpc(HID_SYNC, hid_sync_slave_handler);
}

bool send_to_slave(const void* data, uint8_t len) {
  return transaction_rpc_send(HID_SYNC, len, data);
}
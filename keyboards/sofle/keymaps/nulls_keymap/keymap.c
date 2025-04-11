#include QMK_KEYBOARD_H
#include "transactions.h"
#include <rust_bindings.c>

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
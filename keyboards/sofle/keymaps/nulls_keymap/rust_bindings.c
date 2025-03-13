void keyboard_pre_init_user_rs(void);
void keyboard_pre_init_user(void) {
  return keyboard_pre_init_user_rs();
}

bool encoder_update_user_rs(uint8_t arg0, bool arg1);
bool encoder_update_user(uint8_t arg0, bool arg1) {
  return encoder_update_user_rs(arg0, arg1);
}
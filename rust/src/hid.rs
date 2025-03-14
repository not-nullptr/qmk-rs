use critical_section::with;
use qmk_macro::qmk_callback;

use crate::state::APP_STATE;

#[qmk_callback((uint8_t*, uint8_t) -> void)]
fn raw_hid_receive(data: &mut [u8], _length: u8) {
    with(|cs| {
        let mut state = APP_STATE.borrow(cs).borrow_mut();
        let is_ours = data.len() > 2 && data[0] == 0x66 && data[1] == 0x66;
        if !is_ours {
            return;
        }

        let payload = &data[2..];

        let cpu_usage = payload[0];
        let mem_usage = payload[1];
        let process_count = u16::from_be_bytes([payload[2], payload[3]]);

        state.cpu_usage = cpu_usage;
        state.mem_usage = mem_usage;
        state.process_count = process_count;
    });
}

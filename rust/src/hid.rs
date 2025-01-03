use critical_section::with;

use crate::state::APP_STATE;

#[no_mangle]
pub extern "C" fn raw_hid_receive_rs(data: &mut [u8], length: u8) {
    with(|cs| {
        let mut state = APP_STATE.borrow(cs).borrow_mut();
        state.debug_count = data[31] as i32;
        // let is_ours = data.len() > 2 && data[0] == 0x66 && data[1] == 0x66;
        // state.debug_count = data[0] as i32;
        // if !is_ours {
        //     return;
        // }

        // let payload = &data[2..];

        // let mem_usage = payload[0];
        // let cpu_usage = payload[1];

        // state.cpu_usage = cpu_usage;
        // state.mem_usage = mem_usage;
    });
}

use std::env;

use qmk_callback::write_glue_code;

fn main() {
    let is_wasm_target = env::var("TARGET").unwrap_or_default() == "wasm32-unknown-unknown";
    if !is_wasm_target {
        write_glue_code("../../../keyboards/sofle/keymaps/nulls_keymap/rust_bindings.c");
    };
}

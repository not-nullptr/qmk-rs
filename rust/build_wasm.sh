cd crates/keymap
wasm-pack build && rm -rf /mnt/c/Users/nullptr/qmk-wasm/src/lib/pkg && cp -r pkg /mnt/c/Users/nullptr/qmk-wasm/src/lib
cd ../..
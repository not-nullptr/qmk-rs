TARGET=thumbv6m-none-eabi

build: $(shell find . -name '*.rs') $(shell find . -name 'Cargo.*')
	cargo build -p keymap --release --target $(TARGET)
	ln -sf ./target/$(TARGET)/release/libkeymap.a \
		rust_keymap.a
macro-expand:
	rustc -Z unstable-options --pretty expanded src/keymap.rs
rustc-build:
	rustc --emit=obj \
		--target=$(TARGET) \
		--codegen opt-level=3 \
		--codegen panic=abort \
		src/keymap.rs
dump-keymap-data:
	arm-none-eabi-objdump -s -j .rodata.keymaps  rust_keymap.a
deps:
	rustup target add $(TARGET)
clean:
	cargo clean
	-rm -f rust_keymap.a
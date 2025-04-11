TARGET=thumbv6m-none-eabi

build: $(shell find . -name '*.rs') $(shell find . -name 'Cargo.*')
	cargo build -p keymap --release --target $(TARGET)
	ln -sf ./target/$(TARGET)/release/libkeymap.a \
		rust_keymap.a
deps:
	rustup target add $(TARGET)
clean:
	cargo clean
	-rm -f rust_keymap.a
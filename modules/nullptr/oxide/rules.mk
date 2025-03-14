RUST_MOD := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))
RUST_DIR := $(RUST_MOD)/rust
TARGET := $(MAKECMDGOALS)

RUST_OBJ := $(RUST_MOD)/rust_keymap.a

@echo "Building $(TARGET) keymap"
dsfsdf
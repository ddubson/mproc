TARGET := x86_64-apple-darwin

ifeq ($(OS),Windows_NT)
TARGET = x86_64-pc-windows-gnu
endif

define execute_cargo
cargo ${1} --target=$(TARGET)
endef

install:
	$(call execute_cargo,$@ --path .)

build:
	$(call execute_cargo,$@)

run:
	$(call execute_cargo,$@)

test:
	$(call execute_cargo,$@)

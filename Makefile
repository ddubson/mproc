OS_TYPE := unix
RUST_TARGET  := x86_64-apple-darwin

ifeq ($(OS),Windows_NT)
    OS_TYPE = windows
    RUST_TARGET  = x86_64-pc-windows-gnu
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
    RUST_TARGET = x86_64-unknown-linux-gnu
    endif
endif

define execute_cargo
cargo ${1} --target=$(RUST_TARGET)
endef

define ensure_unix_program_exists
command -v ${1} >/dev/null 2>&1 || { echo >&2 "Program '${1}' is not installed!"; exit 1; }
endef

define ensure_windows_program_exists
where /q ${1} || ECHO "Program '${1}' is not installed!" && exit /F
endef

define ensure_program_exists
$(call ensure_$(OS_TYPE)_program_exists,${1})
endef

ensure_programs_installed:
	$(call ensure_program_exists,rustup)
	$(call ensure_program_exists,rustfmt)
	$(call ensure_program_exists,rustc)
	$(call ensure_program_exists,cargo)

install: ensure_programs_installed
	$(call execute_cargo,$@ --path .)

fmt:
	cargo fmt

fmt_ci:
	cargo fmt -- --check

build:
	$(call execute_cargo,$@)

run:
	$(call execute_cargo,$@)

test:
	$(call execute_cargo,$@)

ship-it: install fmt test build
	echo "Ready to ship."

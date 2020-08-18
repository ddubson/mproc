OS_TYPE 			   	:=unix
RUST_MACOS_TARGET		=x86_64-apple-darwin
RUST_WINDOWS_TARGET		=x86_64-pc-windows-gnu
RUST_LINUX_TARGET		=x86_64-unknown-linux-gnu
RUST_OPERATING_TARGET  	:=$(RUST_MACOS_TARGET)

TEST_MPROC_CONFIG_FILE  :=test-data/.mproc.yml

export PKG_CONFIG_ALLOW_CROSS = 1

ifeq ($(OS),Windows_NT)
    OS_TYPE = windows
    RUST_OPERATING_TARGET = $(RUST_WINDOWS_TARGET)
    TEST_MPROC_CONFIG_FILE = test-data/.mproc.win.yml
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
    RUST_OPERATING_TARGET  = $(RUST_LINUX_TARGET)
    endif
endif

define execute_cargo
cargo ${1} --target=$(RUST_OPERATING_TARGET) ${2}
endef

define execute_cargo_no_target
cargo ${1}
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
	$(call execute_cargo,$@,--path .)

fmt:
	$(call execute_cargo_no_target,$@)

fmt_ci:
	$(call execute_cargo_no_target,fmt -- --check)

build:
	$(call execute_cargo,$@)

build-rel:
	$(call execute_cargo,build,--release)

run:
	$(call execute_cargo,$@,${TEST_MPROC_CONFIG_FILE})

test:
	$(call execute_cargo,$@)

ship-it: install fmt test build
	echo "Ready to ship."

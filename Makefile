WIN_X86_64_TARGET = x86_64-pc-windows-gnu

run:
	cargo run

run-windows:
	cargo run --target=$(WIN_X86_64_TARGET)

install:
	cargo install

install-windows:
	cargo install --path . --target=$(WIN_X86_64_TARGET)

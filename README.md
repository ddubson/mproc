# mproc

![Rust](https://github.com/ddubson/mproc/workflows/Rust/badge.svg)

## Building pre-requisites

- `glib`
    - MacOS: `brew install glib` then `export PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig"`
- Gtk3
    - MacOS: `brew install gtk+3`
    - Windows:
      - Install msys2
      - Within msys2, run:
        - `pacman -S mingw-w64-x86_64-gtk3`
        - `pacman -S mingw-w64-x86_64-toolchain`
      - Add the following vars to local system envs:
        - `GTK_LIB_DIR` -> usually `C:\msys64\mingw64\lib`
        - Update `Path` with `C:\msys64\mingw64\bin`
      - Add target `rustup target add x86_64-pc-windows-gnu`

## Launch locally

MacOS: `cargo run`

Windows: `cargo run --target=x86_64-pc-windows-gnu`


## Build binary

MacOS: `cargo build`

Windows: `cargo build --target=x86_64-pc-windows-gnu`

# Developing mproc

## Setup - Windows

### Install GTK+3 on Windows

1. [Install msys2](https://www.msys2.org/)
1. Using msys2, run the following: 
    ```powershell
    pacman -S mingw-w64-x86_64-gtk3
    pacman -S mingw-w64-x86_64-toolchain
    ```
1. Add the following vars to local system envs:
    ```env
    GTK_LIB_DIR=C:\msys64\mingw64\lib
    Path=$Path;C:\msys64\mingw64\bin
    ```
1. Add Rust target: `rustup target add x86_64-pc-windows-gnu`

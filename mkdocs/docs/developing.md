# Developing

## Setup - Windows

### Install GTK+3 on Windows

- [Install msys2](https://www.msys2.org/)
- Using msys2, run the following:
```powershell
pacman -S mingw-w64-x86_64-gtk3
pacman -S mingw-w64-x86_64-toolchain
```
- Add the following vars to local system envs:
```env
GTK_LIB_DIR=C:\msys64\mingw64\lib
Path=$Path;C:\msys64\mingw64\bin
```
- Add Rust target: 

```bash
rustup target add x86_64-pc-windows-gnu
```

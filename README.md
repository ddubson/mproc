# mproc

![Rust](https://github.com/ddubson/mproc/workflows/Rust/badge.svg)

mproc is a GUI-based GTK3 application written in Rust, and is aimed to help developers manage their processes while developing
a project locally on their dev systems.

The common use case is for a developer to use mproc to monitor the logging and health status of a 
front-end application and a back-end application while working on a project.

## Developing mproc

### Building pre-requisites

#### MacOS

|Requirement|How-to-Install|
|---|---|
|Homebrew|[Installation](https://brew.sh/)|
|Rust 1.44+|[Installation / Getting Started](https://www.rust-lang.org/learn/get-started)|
|rustup 1.21.1+|After installing Rust, run `rustup update`|
|rustfmt|After installing Rust, run `rustup component add rustfmt`|
|GNU make|Run `brew install make`|
|GTK+3 (Graphics Toolkit)|Run `brew install gtk+3`, `export PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig`|

#### Windows

|Requirement|How-to-Install|
|---|---|
|Rust 1.44+|[Installation / Getting Started](https://www.rust-lang.org/learn/get-started)|
|rustup 1.21.1+|After installing Rust, run `rustup update`|
|rustfmt|After installing Rust, run `rustup component add rustfmt`|
|GNU make|[Install msys2](https://www.msys2.org/)|
|GTK+3 (Graphics Toolkit)|[GTK+3 on Windows](docs/GTK3_Windows.md)|

### Launch locally

```bash
make run
```

### Building native binary

```bash
make install
```

### Footnotes and References

- [GTK3 and threads](https://coaxion.net/blog/2019/02/mpsc-channel-api-for-painless-usage-of-threads-with-gtk-in-rust/)

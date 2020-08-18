# mproc

![Rust](https://github.com/ddubson/mproc/workflows/Rust/badge.svg)

mproc is a GUI-based cross-platform GTK3 application written in Rust, and is aimed to help developers 
manage their processes while developing a project locally on their dev machines.

The common use case is for a developer to use mproc to monitor the logging and health status of a 
front-end application and a back-end application while working on a project.

## Usage

mproc requires a configuration file `.mproc.yml` to be located in the root of your project. 
The schema for mproc's yaml configuration files will be published
soon, but for now, take a look at `test-data` directory for example yaml configurations.

```
mproc [path-to-mproc-config]
```

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

#### Debian Linux

|Requirement|How-to-Install|
|---|---|
|Rust 1.44+|[Installation / Getting Started](https://www.rust-lang.org/learn/get-started)|
|rustup 1.21.1+|After installing Rust, run `rustup update`|
|rustfmt|After installing Rust, run `rustup component add rustfmt`|
|GNU make|Run `sudo apt install -y make`|
|GTK+3 (Graphics Toolkit)|Run `sudo apt install -y libgtk-3-dev`|

#### Windows

|Requirement|How-to-Install|
|---|---|
|Rust 1.44+|[Installation / Getting Started](https://www.rust-lang.org/learn/get-started)|
|rustup 1.21.1+|After installing Rust, run `rustup update`|
|rustfmt|After installing Rust, run `rustup component add rustfmt`|
|GNU make|[Install msys2](https://www.msys2.org/)|
|GTK+3 (Graphics Toolkit)|[GTK+3 on Windows](docs/GTK3_Windows.md)|

### Launch locally

Launches mproc with a test `.mproc.yml` configuration 
(or if on Windows, a test `.mproc.win.yml` configuration)

```bash
make run
```

### Building native binary

```bash
make install
```

`mproc` is now installed and available on your PATH.

### Footnotes and References

- [GTK3 and threads](https://coaxion.net/blog/2019/02/mpsc-channel-api-for-painless-usage-of-threads-with-gtk-in-rust/)
- [GTK3 callbacks/closures](https://gtk-rs.org/docs-src/tutorial/closures)
- [CSS + GTK3 examples](https://github.com/gtk-rs/examples/pull/227/files)

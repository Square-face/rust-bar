# Rust-Bar
A rust based wayland status bar for Linux.

This is originally a fork of [HybridBar](https://github.com/vars1ty/HybridBar.git) but instead of being easily
configurable, i have decided to make it specifically for my own use case. This means that it will be hard to configure
but comes with performance improvements as there is no longer a config file, and it is instead rebuilt when making
changes. Personal I think this is a good trade-off, and I plan to still use a widget based system. So configuration
is still going to be possible, but it will be done in the source code and require forking the repo.

## Building/Installation
1. `git clone https://github.com/Square-face/rust-bar`
2. `cd rust-bar`
3. `cargo build --release`
4. `cd target/release`
5. Done, the executable is called `hybrid-bar`.

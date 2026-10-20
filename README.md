# rusty
Pure rust desktop minimal webview application with WASM

## Descrition

This project compiles the WASM code with `cargo make` and embeds the wasm package and the js runtime afer running `cargo run`. The window loads that page from a `wry://` custom protocol, with devtools enabled.

## Linux build dependencies

Ubuntu 24.04, Debian 13, and later need WebKitGTK 4.1 (wry 0.5 required 4.0, which these releases do not ship):

```
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev pkg-config
```

Also install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) and [`cargo-make`](https://github.com/sagiegurari/cargo-make). Rust 1.85 or newer is required.

## Try it

```
cargo make
cargo run
```

<div align="center">

# 🦀 rusty

**A minimal desktop app in pure Rust: a native webview window running a Rust → WebAssembly frontend.**

[![CI](https://github.com/Markcial/rusty/actions/workflows/ci.yml/badge.svg)](https://github.com/Markcial/rusty/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/github/license/Markcial/rusty)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-2018-000000?logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-wasm--bindgen-654FF0?logo=webassembly&logoColor=white)
![wry](https://img.shields.io/badge/webview-wry%200.5-orange)
[![GitHub stars](https://img.shields.io/github/stars/Markcial/rusty?style=social)](https://github.com/Markcial/rusty/stargazers)

</div>

> 🧪 A small experiment from 2021: no JavaScript framework, no Electron, no Node. Just two Rust crates and one binary.

## ✨ What it does

- 🖼️ Opens a **native desktop window** using [wry](https://github.com/tauri-apps/wry) 0.5 (the webview library behind Tauri).
- 🦀 The UI is a **Rust crate compiled to WebAssembly** with `wasm-bindgen` + `web-sys`. It writes *"Hello from Rust!"* into the page.
- 📦 The HTML, the JS glue and the `.wasm` file are **embedded in the binary** with `include_bytes!`, so you ship one executable.
- 🌐 A tiny [rouille](https://github.com/tomaka/rouille) HTTP server serves those embedded files on a **random `127.0.0.1` port**, and the webview loads them from there.
- 🛠️ The webview opens with **devtools enabled** (`debug: true` turns on the inspector), so you can inspect it.

## 🧭 How it works

```text
┌──────────────── rusty (one native binary) ────────────────┐
│                                                           │
│  include_bytes!  ─►  index.html · package.js · .wasm      │
│                              │                            │
│                    rouille server on 127.0.0.1:<random>   │
│                              │                            │
│                 wry webview window "Rust WASM test"       │
│                              │                            │
│         wasm crate (wasm-bindgen) ─► <p>Hello from Rust!</p>
└───────────────────────────────────────────────────────────┘
```

| Path | What lives there |
| --- | --- |
| `src/main.rs` | 🖥️ Native app: embeds the assets, starts the server, opens the window |
| `build.rs` | 🛑 Stops the native compile with a clear error when `wasm/pkg` is missing |
| `wasm/src/lib.rs` | 🦀 Frontend compiled to WebAssembly |
| `wasm/index.html` | 📄 Page that loads `./pkg/package.js` |
| `Makefile.toml`, `wasm/Makefile.toml` | ⚙️ [cargo-make](https://github.com/sagiegurari/cargo-make) tasks (`wasm-pack build`) |

`cargo make` from the repository root walks the workspace. The `wasm` member runs first and executes `wasm-pack build --target web --out-name package --dev`, which writes `wasm/pkg/`. The root `rusty` package is next and runs `cargo build`, embedding those files. `wasm/pkg/` is generated and gitignored.

## 🔍 A taste of the code

The whole frontend, in Rust:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;
    Ok(())
}
```

And the native side serving it from memory:

```rust
static WASM: &'static [u8] = include_bytes!("../wasm/pkg/package_bg.wasm");

rouille::start_server(addr, move |request| {
    router!(request,
        (GET) ["/pkg/package_bg.wasm"] => {
            rouille::Response::from_data("application/wasm", WASM)
        },
        // ... index.html and package.js too
        _ => rouille::Response::empty_404()
    )
});
```

## 📋 Requirements

- 🦀 **Rust 1.83** with the `wasm32-unknown-unknown` target. This tree pins `wasm-bindgen` 0.2.71. Rust **1.86 and newer** reject that crate (`wasm_c_abi` is a hard error), so a current stable toolchain cannot build it. CI uses 1.83.0.
  ```bash
  rustup toolchain install 1.83.0
  rustup target add wasm32-unknown-unknown --toolchain 1.83.0
  ```
- 📦 [wasm-pack](https://rustwasm.github.io/wasm-pack/) **0.13.1** and [cargo-make](https://github.com/sagiegurari/cargo-make). CI downloads that wasm-pack release and runs the same `wasm-pack build --target web --out-name package --dev` command as `wasm/Makefile.toml`.
  ```bash
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
  cargo +1.83.0 install cargo-make
  ```
- 🐧 On Linux, the desktop binary needs **GTK 3** and **WebKitGTK 4.0** (`libwebkit2gtk-4.0-dev`, which pulls in `libsoup2.4`).

> ⚠️ **WebKitGTK 4.0 is not in current Debian or Ubuntu.** wry 0.5 links `webkit2gtk-4.0`. Ubuntu 22.04 has `libwebkit2gtk-4.0-dev`. Ubuntu 24.04 and Debian 13 ship WebKitGTK 4.1 (`libwebkit2gtk-4.1`) instead, and `cargo run` cannot link the window there. CI builds only the `wasm` crate, which does not need WebKitGTK. A newer wry would fix the link, but `wry::Application` no longer exists, so that upgrade is separate from this tree.

## 🚀 Try it

> ⚠️ Order matters. `cargo make` builds `wasm/pkg/` before it compiles the desktop binary. `cargo run` embeds whatever is in `wasm/pkg/` at compile time. If that directory is missing, the build stops in `build.rs` and tells you to run `cargo make`.

```bash
git clone https://github.com/Markcial/rusty
cd rusty

rustup toolchain install 1.83.0
rustup target add wasm32-unknown-unknown --toolchain 1.83.0

# 1️⃣ wasm-pack writes wasm/pkg/, then cargo compiles the native binary
cargo +1.83.0 make

# 2️⃣ Launch the desktop window
cargo +1.83.0 run
```

On a machine that already has WebKitGTK 4.0, the app prints the local address it is serving on (for example `127.0.0.1:41234`) and opens a window titled **"Rust WASM test"** showing *Hello from Rust!* 👋

## 📄 License

[MIT](LICENSE) © Marc Garcia Sastre

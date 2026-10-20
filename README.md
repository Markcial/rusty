<div align="center">

# 🦀 rusty

**A minimal desktop app in pure Rust: a native webview window running a Rust → WebAssembly frontend.**

[![CI](https://github.com/Markcial/rusty/actions/workflows/ci.yml/badge.svg)](https://github.com/Markcial/rusty/actions/workflows/ci.yml)
[![native](https://github.com/Markcial/rusty/actions/workflows/native.yml/badge.svg)](https://github.com/Markcial/rusty/actions/workflows/native.yml)
[![License: MIT](https://img.shields.io/github/license/Markcial/rusty)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-2021-000000?logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-wasm--bindgen-654FF0?logo=webassembly&logoColor=white)
![wry](https://img.shields.io/badge/webview-wry%200.57-orange)
[![GitHub stars](https://img.shields.io/github/stars/Markcial/rusty?style=social)](https://github.com/Markcial/rusty/stargazers)

</div>

> 🧪 A small experiment: no JavaScript framework, no Electron, no Node. Just two Rust crates and one binary.

## ✨ What it does

- 🖼️ Opens a **native desktop window** using [wry](https://github.com/tauri-apps/wry) 0.57 and [tao](https://github.com/tauri-apps/tao) 0.37 (WebKitGTK 4.1 on Linux).
- 🦀 The UI is a **Rust crate compiled to WebAssembly** with `wasm-bindgen` + `web-sys`. It writes *"Hello from Rust!"* into the page.
- 📦 The HTML, the JS glue and the `.wasm` file are **embedded in the binary** with `include_bytes!`, so you ship one executable.
- 🌐 Those files are served to the webview with a **`wry://localhost` custom protocol**. There is no local TCP server and no port to race.
- 🛠️ The webview is created with **devtools enabled**, so the inspector can be opened from the page.

## 🧭 How it works

```text
┌──────────────── rusty (one native binary) ────────────────┐
│                                                           │
│  include_bytes!  ─►  index.html · package.js · .wasm      │
│                              │                            │
│              wry custom protocol  wry://localhost/        │
│                              │                            │
│                 wry webview window "Rust WASM test"       │
│                              │                            │
│         wasm crate (wasm-bindgen) ─► <p>Hello from Rust!</p>
└───────────────────────────────────────────────────────────┘
```

| Path | What lives there |
| --- | --- |
| `src/main.rs` | 🖥️ Native app: embeds the assets, registers `wry://`, opens the window |
| `build.rs` | 🛑 Stops the native compile with a clear error when `wasm/pkg` is missing |
| `wasm/src/lib.rs` | 🦀 Frontend compiled to WebAssembly |
| `wasm/index.html` | 📄 Page that loads `./pkg/package.js` |
| `Makefile.toml`, `wasm/Makefile.toml` | ⚙️ [cargo-make](https://github.com/sagiegurari/cargo-make) tasks (`wasm-pack build`) |

`cargo make` from the repository root walks the workspace. The `wasm` member runs `wasm-pack build --target web --out-name package --dev`, which writes `wasm/pkg/`. The root package then runs `cargo build`, embedding those files. `wasm/pkg/` is generated and gitignored.

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

The window loads that page from the custom protocol (`wry://localhost/`), including `./pkg/package.js` and `./pkg/package_bg.wasm`.

## 📋 Requirements

- 🦀 **Rust 1.85** or newer (wry 0.57’s minimum), plus the `wasm32-unknown-unknown` target.
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- 📦 [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and [cargo-make](https://github.com/sagiegurari/cargo-make).
  ```bash
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
  cargo install cargo-make
  ```
- 🐧 On Linux (Ubuntu 24.04, Debian 13, and later), the desktop binary needs **GTK 3** and **WebKitGTK 4.1**:
  ```bash
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev pkg-config
  ```

## 🚀 Try it

> ⚠️ Order matters. `cargo make` builds `wasm/pkg/` before it compiles the desktop binary. `cargo run` embeds whatever is in `wasm/pkg/` at compile time. If that directory is missing, the build stops in `build.rs` and tells you to run `cargo make`.

```bash
git clone https://github.com/Markcial/rusty
cd rusty

rustup target add wasm32-unknown-unknown

# 1️⃣ wasm-pack writes wasm/pkg/, then cargo compiles the native binary
cargo make

# 2️⃣ Launch the desktop window
cargo run
```

The app opens a window titled **"Rust WASM test"** showing *Hello from Rust!* 👋

## 📄 License

[MIT](LICENSE) © Marc Garcia Sastre

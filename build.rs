use std::env;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let required = ["wasm/pkg/package.js", "wasm/pkg/package_bg.wasm"];

    let mut missing = Vec::new();
    for relative in &required {
        let path = manifest_dir.join(relative);
        println!("cargo:rerun-if-changed={}", path.display());
        if !path.is_file() {
            missing.push(*relative);
        }
    }

    if missing.is_empty() {
        return;
    }

    eprintln!();
    eprintln!("error: wasm/pkg is missing, so the desktop binary cannot be compiled.");
    eprintln!();
    eprintln!("src/main.rs embeds these files at compile time with include_bytes!:");
    for relative in &missing {
        eprintln!("  - {relative}");
    }
    eprintln!();
    eprintln!("Build the WebAssembly package first (from the repository root):");
    eprintln!();
    eprintln!("    cargo make");
    eprintln!();
    eprintln!("That runs wasm-pack for the wasm crate and writes wasm/pkg/.");
    eprintln!("Then compile and launch the window:");
    eprintln!();
    eprintln!("    cargo run");
    eprintln!();
    exit(1);
}

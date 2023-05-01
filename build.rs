use std::{env, fs, path::PathBuf};

// Environment variables that have the same value in all crates built for TARGET
// can be supported here; environment variables that have different values in
// each crate, such as CARGO_MANIFEST_DIR, cannot be supported.
//
// See https://doc.rust-lang.org/nightly/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
// for available environment variables.
//
// NB: update README.md together when updating list.
const NAMES: &[&str] = &[
    "CARGO_CFG_SANITIZE",
    "CARGO_CFG_TARGET_ARCH",
    "CARGO_CFG_TARGET_ENDIAN",
    "CARGO_CFG_TARGET_ENV",
    "CARGO_CFG_TARGET_FAMILY",
    "CARGO_CFG_TARGET_OS",
    "CARGO_CFG_TARGET_POINTER_WIDTH",
    "CARGO_CFG_TARGET_VENDOR",
    "CARGO",
    "HOST",
    "RUSTC",
    "RUSTDOC",
    "TARGET",
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir: PathBuf = env::var_os("OUT_DIR").expect("OUT_DIR not set").into();
    let mut buf = String::new();
    for &name in NAMES {
        let var = env::var(name).unwrap_or_default();
        buf.push_str(&format!(
            "/// `{}`\npub const {}: &str = r\"{}\";\n",
            name,
            name.strip_prefix("CARGO_CFG_").unwrap_or(name),
            var
        ));
    }
    let path = &out_dir.join("build-context");
    fs::write(path, buf).unwrap_or_else(|e| panic!("failed to write {}: {}", path.display(), e));
}

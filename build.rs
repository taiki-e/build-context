// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{env, fmt::Write, fs, path::PathBuf};

// Environment variables that have the same value in all crates built for TARGET
// can be supported here; environment variables that can have different values
// in each crate, such as CARGO_MANIFEST_DIR and profiles (OPT_LEVEL/DEBUG),
// cannot be supported.
//
// See https://doc.rust-lang.org/nightly/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
// for available environment variables.
const NAMES: &[&str] = &[
    "CARGO_CFG_PANIC",
    "CARGO_CFG_SANITIZE",
    "CARGO_CFG_TARGET_ABI",
    "CARGO_CFG_TARGET_ARCH",
    "CARGO_CFG_TARGET_ENDIAN",
    "CARGO_CFG_TARGET_ENV",
    "CARGO_CFG_TARGET_FAMILY",
    "CARGO_CFG_TARGET_FEATURE",
    "CARGO_CFG_TARGET_HAS_ATOMIC",
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
    println!(r#"cargo:rustc-check-cfg=cfg(host_os,values("windows"))"#);

    let out_dir: PathBuf = env::var_os("OUT_DIR").expect("OUT_DIR not set").into();
    let mut buf = String::new();
    for &name in NAMES {
        let var = env::var(name).unwrap_or_default();
        let _ = write!(
            buf,
            "/// `{}`\npub const {}: &str = r\"{}\";\n",
            name,
            strip_prefix(name, "CARGO_CFG_").unwrap_or(name),
            var
        );
    }
    let path = &out_dir.join("build-context");
    fs::write(path, buf).unwrap_or_else(|e| panic!("failed to write {}: {}", path.display(), e));

    let host = env::var("HOST").expect("HOST not set");
    if host.contains("-windows") {
        println!(r#"cargo:rustc-cfg=host_os="windows""#);
    }
}

// str::strip_prefix requires Rust 1.45
#[must_use]
fn strip_prefix<'a>(s: &'a str, pat: &str) -> Option<&'a str> {
    if s.starts_with(pat) { Some(&s[pat.len()..]) } else { None }
}

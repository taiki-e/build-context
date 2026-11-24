// SPDX-License-Identifier: Apache-2.0 OR MIT

#[path = "src/build/version.rs"]
mod version;

use std::{env, fmt::Write, fs, path::PathBuf, str};

use self::version::rustc_version;

// Environment variables that have the same value in all crates built for TARGET
// can be supported here; environment variables that can have different values
// in each crate, such as CARGO_MANIFEST_DIR and profiles (OPT_LEVEL/DEBUG),
// cannot be supported.
//
// See https://doc.rust-lang.org/nightly/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
// for available environment variables.
//
// CARGO_CFG_TARGET_ABI may be changed to "string (comma-separated)" in the future:
// https://github.com/rust-lang/rust/issues/80970#issuecomment-1877682716
const NAMES: &[&str] = &[
    "CARGO_CFG_PANIC",                // string
    "CARGO_CFG_SANITIZE",             // string (comma-separated)
    "CARGO_CFG_TARGET_ABI",           // string
    "CARGO_CFG_TARGET_ARCH",          // string
    "CARGO_CFG_TARGET_ENDIAN",        // string
    "CARGO_CFG_TARGET_ENV",           // string
    "CARGO_CFG_TARGET_FAMILY",        // string (comma-separated)
    "CARGO_CFG_TARGET_FEATURE",       // string (comma-separated)
    "CARGO_CFG_TARGET_HAS_ATOMIC",    // string (comma-separated)
    "CARGO_CFG_TARGET_OS",            // string
    "CARGO_CFG_TARGET_POINTER_WIDTH", // string (int)
    "CARGO_CFG_TARGET_VENDOR",        // string
    "CARGO",                          // string (path)
    "HOST",                           // string
    "RUSTC",                          // string (path)
    "RUSTDOC",                        // string (path)
    "TARGET",                         // string
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

    let version = rustc_version().expect("failed to get rustc version");
    let _ = write!(
        buf,
        "\
        /// The rustc minor version of `$RUSTC` (major version is always 1).\n\
        pub const RUSTC_MINOR_VERSION: u32 = {};\n\
        /// `true` if `$RUSTC` is \"nightly\" or \"dev\" channel.\n\
        pub const RUSTC_NIGHTLY: bool = {};\n\
        /// The LLVM major version of `$RUSTC`.\n\
        ///\n\
        /// This is known to be `None` in rustc 1.49-1.50 (and 1.13 or older) that don't print LLVM version.\n\
        pub const LLVM_MAJOR_VERSION: Option<u32> = \
        ",
        version.minor, version.nightly,
    );
    let _ = match version.llvm {
        Some(llvm) => writeln!(buf, "Some({});", llvm),
        None => writeln!(buf, "None;"),
    };

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

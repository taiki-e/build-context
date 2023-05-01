use std::{env::consts as env, path::Path};

use build_context::*;

#[test]
fn basic() {
    assert_eq!(TARGET_ARCH, env::ARCH);
    assert_eq!(TARGET_OS, env::OS);
    assert_eq!(TARGET_FAMILY, env::FAMILY);
    if is_path(CARGO) {
        assert!(Path::new(CARGO).exists());
    }
    if is_path(RUSTC) {
        assert!(Path::new(RUSTC).exists());
    }
    if is_path(RUSTDOC) {
        assert!(Path::new(RUSTDOC).exists());
    }
}

fn is_path(s: &str) -> bool {
    s.contains('/') || s.contains('\\')
}

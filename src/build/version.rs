// SPDX-License-Identifier: Apache-2.0 OR MIT

// rustc version parsing used in the build script.
//
// Adapted from https://github.com/taiki-e/atomic-maybe-uninit

use std::{env, ffi::OsString, iter, process::Command, str};

pub(crate) fn rustc_version() -> Option<Version> {
    let rustc = env::var_os("RUSTC")?;
    let rustc_wrapper = if env::var_os("CARGO_ENCODED_RUSTFLAGS").is_some() {
        env::var_os("RUSTC_WRAPPER").filter(|v| !v.is_empty())
    } else {
        // Cargo sets environment variables for wrappers correctly only since Cargo 1.55 (https://github.com/rust-lang/cargo/pull/9601).
        None
    };
    // Do not apply RUSTC_WORKSPACE_WRAPPER: https://github.com/cuviper/autocfg/issues/58#issuecomment-2067625980
    let mut rustc = rustc_wrapper.into_iter().chain(iter::once(rustc));
    let mut cmd = Command::new(rustc.next().unwrap());
    cmd.args(rustc);
    // Use verbose version output because the packagers add extra strings to the normal version output.
    // Do not use long flags (--version --verbose) because clippy-deriver doesn't handle them properly.
    // -vV is also matched with that cargo internally uses: https://github.com/rust-lang/cargo/blob/0.80.0/src/cargo/util/rustc.rs#L65
    let output = cmd.arg("-vV").output().ok()?;
    let verbose_version = str::from_utf8(&output.stdout).ok()?;
    Version::parse(verbose_version, env::var_os("RUSTC_BOOTSTRAP").as_ref())
}

pub(crate) struct Version {
    pub(crate) minor: u32,
    pub(crate) nightly: bool,
    pub(crate) llvm: Option<u32>,
}

impl Version {
    fn parse(verbose_version: &str, rustc_bootstrap: Option<&OsString>) -> Option<Self> {
        let mut release = verbose_version
            .lines()
            .find(|line| line.starts_with("release: "))
            .map(|line| &line["release: ".len()..])?
            .splitn(2, '-');
        let version = release.next().unwrap();
        let channel = release.next().unwrap_or_default();
        let mut digits = version.splitn(3, '.');
        let major = digits.next()?;
        if major != "1" {
            return None;
        }
        let minor = digits.next()?.parse::<u32>().ok()?;
        let _patch = digits.next().unwrap_or("0").parse::<u32>().ok()?;
        let nightly = channel == "nightly" || channel == "dev";
        let nightly = match rustc_bootstrap {
            // When -1 is passed rustc works like stable, e.g., cfg(target_feature = "unstable_target_feature") will never be set. https://github.com/rust-lang/rust/pull/132993
            Some(v) if v == "-1" => false,
            // When 1 or this crate name is passed stable rustc works like nightly, but we ignore it
            // since there is no guarantee that this will continue to work in the future.
            // https://doc.rust-lang.org/nightly/unstable-book/compiler-environment-variables/RUSTC_BOOTSTRAP.html
            _ => nightly,
        };

        // Note that rustc 1.49-1.50 (and 1.13 or older) don't print LLVM version.
        let llvm_major = (|| {
            let version = verbose_version
                .lines()
                .find(|line| line.starts_with("LLVM version: "))
                .map(|line| &line["LLVM version: ".len()..])?;
            let mut digits = version.splitn(3, '.');
            let major = digits.next()?.parse::<u32>().ok()?;
            let _minor = digits.next()?.parse::<u32>().ok()?;
            let _patch = digits.next().unwrap_or("0").parse::<u32>().ok()?;
            Some(major)
        })();

        Some(Self { minor, nightly, llvm: llvm_major })
    }
}
